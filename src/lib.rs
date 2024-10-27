extern crate proc_macro;

use proc_macro::{
    token_stream::IntoIter, Delimiter, TokenStream, TokenTree, TokenTree::Group, TokenTree::Ident,
    TokenTree::Punct,
};

/// Bindgen doesn't capitalize constants for some reason.
#[allow(non_upper_case_globals)]
mod bindings;

/// Computes the hash of the structure name.
///
/// * `name` - Name of the structure as a String.
macro_rules! name_hash {
    ($name:expr) => {
        $name.into_bytes().iter().fold(0, |acc, &x| {
            (acc + (u32::from(x) << 4) + (u32::from(x) >> 4)).wrapping_mul(11)
        })
    };
}

/// Rotates a number left by a number.
///
/// * `x` - Number to rotate.
/// * `k` - Number to rotate by.
macro_rules! rot {
    ($x:expr, $k:expr) => {
        $x.rotate_left($k)
    };
}

/// Convenience struct for holding random context.
struct Ranctx {
    a: u64,
    b: u64,
    c: u64,
    d: u64,
}

/// We implement `new` for Ranctx as a convenience to create it from a &[u64, 4]
impl Ranctx {
    /// Creates the ranctx from a [u64; 4]
    ///
    /// * `seed` - Seed to create context from.
    pub fn new(seed: &[u64; 4]) -> Self {
        Self {
            a: seed[0],
            b: seed[1],
            c: seed[2],
            d: seed[3],
        }
    }
}

/// Does the random value maths as specified in the gcc plugin.
///
/// * `x` - Random context to use for random generation.
fn ranval(x: &mut Ranctx) -> u64 {
    let e: u64 = x.a.wrapping_sub(rot!(x.b, 7));

    x.a = x.b ^ rot!(x.c, 13);
    x.b = x.c.wrapping_add(rot!(x.d, 37));
    x.c = x.d.wrapping_add(e);
    x.d = e.wrapping_add(x.a);

    x.d
}

/// Creates the random context from a seed.
///
/// * `seed` - Seed to use to generate the random context.
fn raninit(seed: &[u64; 4]) -> Ranctx {
    let mut ranctx: Ranctx = Ranctx::new(seed);

    for _ in 0..30 {
        ranval(&mut ranctx);
    }

    ranctx
}

/// Shuffles the structure members based on the random context.
///
/// * `struct_members` - The structure members to randomize.
/// * `prng_state`     - The random context to use to randomize the structure members.
fn full_shuffle(struct_members: &mut [Vec<TokenTree>], prng_state: &mut Ranctx) {
    for i in (1..(struct_members.len() as u64)).rev() {
        // Generate a random index to swap with.
        let randnum: u64 = ranval(prng_state) % (i + 1);

        let tmp: Vec<TokenTree> = struct_members[i as usize].clone();
        struct_members[i as usize] = struct_members[randnum as usize].clone();
        struct_members[randnum as usize] = tmp;
    }
}

/// Generates the shuffle seed from a precomputed string.
///
/// * `randstruct_seed` - Hex string to use as the seed.
///
/// Note:
///     The bound ffi `randstruct_seed` is passed as an argument primarily for
///     the purpose of being able to unit test this function.
fn generate_shuffle_seed(randstruct_seed: &std::ffi::CStr) -> [u64; 4] {
    let seed = randstruct_seed
        .to_str()
        .expect("Failed to convert randstruct_seed to primitive str");

    assert!(seed.len() == 64);

    [
        u64::from_str_radix(&seed[0..16], 16).expect("Failed to parse randstruct seed as hex"),
        u64::from_str_radix(&seed[16..32], 16).expect("Failed to parse randstruct seed as hex"),
        u64::from_str_radix(&seed[32..48], 16).expect("Failed to parse randstruct seed as hex"),
        u64::from_str_radix(&seed[48..64], 16).expect("Failed to parse randstruct seed as hex"),
    ]
}

/// Does the magic structure shuffle.
///
/// * `struct_name`   - Name of the structure to shuffle.
/// * `struct_members` - The structure's members.
fn shuffle(struct_name: String, struct_members: &mut [Vec<TokenTree>]) {
    let name_hash: u32 = name_hash!(struct_name);
    let shuffle_seed: [u64; 4] = generate_shuffle_seed(bindings::randstruct_seed);

    let mut prng_state: Ranctx = raninit(&shuffle_seed.map(|x| x ^ u64::from(name_hash)));

    full_shuffle(struct_members, &mut prng_state);
}

/// Captures the structure members from the iterator.
///
/// * `struct_iterator` - Iterator to capture the structure members from.
fn capture_struct_members(struct_iterator: &mut IntoIter) -> Vec<Vec<TokenTree>> {
    let mut members: Vec<Vec<TokenTree>> = vec![Vec::<TokenTree>::new()];

    let members_group = match struct_iterator.next() {
        Some(Group(x)) => x.stream().into_iter(),
        _ => unimplemented!("Something is wrong wtih struct, I can't parse it."),
    };

    for token in members_group {
        members
            .last_mut()
            .expect("Failed to retrieve last member from members.")
            .push(token.clone());

        if let Punct(ref punct) = token {
            if punct.as_char() == ',' {
                members.push(Vec::<TokenTree>::new());
            }
        }
    }

    // Because of the simple nature of the above code, it always adds an empty
    // member at the end. Hence, we need to remove it.
    members.pop();

    members
}

/// Advances the iterator until it finds what it believes to be a struct.
///
/// * `struct_iterator` - Iterator to iterate through until a struct is found.
fn advance_to_struct_members(struct_iterator: &mut IntoIter) -> Vec<TokenTree> {
    let mut result: Vec<TokenTree> = Vec::<TokenTree>::new();

    for tree in struct_iterator.by_ref() {
        result.push(tree.clone());

        if let Ident(ref ident) = tree {
            if ident.to_string() == "struct" {
                break;
            }
        }
    }

    result
}

/// Processes and captures structure tokens before the structure members.
///
/// * `struct_iterator` - Iterator of structure to process and capture.
fn struct_pre_info(struct_iterator: &mut IntoIter) -> TokenStream {
    let mut result: TokenStream = TokenStream::default();
    result.extend(advance_to_struct_members(struct_iterator));
    result
}

/// Retrieves and returns the structure name Token.
///
/// * `struct_iterator` - Iterator to capture the struct name from.
fn get_struct_name(struct_iterator: &mut IntoIter) -> TokenTree {
    match struct_iterator.next() {
        Some(name) => name,
        _ => unimplemented!("Failed to retrieve structure name"),
    }
}

/// Builds the brace delimited structure members group.
///
/// * `members` - Structure members to frame into a `TokenTree`.
///
/// Note:
///     This returns a vector, with only one element, of `TokenTree`.
///     `extend_one` is a nightly feature(24/10/2024), for now we
///     have to use `extend` which only accepts an iterable.
fn build_members(members: Vec<Vec<TokenTree>>) -> Vec<TokenTree> {
    vec![Group(proc_macro::Group::new(
        Delimiter::Brace,
        // We fold all the TokenTrees into a TokenStream.
        members
            .into_iter()
            .fold(TokenStream::new(), |mut members, member| {
                members.extend(member);
                members
            }),
    ))]
}

/// `proc_macro` for doing the structure layout randomization. Based on the seed value
/// that is passed to it through the `SEED_HEADER_FILE` environment variable.
///
/// The header file should be the same one used by the corresponding C code.
///
/// * `attr` - unused.
/// * `item` - Token stream to parse.
#[proc_macro_attribute]
pub fn randomize_layout(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut struct_iterator = item.into_iter();

    // Capture all tokens before the actual struct members, this includes other
    // attributes etc.
    let mut result: TokenStream = struct_pre_info(&mut struct_iterator);

    // Retrieve and store the name of the structure.
    //
    // This has to be called before `capture_struct_members` to make sure that
    // there is not an attempt to parse the name as a member.
    let name_token = get_struct_name(&mut struct_iterator);
    result.extend(vec![name_token.clone()]);

    // Capture all members of the structure.
    let mut members = capture_struct_members(&mut struct_iterator);

    shuffle(name_token.to_string(), &mut members);

    result.extend(build_members(members));

    result
}

#[cfg(test)]
mod name_hash_tests {

    #[test]
    fn test_name_hash() {
        assert_eq!(name_hash!(String::from("TestStruct")), 0x6445c434);
    }

    #[test]
    fn test_name_hash_empty() {
        assert_eq!(name_hash!(String::from("")), 0);
    }

    #[test]
    fn test_name_hash_zeros() {
        assert_eq!(name_hash!(String::from("000000")), 0x598db884);
    }

    #[test]
    fn test_name_hash_long() {
        assert_eq!(
            name_hash!(String::from(
                "qwertyuiopasdfghjklzxcvbnmabcdefghijklmnopqrstuvwxyz"
            )),
            0x374cd086
        );
    }
}

#[cfg(test)]
mod rot_tests {

    #[test]
    fn test_rot_0() {
        assert_eq!(rot!(0xdeadbeef_u64, 0), 0xdeadbeef);
    }

    #[test]
    fn test_rot_1() {
        assert_eq!(rot!(0xdeadbeef_u64, 1), 0x1bd5b7dde_u64);
    }

    #[test]
    fn test_rot_32() {
        assert_eq!(rot!(0xdeadbeef_u64, 32), 0xdeadbeef00000000_u64);
    }

    #[test]
    fn test_rot_48() {
        assert_eq!(rot!(0xdeadbeef_u64, 48), 0xbeef00000000dead_u64);
    }
}

#[cfg(test)]
mod ranctx_tests {

    use super::*;

    #[test]
    fn test_ranctx_creation_zeros() {
        let seed: [u64; 4] = [0_u64, 0_u64, 0_u64, 0_u64];
        let ctx: Ranctx = Ranctx::new(&seed);

        assert_eq!(ctx.a, 0_u64);
        assert_eq!(ctx.b, 0_u64);
        assert_eq!(ctx.c, 0_u64);
        assert_eq!(ctx.d, 0_u64);
    }

    #[test]
    fn test_ranctx_creation_average() {
        let seed: [u64; 4] = [0xdead_u64, 0xbeef_u64, 0xcafe_u64, 0xbabe_u64];
        let ctx: Ranctx = Ranctx::new(&seed);

        assert_eq!(ctx.a, 0xdead_u64);
        assert_eq!(ctx.b, 0xbeef_u64);
        assert_eq!(ctx.c, 0xcafe_u64);
        assert_eq!(ctx.d, 0xbabe_u64);
    }

    #[test]
    fn test_ranctx_creation_all_ones() {
        let seed: [u64; 4] = [
            0xffffffffffffffff_u64,
            0xffffffffffffffff_u64,
            0xffffffffffffffff_u64,
            0xffffffffffffffff_u64,
        ];
        let ctx: Ranctx = Ranctx::new(&seed);

        assert_eq!(ctx.a, 0xffffffffffffffff_u64);
        assert_eq!(ctx.b, 0xffffffffffffffff_u64);
        assert_eq!(ctx.c, 0xffffffffffffffff_u64);
        assert_eq!(ctx.d, 0xffffffffffffffff_u64);
    }
}

#[cfg(test)]
mod ranval_tests {

    use super::*;

    #[test]
    fn test_ranval_zeroes() {
        let mut ctx: Ranctx = Ranctx::new(&[0_u64, 0_u64, 0_u64, 0_u64]);

        assert_eq!(ranval(&mut ctx), 0);
    }

    #[test]
    fn test_ranval_ones() {
        let mut ctx: Ranctx = Ranctx::new(&[1_u64, 1_u64, 1_u64, 1_u64]);

        assert_eq!(ranval(&mut ctx), 0x1f82);
    }

    #[test]
    fn test_ranval_all_ones() {
        let mut ctx: Ranctx = Ranctx::new(&[
            0xffffffffffffffff_u64,
            0xffffffffffffffff_u64,
            0xffffffffffffffff_u64,
            0xffffffffffffffff_u64,
        ]);

        assert_eq!(ranval(&mut ctx), 0);
    }

    #[test]
    fn test_ranval_normal() {
        let mut ctx: Ranctx = Ranctx::new(&[
            0xdeadbeef_u64,
            0xcafebabe_u64,
            0xbeefcafe_u64,
            0xcafedead_u64,
        ]);

        assert_eq!(ranval(&mut ctx), 0x177892f1daad);
    }
}

#[cfg(test)]
mod raninit_tests {

    use super::*;

    #[test]
    fn test_raninit_zeroes() {
        let ctx: Ranctx = raninit(&[0_u64, 0_u64, 0_u64, 0_u64]);

        assert_eq!(ctx.a, 0);
        assert_eq!(ctx.b, 0);
        assert_eq!(ctx.c, 0);
        assert_eq!(ctx.d, 0);
    }

    #[test]
    fn test_raninit_ones() {
        let ctx: Ranctx = raninit(&[1_u64, 1_u64, 1_u64, 1_u64]);

        assert_eq!(ctx.a, 0x8acaa442b419eb85_u64);
        assert_eq!(ctx.b, 0x5ec59a9fa369da3f_u64);
        assert_eq!(ctx.c, 0xa84c930729b5df74_u64);
        assert_eq!(ctx.d, 0x8782d791cf581838_u64);
    }

    #[test]
    fn test_raninit_all_ones() {
        let ctx: Ranctx = raninit(&[
            0xffffffffffffffff_u64,
            0xffffffffffffffff_u64,
            0xffffffffffffffff_u64,
            0xffffffffffffffff_u64,
        ]);

        assert_eq!(ctx.a, 0xaad00fe963fab0c6_u64);
        assert_eq!(ctx.b, 0x9debd8bea64bb0b_u64);
        assert_eq!(ctx.c, 0xd44ba6c496ef8bc0_u64);
        assert_eq!(ctx.d, 0x391579b0adf3a7f3_u64);
    }

    #[test]
    fn test_ranval_normal() {
        let ctx: Ranctx = raninit(&[
            0xdeadbeef_u64,
            0xcafebabe_u64,
            0xbeefcafe_u64,
            0xcafedead_u64,
        ]);

        assert_eq!(ctx.a, 0x23a75d9f1bfce48a_u64);
        assert_eq!(ctx.b, 0x4422a846c906a7d_u64);
        assert_eq!(ctx.c, 0xf992ac8241f61d4a_u64);
        assert_eq!(ctx.d, 0x8bf245c4ee74da08_u64);
    }
}

#[cfg(test)]
mod generate_shuffle_seed_tests {

    use super::*;

    #[test]
    fn test_generate_shuffle_seed_zero() {
        let test_slice: [u64; 4] = generate_shuffle_seed(
            c"0000000000000000000000000000000000000000000000000000000000000000",
        );

        assert_eq!(test_slice[0], 0);
        assert_eq!(test_slice[1], 0);
        assert_eq!(test_slice[2], 0);
        assert_eq!(test_slice[3], 0);
    }

    #[test]
    fn test_generate_shuffle_seed() {
        let test_slice: [u64; 4] = generate_shuffle_seed(
            c"e2f28820694c414d09646092a3bf9dec6cba6b389be38112ef808d56300f3d89",
        );

        assert_eq!(test_slice[0], 0xe2f28820694c414d);
        assert_eq!(test_slice[1], 0x09646092a3bf9dec);
        assert_eq!(test_slice[2], 0x6cba6b389be38112);
        assert_eq!(test_slice[3], 0xef808d56300f3d89);
    }

    #[test]
    fn test_generate_shuffle_seed_all_ones() {
        let test_slice: [u64; 4] = generate_shuffle_seed(
            c"ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
        );

        assert_eq!(test_slice[0], 0xffffffffffffffff);
        assert_eq!(test_slice[1], 0xffffffffffffffff);
        assert_eq!(test_slice[2], 0xffffffffffffffff);
        assert_eq!(test_slice[3], 0xffffffffffffffff);
    }

    #[test]
    fn test_generate_shuffle_seed_boundary() {
        let test_slice: [u64; 4] = generate_shuffle_seed(
            c"fffffffffffffff0affffffffffffff1bffffffffffffff2cffffffffffffff3",
        );

        assert_eq!(test_slice[0], 0xfffffffffffffff0);
        assert_eq!(test_slice[1], 0xaffffffffffffff1);
        assert_eq!(test_slice[2], 0xbffffffffffffff2);
        assert_eq!(test_slice[3], 0xcffffffffffffff3);
    }
}
