# Randstruct

**This crate implements a subset of the features of the GCC randomize_layout plugin.**

---

```toml
[dependencies]
randstruct = "1.0"
```

## Background

The randomize_layout plugin is used by software to harden structs that contain sensitive/privileged data. One notable use case is a hardened Linux Kernel which uses it to remove predictable offsets of sensitive data in it's structures.

Consider the following as a trivial example:
```c
struct TestStruct {
    const char    *elem0;
    const char    *elem1;
    unsigned int   elem3;
    unsigned short elem4;
} __attribute__((randomize_layout));
```

Which may produce the following:
```c
struct TestStruct {
    const char    *elem1;
    unsigned short elem4;
    const char    *elem0;
    unsigned int   elem3;
} __attribute__((randomize_layout));
```

As is evident in the above example, the offsets of all the members have been changed.

### The Problem
Rust uses `#[repr(C)]` to force the underlying C layout of a particular structure. In the above example, a Rust equivalent structure would be:
```rust
#[repr(C)]
struct TestStruct {
    elem0: *const core::ffi::c_char,
    elem1: *const core::ffi::c_char,
    elem2: core::ffi::c_uint,
    elem3: core::ffi::c_ushort,
}
```
These Rust structures are typically either programmatically generated through a tool like bindgen or manually created by hand.

The generation of these structures is source-based; however, the GCC randomize_layout attribute will randomize at build time. Meaning, the source-level representation of the structure is not an accurate depiction of the structure once compiled.

This presents an issue in FFI when you want to process a C structure or pass a Rust structure through C code.

### The Solution
To replicate the exact structure member shuffling that randomize_layout achieves, this crate reimplements the logic to operate on Rust proc-macro TokenStreams instead of the GCC AST values.

## How To Use
Continuing the example above, we have a C structure that uses the randomize_layout plugin.
```c
struct TestStruct {
    const char    *elem0;
    const char    *elem1;
    unsigned int   elem3;
    unsigned short elem4;
} __attribute__((randomize_layout));
```

The equivalent Rust structure (with this crate) looks like the following:
```rust
#[repr(C)]
#[randomize_layout]
struct TestStruct {
    elem0: *const core::ffi::c_char,
    elem1: *const core::ffi::c_char,
    elem2: core::ffi::c_uint,
    elem3: core::ffi::c_ushort,
}
```

That is it. That is all that is required. The C and Rust can be compiled as separate compilation units, and they will be able to interoperate safely.

## Bulding With This Crate
You need to tell the crate where the header file is that contains the randstruct seed through the `SEED_HEADER_FILE` environment variable. For example, the below will suffice:
```bash
SEED_HEADER_FILE=`pwd`/seed.h cargo run
```
Where seed.h should contain a string called `randstruct_seed`, this header should be the same header as the one used by the GCC plugin. An exemplar header file is shown below:
```c
const char *randstruct_seed = "123456789abcdef123456789abcdef123456789abcdef123456789abcdef1234";
```