use core::mem::offset_of;
use std::fs::File;
use std::env;

use randstruct::randomize_layout;

#[repr(C)]
#[randomize_layout]
struct TestStruct_1 {
    elem1: *const core::ffi::c_char,
}
#[repr(C)]
#[randomize_layout]
struct TestStruct_2 {
    elem1: *const core::ffi::c_char,
    elem2: *const core::ffi::c_char,
}
#[repr(C)]
#[randomize_layout]
struct TestStruct_3 {
    elem1: *const core::ffi::c_char,
    elem2: *const core::ffi::c_char,
    elem3: *const core::ffi::c_char,
}
#[repr(C)]
#[randomize_layout]
struct TestStruct_4 {
    elem1: *const core::ffi::c_char,
    elem2: *const core::ffi::c_char,
    elem3: *const core::ffi::c_char,
    elem4: *const core::ffi::c_char,
}
#[repr(C)]
#[randomize_layout]
struct TestStruct_5 {
    elem1: *const core::ffi::c_char,
    elem2: *const core::ffi::c_char,
    elem3: *const core::ffi::c_char,
    elem4: *const core::ffi::c_char,
    elem5: *const core::ffi::c_char,
}
#[repr(C)]
#[randomize_layout]
struct TestStruct_6 {
    elem1: *const core::ffi::c_char,
    elem2: *const core::ffi::c_char,
    elem3: *const core::ffi::c_char,
    elem4: *const core::ffi::c_char,
    elem5: *const core::ffi::c_char,
    elem6: *const core::ffi::c_char,
}
#[repr(C)]
#[randomize_layout]
struct TestStruct_7 {
    elem1: *const core::ffi::c_char,
    elem2: *const core::ffi::c_char,
    elem3: *const core::ffi::c_char,
    elem4: *const core::ffi::c_char,
    elem5: *const core::ffi::c_char,
    elem6: *const core::ffi::c_char,
    elem7: *const core::ffi::c_char,
}
#[repr(C)]
#[randomize_layout]
struct TestStruct_8 {
    elem1: *const core::ffi::c_char,
    elem2: *const core::ffi::c_char,
    elem3: *const core::ffi::c_char,
    elem4: *const core::ffi::c_char,
    elem5: *const core::ffi::c_char,
    elem6: *const core::ffi::c_char,
    elem7: *const core::ffi::c_char,
    elem8: *const core::ffi::c_char,
}
#[repr(C)]
#[randomize_layout]
struct TestStruct_9 {
    elem1: *const core::ffi::c_char,
    elem2: *const core::ffi::c_char,
    elem3: *const core::ffi::c_char,
    elem4: *const core::ffi::c_char,
    elem5: *const core::ffi::c_char,
    elem6: *const core::ffi::c_char,
    elem7: *const core::ffi::c_char,
    elem8: *const core::ffi::c_char,
    elem9: *const core::ffi::c_char,
}
#[repr(C)]
#[randomize_layout]
struct TestStruct_10 {
    elem1: *const core::ffi::c_char,
    elem2: *const core::ffi::c_char,
    elem3: *const core::ffi::c_char,
    elem4: *const core::ffi::c_char,
    elem5: *const core::ffi::c_char,
    elem6: *const core::ffi::c_char,
    elem7: *const core::ffi::c_char,
    elem8: *const core::ffi::c_char,
    elem9: *const core::ffi::c_char,
    elem10: *const core::ffi::c_char,
}
#[repr(C)]
#[randomize_layout]
struct TestStruct_11 {
    elem1: *const core::ffi::c_char,
    elem2: *const core::ffi::c_char,
    elem3: *const core::ffi::c_char,
    elem4: *const core::ffi::c_char,
    elem5: *const core::ffi::c_char,
    elem6: *const core::ffi::c_char,
    elem7: *const core::ffi::c_char,
    elem8: *const core::ffi::c_char,
    elem9: *const core::ffi::c_char,
    elem10: *const core::ffi::c_char,
    elem11: *const core::ffi::c_char,
}
#[repr(C)]
#[randomize_layout]
struct TestStruct_12 {
    elem1: *const core::ffi::c_char,
    elem2: *const core::ffi::c_char,
    elem3: *const core::ffi::c_char,
    elem4: *const core::ffi::c_char,
    elem5: *const core::ffi::c_char,
    elem6: *const core::ffi::c_char,
    elem7: *const core::ffi::c_char,
    elem8: *const core::ffi::c_char,
    elem9: *const core::ffi::c_char,
    elem10: *const core::ffi::c_char,
    elem11: *const core::ffi::c_char,
    elem12: *const core::ffi::c_char,
}
#[repr(C)]
#[randomize_layout]
struct TestStruct_13 {
    elem1: *const core::ffi::c_char,
    elem2: *const core::ffi::c_char,
    elem3: *const core::ffi::c_char,
    elem4: *const core::ffi::c_char,
    elem5: *const core::ffi::c_char,
    elem6: *const core::ffi::c_char,
    elem7: *const core::ffi::c_char,
    elem8: *const core::ffi::c_char,
    elem9: *const core::ffi::c_char,
    elem10: *const core::ffi::c_char,
    elem11: *const core::ffi::c_char,
    elem12: *const core::ffi::c_char,
    elem13: *const core::ffi::c_char,
}
#[repr(C)]
#[randomize_layout]
struct TestStruct_14 {
    elem1: *const core::ffi::c_char,
    elem2: *const core::ffi::c_char,
    elem3: *const core::ffi::c_char,
    elem4: *const core::ffi::c_char,
    elem5: *const core::ffi::c_char,
    elem6: *const core::ffi::c_char,
    elem7: *const core::ffi::c_char,
    elem8: *const core::ffi::c_char,
    elem9: *const core::ffi::c_char,
    elem10: *const core::ffi::c_char,
    elem11: *const core::ffi::c_char,
    elem12: *const core::ffi::c_char,
    elem13: *const core::ffi::c_char,
    elem14: *const core::ffi::c_char,
}
#[repr(C)]
#[randomize_layout]
struct TestStruct_15 {
    elem1: *const core::ffi::c_char,
    elem2: *const core::ffi::c_char,
    elem3: *const core::ffi::c_char,
    elem4: *const core::ffi::c_char,
    elem5: *const core::ffi::c_char,
    elem6: *const core::ffi::c_char,
    elem7: *const core::ffi::c_char,
    elem8: *const core::ffi::c_char,
    elem9: *const core::ffi::c_char,
    elem10: *const core::ffi::c_char,
    elem11: *const core::ffi::c_char,
    elem12: *const core::ffi::c_char,
    elem13: *const core::ffi::c_char,
    elem14: *const core::ffi::c_char,
    elem15: *const core::ffi::c_char,
}
#[repr(C)]
#[randomize_layout]
struct TestStruct_16 {
    elem1: *const core::ffi::c_char,
    elem2: *const core::ffi::c_char,
    elem3: *const core::ffi::c_char,
    elem4: *const core::ffi::c_char,
    elem5: *const core::ffi::c_char,
    elem6: *const core::ffi::c_char,
    elem7: *const core::ffi::c_char,
    elem8: *const core::ffi::c_char,
    elem9: *const core::ffi::c_char,
    elem10: *const core::ffi::c_char,
    elem11: *const core::ffi::c_char,
    elem12: *const core::ffi::c_char,
    elem13: *const core::ffi::c_char,
    elem14: *const core::ffi::c_char,
    elem15: *const core::ffi::c_char,
    elem16: *const core::ffi::c_char,
}
#[repr(C)]
#[randomize_layout]
struct TestStruct_17 {
    elem1: *const core::ffi::c_char,
    elem2: *const core::ffi::c_char,
    elem3: *const core::ffi::c_char,
    elem4: *const core::ffi::c_char,
    elem5: *const core::ffi::c_char,
    elem6: *const core::ffi::c_char,
    elem7: *const core::ffi::c_char,
    elem8: *const core::ffi::c_char,
    elem9: *const core::ffi::c_char,
    elem10: *const core::ffi::c_char,
    elem11: *const core::ffi::c_char,
    elem12: *const core::ffi::c_char,
    elem13: *const core::ffi::c_char,
    elem14: *const core::ffi::c_char,
    elem15: *const core::ffi::c_char,
    elem16: *const core::ffi::c_char,
    elem17: *const core::ffi::c_char,
}
#[repr(C)]
#[randomize_layout]
struct TestStruct_18 {
    elem1: *const core::ffi::c_char,
    elem2: *const core::ffi::c_char,
    elem3: *const core::ffi::c_char,
    elem4: *const core::ffi::c_char,
    elem5: *const core::ffi::c_char,
    elem6: *const core::ffi::c_char,
    elem7: *const core::ffi::c_char,
    elem8: *const core::ffi::c_char,
    elem9: *const core::ffi::c_char,
    elem10: *const core::ffi::c_char,
    elem11: *const core::ffi::c_char,
    elem12: *const core::ffi::c_char,
    elem13: *const core::ffi::c_char,
    elem14: *const core::ffi::c_char,
    elem15: *const core::ffi::c_char,
    elem16: *const core::ffi::c_char,
    elem17: *const core::ffi::c_char,
    elem18: *const core::ffi::c_char,
}
#[repr(C)]
#[randomize_layout]
struct TestStruct_19 {
    elem1: *const core::ffi::c_char,
    elem2: *const core::ffi::c_char,
    elem3: *const core::ffi::c_char,
    elem4: *const core::ffi::c_char,
    elem5: *const core::ffi::c_char,
    elem6: *const core::ffi::c_char,
    elem7: *const core::ffi::c_char,
    elem8: *const core::ffi::c_char,
    elem9: *const core::ffi::c_char,
    elem10: *const core::ffi::c_char,
    elem11: *const core::ffi::c_char,
    elem12: *const core::ffi::c_char,
    elem13: *const core::ffi::c_char,
    elem14: *const core::ffi::c_char,
    elem15: *const core::ffi::c_char,
    elem16: *const core::ffi::c_char,
    elem17: *const core::ffi::c_char,
    elem18: *const core::ffi::c_char,
    elem19: *const core::ffi::c_char,
}
#[repr(C)]
#[randomize_layout]
struct TestStruct_20 {
    elem1: *const core::ffi::c_char,
    elem2: *const core::ffi::c_char,
    elem3: *const core::ffi::c_char,
    elem4: *const core::ffi::c_char,
    elem5: *const core::ffi::c_char,
    elem6: *const core::ffi::c_char,
    elem7: *const core::ffi::c_char,
    elem8: *const core::ffi::c_char,
    elem9: *const core::ffi::c_char,
    elem10: *const core::ffi::c_char,
    elem11: *const core::ffi::c_char,
    elem12: *const core::ffi::c_char,
    elem13: *const core::ffi::c_char,
    elem14: *const core::ffi::c_char,
    elem15: *const core::ffi::c_char,
    elem16: *const core::ffi::c_char,
    elem17: *const core::ffi::c_char,
    elem18: *const core::ffi::c_char,
    elem19: *const core::ffi::c_char,
    elem20: *const core::ffi::c_char,
}
#[repr(C)]
#[randomize_layout]
struct TestStruct_21 {
    elem1: *const core::ffi::c_char,
    elem2: *const core::ffi::c_char,
    elem3: *const core::ffi::c_char,
    elem4: *const core::ffi::c_char,
    elem5: *const core::ffi::c_char,
    elem6: *const core::ffi::c_char,
    elem7: *const core::ffi::c_char,
    elem8: *const core::ffi::c_char,
    elem9: *const core::ffi::c_char,
    elem10: *const core::ffi::c_char,
    elem11: *const core::ffi::c_char,
    elem12: *const core::ffi::c_char,
    elem13: *const core::ffi::c_char,
    elem14: *const core::ffi::c_char,
    elem15: *const core::ffi::c_char,
    elem16: *const core::ffi::c_char,
    elem17: *const core::ffi::c_char,
    elem18: *const core::ffi::c_char,
    elem19: *const core::ffi::c_char,
    elem20: *const core::ffi::c_char,
    elem21: *const core::ffi::c_char,
}
#[repr(C)]
#[randomize_layout]
struct TestStruct_22 {
    elem1: *const core::ffi::c_char,
    elem2: *const core::ffi::c_char,
    elem3: *const core::ffi::c_char,
    elem4: *const core::ffi::c_char,
    elem5: *const core::ffi::c_char,
    elem6: *const core::ffi::c_char,
    elem7: *const core::ffi::c_char,
    elem8: *const core::ffi::c_char,
    elem9: *const core::ffi::c_char,
    elem10: *const core::ffi::c_char,
    elem11: *const core::ffi::c_char,
    elem12: *const core::ffi::c_char,
    elem13: *const core::ffi::c_char,
    elem14: *const core::ffi::c_char,
    elem15: *const core::ffi::c_char,
    elem16: *const core::ffi::c_char,
    elem17: *const core::ffi::c_char,
    elem18: *const core::ffi::c_char,
    elem19: *const core::ffi::c_char,
    elem20: *const core::ffi::c_char,
    elem21: *const core::ffi::c_char,
    elem22: *const core::ffi::c_char,
}
#[repr(C)]
#[randomize_layout]
struct TestStruct_23 {
    elem1: *const core::ffi::c_char,
    elem2: *const core::ffi::c_char,
    elem3: *const core::ffi::c_char,
    elem4: *const core::ffi::c_char,
    elem5: *const core::ffi::c_char,
    elem6: *const core::ffi::c_char,
    elem7: *const core::ffi::c_char,
    elem8: *const core::ffi::c_char,
    elem9: *const core::ffi::c_char,
    elem10: *const core::ffi::c_char,
    elem11: *const core::ffi::c_char,
    elem12: *const core::ffi::c_char,
    elem13: *const core::ffi::c_char,
    elem14: *const core::ffi::c_char,
    elem15: *const core::ffi::c_char,
    elem16: *const core::ffi::c_char,
    elem17: *const core::ffi::c_char,
    elem18: *const core::ffi::c_char,
    elem19: *const core::ffi::c_char,
    elem20: *const core::ffi::c_char,
    elem21: *const core::ffi::c_char,
    elem22: *const core::ffi::c_char,
    elem23: *const core::ffi::c_char,
}
#[repr(C)]
#[randomize_layout]
struct TestStruct_24 {
    elem1: *const core::ffi::c_char,
    elem2: *const core::ffi::c_char,
    elem3: *const core::ffi::c_char,
    elem4: *const core::ffi::c_char,
    elem5: *const core::ffi::c_char,
    elem6: *const core::ffi::c_char,
    elem7: *const core::ffi::c_char,
    elem8: *const core::ffi::c_char,
    elem9: *const core::ffi::c_char,
    elem10: *const core::ffi::c_char,
    elem11: *const core::ffi::c_char,
    elem12: *const core::ffi::c_char,
    elem13: *const core::ffi::c_char,
    elem14: *const core::ffi::c_char,
    elem15: *const core::ffi::c_char,
    elem16: *const core::ffi::c_char,
    elem17: *const core::ffi::c_char,
    elem18: *const core::ffi::c_char,
    elem19: *const core::ffi::c_char,
    elem20: *const core::ffi::c_char,
    elem21: *const core::ffi::c_char,
    elem22: *const core::ffi::c_char,
    elem23: *const core::ffi::c_char,
    elem24: *const core::ffi::c_char,
}
#[repr(C)]
#[randomize_layout]
struct TestStruct_25 {
    elem1: *const core::ffi::c_char,
    elem2: *const core::ffi::c_char,
    elem3: *const core::ffi::c_char,
    elem4: *const core::ffi::c_char,
    elem5: *const core::ffi::c_char,
    elem6: *const core::ffi::c_char,
    elem7: *const core::ffi::c_char,
    elem8: *const core::ffi::c_char,
    elem9: *const core::ffi::c_char,
    elem10: *const core::ffi::c_char,
    elem11: *const core::ffi::c_char,
    elem12: *const core::ffi::c_char,
    elem13: *const core::ffi::c_char,
    elem14: *const core::ffi::c_char,
    elem15: *const core::ffi::c_char,
    elem16: *const core::ffi::c_char,
    elem17: *const core::ffi::c_char,
    elem18: *const core::ffi::c_char,
    elem19: *const core::ffi::c_char,
    elem20: *const core::ffi::c_char,
    elem21: *const core::ffi::c_char,
    elem22: *const core::ffi::c_char,
    elem23: *const core::ffi::c_char,
    elem24: *const core::ffi::c_char,
    elem25: *const core::ffi::c_char,
}
#[repr(C)]
#[randomize_layout]
struct TestStruct_26 {
    elem1: *const core::ffi::c_char,
    elem2: *const core::ffi::c_char,
    elem3: *const core::ffi::c_char,
    elem4: *const core::ffi::c_char,
    elem5: *const core::ffi::c_char,
    elem6: *const core::ffi::c_char,
    elem7: *const core::ffi::c_char,
    elem8: *const core::ffi::c_char,
    elem9: *const core::ffi::c_char,
    elem10: *const core::ffi::c_char,
    elem11: *const core::ffi::c_char,
    elem12: *const core::ffi::c_char,
    elem13: *const core::ffi::c_char,
    elem14: *const core::ffi::c_char,
    elem15: *const core::ffi::c_char,
    elem16: *const core::ffi::c_char,
    elem17: *const core::ffi::c_char,
    elem18: *const core::ffi::c_char,
    elem19: *const core::ffi::c_char,
    elem20: *const core::ffi::c_char,
    elem21: *const core::ffi::c_char,
    elem22: *const core::ffi::c_char,
    elem23: *const core::ffi::c_char,
    elem24: *const core::ffi::c_char,
    elem25: *const core::ffi::c_char,
    elem26: *const core::ffi::c_char,
}
#[repr(C)]
#[randomize_layout]
struct TestStruct_27 {
    elem1: *const core::ffi::c_char,
    elem2: *const core::ffi::c_char,
    elem3: *const core::ffi::c_char,
    elem4: *const core::ffi::c_char,
    elem5: *const core::ffi::c_char,
    elem6: *const core::ffi::c_char,
    elem7: *const core::ffi::c_char,
    elem8: *const core::ffi::c_char,
    elem9: *const core::ffi::c_char,
    elem10: *const core::ffi::c_char,
    elem11: *const core::ffi::c_char,
    elem12: *const core::ffi::c_char,
    elem13: *const core::ffi::c_char,
    elem14: *const core::ffi::c_char,
    elem15: *const core::ffi::c_char,
    elem16: *const core::ffi::c_char,
    elem17: *const core::ffi::c_char,
    elem18: *const core::ffi::c_char,
    elem19: *const core::ffi::c_char,
    elem20: *const core::ffi::c_char,
    elem21: *const core::ffi::c_char,
    elem22: *const core::ffi::c_char,
    elem23: *const core::ffi::c_char,
    elem24: *const core::ffi::c_char,
    elem25: *const core::ffi::c_char,
    elem26: *const core::ffi::c_char,
    elem27: *const core::ffi::c_char,
}
#[repr(C)]
#[randomize_layout]
struct TestStruct_28 {
    elem1: *const core::ffi::c_char,
    elem2: *const core::ffi::c_char,
    elem3: *const core::ffi::c_char,
    elem4: *const core::ffi::c_char,
    elem5: *const core::ffi::c_char,
    elem6: *const core::ffi::c_char,
    elem7: *const core::ffi::c_char,
    elem8: *const core::ffi::c_char,
    elem9: *const core::ffi::c_char,
    elem10: *const core::ffi::c_char,
    elem11: *const core::ffi::c_char,
    elem12: *const core::ffi::c_char,
    elem13: *const core::ffi::c_char,
    elem14: *const core::ffi::c_char,
    elem15: *const core::ffi::c_char,
    elem16: *const core::ffi::c_char,
    elem17: *const core::ffi::c_char,
    elem18: *const core::ffi::c_char,
    elem19: *const core::ffi::c_char,
    elem20: *const core::ffi::c_char,
    elem21: *const core::ffi::c_char,
    elem22: *const core::ffi::c_char,
    elem23: *const core::ffi::c_char,
    elem24: *const core::ffi::c_char,
    elem25: *const core::ffi::c_char,
    elem26: *const core::ffi::c_char,
    elem27: *const core::ffi::c_char,
    elem28: *const core::ffi::c_char,
}
#[repr(C)]
#[randomize_layout]
struct TestStruct_29 {
    elem1: *const core::ffi::c_char,
    elem2: *const core::ffi::c_char,
    elem3: *const core::ffi::c_char,
    elem4: *const core::ffi::c_char,
    elem5: *const core::ffi::c_char,
    elem6: *const core::ffi::c_char,
    elem7: *const core::ffi::c_char,
    elem8: *const core::ffi::c_char,
    elem9: *const core::ffi::c_char,
    elem10: *const core::ffi::c_char,
    elem11: *const core::ffi::c_char,
    elem12: *const core::ffi::c_char,
    elem13: *const core::ffi::c_char,
    elem14: *const core::ffi::c_char,
    elem15: *const core::ffi::c_char,
    elem16: *const core::ffi::c_char,
    elem17: *const core::ffi::c_char,
    elem18: *const core::ffi::c_char,
    elem19: *const core::ffi::c_char,
    elem20: *const core::ffi::c_char,
    elem21: *const core::ffi::c_char,
    elem22: *const core::ffi::c_char,
    elem23: *const core::ffi::c_char,
    elem24: *const core::ffi::c_char,
    elem25: *const core::ffi::c_char,
    elem26: *const core::ffi::c_char,
    elem27: *const core::ffi::c_char,
    elem28: *const core::ffi::c_char,
    elem29: *const core::ffi::c_char,
}
#[repr(C)]
#[randomize_layout]
struct TestStruct_30 {
    elem1: *const core::ffi::c_char,
    elem2: *const core::ffi::c_char,
    elem3: *const core::ffi::c_char,
    elem4: *const core::ffi::c_char,
    elem5: *const core::ffi::c_char,
    elem6: *const core::ffi::c_char,
    elem7: *const core::ffi::c_char,
    elem8: *const core::ffi::c_char,
    elem9: *const core::ffi::c_char,
    elem10: *const core::ffi::c_char,
    elem11: *const core::ffi::c_char,
    elem12: *const core::ffi::c_char,
    elem13: *const core::ffi::c_char,
    elem14: *const core::ffi::c_char,
    elem15: *const core::ffi::c_char,
    elem16: *const core::ffi::c_char,
    elem17: *const core::ffi::c_char,
    elem18: *const core::ffi::c_char,
    elem19: *const core::ffi::c_char,
    elem20: *const core::ffi::c_char,
    elem21: *const core::ffi::c_char,
    elem22: *const core::ffi::c_char,
    elem23: *const core::ffi::c_char,
    elem24: *const core::ffi::c_char,
    elem25: *const core::ffi::c_char,
    elem26: *const core::ffi::c_char,
    elem27: *const core::ffi::c_char,
    elem28: *const core::ffi::c_char,
    elem29: *const core::ffi::c_char,
    elem30: *const core::ffi::c_char,
}
#[repr(C)]
#[randomize_layout]
struct TestStruct_31 {
    elem1: *const core::ffi::c_char,
    elem2: *const core::ffi::c_char,
    elem3: *const core::ffi::c_char,
    elem4: *const core::ffi::c_char,
    elem5: *const core::ffi::c_char,
    elem6: *const core::ffi::c_char,
    elem7: *const core::ffi::c_char,
    elem8: *const core::ffi::c_char,
    elem9: *const core::ffi::c_char,
    elem10: *const core::ffi::c_char,
    elem11: *const core::ffi::c_char,
    elem12: *const core::ffi::c_char,
    elem13: *const core::ffi::c_char,
    elem14: *const core::ffi::c_char,
    elem15: *const core::ffi::c_char,
    elem16: *const core::ffi::c_char,
    elem17: *const core::ffi::c_char,
    elem18: *const core::ffi::c_char,
    elem19: *const core::ffi::c_char,
    elem20: *const core::ffi::c_char,
    elem21: *const core::ffi::c_char,
    elem22: *const core::ffi::c_char,
    elem23: *const core::ffi::c_char,
    elem24: *const core::ffi::c_char,
    elem25: *const core::ffi::c_char,
    elem26: *const core::ffi::c_char,
    elem27: *const core::ffi::c_char,
    elem28: *const core::ffi::c_char,
    elem29: *const core::ffi::c_char,
    elem30: *const core::ffi::c_char,
    elem31: *const core::ffi::c_char,
}
#[repr(C)]
#[randomize_layout]
struct TestStruct_32 {
    elem1: *const core::ffi::c_char,
    elem2: *const core::ffi::c_char,
    elem3: *const core::ffi::c_char,
    elem4: *const core::ffi::c_char,
    elem5: *const core::ffi::c_char,
    elem6: *const core::ffi::c_char,
    elem7: *const core::ffi::c_char,
    elem8: *const core::ffi::c_char,
    elem9: *const core::ffi::c_char,
    elem10: *const core::ffi::c_char,
    elem11: *const core::ffi::c_char,
    elem12: *const core::ffi::c_char,
    elem13: *const core::ffi::c_char,
    elem14: *const core::ffi::c_char,
    elem15: *const core::ffi::c_char,
    elem16: *const core::ffi::c_char,
    elem17: *const core::ffi::c_char,
    elem18: *const core::ffi::c_char,
    elem19: *const core::ffi::c_char,
    elem20: *const core::ffi::c_char,
    elem21: *const core::ffi::c_char,
    elem22: *const core::ffi::c_char,
    elem23: *const core::ffi::c_char,
    elem24: *const core::ffi::c_char,
    elem25: *const core::ffi::c_char,
    elem26: *const core::ffi::c_char,
    elem27: *const core::ffi::c_char,
    elem28: *const core::ffi::c_char,
    elem29: *const core::ffi::c_char,
    elem30: *const core::ffi::c_char,
    elem31: *const core::ffi::c_char,
    elem32: *const core::ffi::c_char,
}
#[repr(C)]
#[randomize_layout]
struct TestStruct_33 {
    elem1: *const core::ffi::c_char,
    elem2: *const core::ffi::c_char,
    elem3: *const core::ffi::c_char,
    elem4: *const core::ffi::c_char,
    elem5: *const core::ffi::c_char,
    elem6: *const core::ffi::c_char,
    elem7: *const core::ffi::c_char,
    elem8: *const core::ffi::c_char,
    elem9: *const core::ffi::c_char,
    elem10: *const core::ffi::c_char,
    elem11: *const core::ffi::c_char,
    elem12: *const core::ffi::c_char,
    elem13: *const core::ffi::c_char,
    elem14: *const core::ffi::c_char,
    elem15: *const core::ffi::c_char,
    elem16: *const core::ffi::c_char,
    elem17: *const core::ffi::c_char,
    elem18: *const core::ffi::c_char,
    elem19: *const core::ffi::c_char,
    elem20: *const core::ffi::c_char,
    elem21: *const core::ffi::c_char,
    elem22: *const core::ffi::c_char,
    elem23: *const core::ffi::c_char,
    elem24: *const core::ffi::c_char,
    elem25: *const core::ffi::c_char,
    elem26: *const core::ffi::c_char,
    elem27: *const core::ffi::c_char,
    elem28: *const core::ffi::c_char,
    elem29: *const core::ffi::c_char,
    elem30: *const core::ffi::c_char,
    elem31: *const core::ffi::c_char,
    elem32: *const core::ffi::c_char,
    elem33: *const core::ffi::c_char,
}
#[repr(C)]
#[randomize_layout]
struct TestStruct_34 {
    elem1: *const core::ffi::c_char,
    elem2: *const core::ffi::c_char,
    elem3: *const core::ffi::c_char,
    elem4: *const core::ffi::c_char,
    elem5: *const core::ffi::c_char,
    elem6: *const core::ffi::c_char,
    elem7: *const core::ffi::c_char,
    elem8: *const core::ffi::c_char,
    elem9: *const core::ffi::c_char,
    elem10: *const core::ffi::c_char,
    elem11: *const core::ffi::c_char,
    elem12: *const core::ffi::c_char,
    elem13: *const core::ffi::c_char,
    elem14: *const core::ffi::c_char,
    elem15: *const core::ffi::c_char,
    elem16: *const core::ffi::c_char,
    elem17: *const core::ffi::c_char,
    elem18: *const core::ffi::c_char,
    elem19: *const core::ffi::c_char,
    elem20: *const core::ffi::c_char,
    elem21: *const core::ffi::c_char,
    elem22: *const core::ffi::c_char,
    elem23: *const core::ffi::c_char,
    elem24: *const core::ffi::c_char,
    elem25: *const core::ffi::c_char,
    elem26: *const core::ffi::c_char,
    elem27: *const core::ffi::c_char,
    elem28: *const core::ffi::c_char,
    elem29: *const core::ffi::c_char,
    elem30: *const core::ffi::c_char,
    elem31: *const core::ffi::c_char,
    elem32: *const core::ffi::c_char,
    elem33: *const core::ffi::c_char,
    elem34: *const core::ffi::c_char,
}
#[repr(C)]
#[randomize_layout]
struct TestStruct_35 {
    elem1: *const core::ffi::c_char,
    elem2: *const core::ffi::c_char,
    elem3: *const core::ffi::c_char,
    elem4: *const core::ffi::c_char,
    elem5: *const core::ffi::c_char,
    elem6: *const core::ffi::c_char,
    elem7: *const core::ffi::c_char,
    elem8: *const core::ffi::c_char,
    elem9: *const core::ffi::c_char,
    elem10: *const core::ffi::c_char,
    elem11: *const core::ffi::c_char,
    elem12: *const core::ffi::c_char,
    elem13: *const core::ffi::c_char,
    elem14: *const core::ffi::c_char,
    elem15: *const core::ffi::c_char,
    elem16: *const core::ffi::c_char,
    elem17: *const core::ffi::c_char,
    elem18: *const core::ffi::c_char,
    elem19: *const core::ffi::c_char,
    elem20: *const core::ffi::c_char,
    elem21: *const core::ffi::c_char,
    elem22: *const core::ffi::c_char,
    elem23: *const core::ffi::c_char,
    elem24: *const core::ffi::c_char,
    elem25: *const core::ffi::c_char,
    elem26: *const core::ffi::c_char,
    elem27: *const core::ffi::c_char,
    elem28: *const core::ffi::c_char,
    elem29: *const core::ffi::c_char,
    elem30: *const core::ffi::c_char,
    elem31: *const core::ffi::c_char,
    elem32: *const core::ffi::c_char,
    elem33: *const core::ffi::c_char,
    elem34: *const core::ffi::c_char,
    elem35: *const core::ffi::c_char,
}
#[repr(C)]
#[randomize_layout]
struct TestStruct_36 {
    elem1: *const core::ffi::c_char,
    elem2: *const core::ffi::c_char,
    elem3: *const core::ffi::c_char,
    elem4: *const core::ffi::c_char,
    elem5: *const core::ffi::c_char,
    elem6: *const core::ffi::c_char,
    elem7: *const core::ffi::c_char,
    elem8: *const core::ffi::c_char,
    elem9: *const core::ffi::c_char,
    elem10: *const core::ffi::c_char,
    elem11: *const core::ffi::c_char,
    elem12: *const core::ffi::c_char,
    elem13: *const core::ffi::c_char,
    elem14: *const core::ffi::c_char,
    elem15: *const core::ffi::c_char,
    elem16: *const core::ffi::c_char,
    elem17: *const core::ffi::c_char,
    elem18: *const core::ffi::c_char,
    elem19: *const core::ffi::c_char,
    elem20: *const core::ffi::c_char,
    elem21: *const core::ffi::c_char,
    elem22: *const core::ffi::c_char,
    elem23: *const core::ffi::c_char,
    elem24: *const core::ffi::c_char,
    elem25: *const core::ffi::c_char,
    elem26: *const core::ffi::c_char,
    elem27: *const core::ffi::c_char,
    elem28: *const core::ffi::c_char,
    elem29: *const core::ffi::c_char,
    elem30: *const core::ffi::c_char,
    elem31: *const core::ffi::c_char,
    elem32: *const core::ffi::c_char,
    elem33: *const core::ffi::c_char,
    elem34: *const core::ffi::c_char,
    elem35: *const core::ffi::c_char,
    elem36: *const core::ffi::c_char,
}
#[repr(C)]
#[randomize_layout]
struct TestStruct_37 {
    elem1: *const core::ffi::c_char,
    elem2: *const core::ffi::c_char,
    elem3: *const core::ffi::c_char,
    elem4: *const core::ffi::c_char,
    elem5: *const core::ffi::c_char,
    elem6: *const core::ffi::c_char,
    elem7: *const core::ffi::c_char,
    elem8: *const core::ffi::c_char,
    elem9: *const core::ffi::c_char,
    elem10: *const core::ffi::c_char,
    elem11: *const core::ffi::c_char,
    elem12: *const core::ffi::c_char,
    elem13: *const core::ffi::c_char,
    elem14: *const core::ffi::c_char,
    elem15: *const core::ffi::c_char,
    elem16: *const core::ffi::c_char,
    elem17: *const core::ffi::c_char,
    elem18: *const core::ffi::c_char,
    elem19: *const core::ffi::c_char,
    elem20: *const core::ffi::c_char,
    elem21: *const core::ffi::c_char,
    elem22: *const core::ffi::c_char,
    elem23: *const core::ffi::c_char,
    elem24: *const core::ffi::c_char,
    elem25: *const core::ffi::c_char,
    elem26: *const core::ffi::c_char,
    elem27: *const core::ffi::c_char,
    elem28: *const core::ffi::c_char,
    elem29: *const core::ffi::c_char,
    elem30: *const core::ffi::c_char,
    elem31: *const core::ffi::c_char,
    elem32: *const core::ffi::c_char,
    elem33: *const core::ffi::c_char,
    elem34: *const core::ffi::c_char,
    elem35: *const core::ffi::c_char,
    elem36: *const core::ffi::c_char,
    elem37: *const core::ffi::c_char,
}
#[repr(C)]
#[randomize_layout]
struct TestStruct_38 {
    elem1: *const core::ffi::c_char,
    elem2: *const core::ffi::c_char,
    elem3: *const core::ffi::c_char,
    elem4: *const core::ffi::c_char,
    elem5: *const core::ffi::c_char,
    elem6: *const core::ffi::c_char,
    elem7: *const core::ffi::c_char,
    elem8: *const core::ffi::c_char,
    elem9: *const core::ffi::c_char,
    elem10: *const core::ffi::c_char,
    elem11: *const core::ffi::c_char,
    elem12: *const core::ffi::c_char,
    elem13: *const core::ffi::c_char,
    elem14: *const core::ffi::c_char,
    elem15: *const core::ffi::c_char,
    elem16: *const core::ffi::c_char,
    elem17: *const core::ffi::c_char,
    elem18: *const core::ffi::c_char,
    elem19: *const core::ffi::c_char,
    elem20: *const core::ffi::c_char,
    elem21: *const core::ffi::c_char,
    elem22: *const core::ffi::c_char,
    elem23: *const core::ffi::c_char,
    elem24: *const core::ffi::c_char,
    elem25: *const core::ffi::c_char,
    elem26: *const core::ffi::c_char,
    elem27: *const core::ffi::c_char,
    elem28: *const core::ffi::c_char,
    elem29: *const core::ffi::c_char,
    elem30: *const core::ffi::c_char,
    elem31: *const core::ffi::c_char,
    elem32: *const core::ffi::c_char,
    elem33: *const core::ffi::c_char,
    elem34: *const core::ffi::c_char,
    elem35: *const core::ffi::c_char,
    elem36: *const core::ffi::c_char,
    elem37: *const core::ffi::c_char,
    elem38: *const core::ffi::c_char,
}
#[repr(C)]
#[randomize_layout]
struct TestStruct_39 {
    elem1: *const core::ffi::c_char,
    elem2: *const core::ffi::c_char,
    elem3: *const core::ffi::c_char,
    elem4: *const core::ffi::c_char,
    elem5: *const core::ffi::c_char,
    elem6: *const core::ffi::c_char,
    elem7: *const core::ffi::c_char,
    elem8: *const core::ffi::c_char,
    elem9: *const core::ffi::c_char,
    elem10: *const core::ffi::c_char,
    elem11: *const core::ffi::c_char,
    elem12: *const core::ffi::c_char,
    elem13: *const core::ffi::c_char,
    elem14: *const core::ffi::c_char,
    elem15: *const core::ffi::c_char,
    elem16: *const core::ffi::c_char,
    elem17: *const core::ffi::c_char,
    elem18: *const core::ffi::c_char,
    elem19: *const core::ffi::c_char,
    elem20: *const core::ffi::c_char,
    elem21: *const core::ffi::c_char,
    elem22: *const core::ffi::c_char,
    elem23: *const core::ffi::c_char,
    elem24: *const core::ffi::c_char,
    elem25: *const core::ffi::c_char,
    elem26: *const core::ffi::c_char,
    elem27: *const core::ffi::c_char,
    elem28: *const core::ffi::c_char,
    elem29: *const core::ffi::c_char,
    elem30: *const core::ffi::c_char,
    elem31: *const core::ffi::c_char,
    elem32: *const core::ffi::c_char,
    elem33: *const core::ffi::c_char,
    elem34: *const core::ffi::c_char,
    elem35: *const core::ffi::c_char,
    elem36: *const core::ffi::c_char,
    elem37: *const core::ffi::c_char,
    elem38: *const core::ffi::c_char,
    elem39: *const core::ffi::c_char,
}
#[repr(C)]
#[randomize_layout]
struct TestStruct_40 {
    elem1: *const core::ffi::c_char,
    elem2: *const core::ffi::c_char,
    elem3: *const core::ffi::c_char,
    elem4: *const core::ffi::c_char,
    elem5: *const core::ffi::c_char,
    elem6: *const core::ffi::c_char,
    elem7: *const core::ffi::c_char,
    elem8: *const core::ffi::c_char,
    elem9: *const core::ffi::c_char,
    elem10: *const core::ffi::c_char,
    elem11: *const core::ffi::c_char,
    elem12: *const core::ffi::c_char,
    elem13: *const core::ffi::c_char,
    elem14: *const core::ffi::c_char,
    elem15: *const core::ffi::c_char,
    elem16: *const core::ffi::c_char,
    elem17: *const core::ffi::c_char,
    elem18: *const core::ffi::c_char,
    elem19: *const core::ffi::c_char,
    elem20: *const core::ffi::c_char,
    elem21: *const core::ffi::c_char,
    elem22: *const core::ffi::c_char,
    elem23: *const core::ffi::c_char,
    elem24: *const core::ffi::c_char,
    elem25: *const core::ffi::c_char,
    elem26: *const core::ffi::c_char,
    elem27: *const core::ffi::c_char,
    elem28: *const core::ffi::c_char,
    elem29: *const core::ffi::c_char,
    elem30: *const core::ffi::c_char,
    elem31: *const core::ffi::c_char,
    elem32: *const core::ffi::c_char,
    elem33: *const core::ffi::c_char,
    elem34: *const core::ffi::c_char,
    elem35: *const core::ffi::c_char,
    elem36: *const core::ffi::c_char,
    elem37: *const core::ffi::c_char,
    elem38: *const core::ffi::c_char,
    elem39: *const core::ffi::c_char,
    elem40: *const core::ffi::c_char,
}
#[repr(C)]
#[randomize_layout]
struct TestStruct_41 {
    elem1: *const core::ffi::c_char,
    elem2: *const core::ffi::c_char,
    elem3: *const core::ffi::c_char,
    elem4: *const core::ffi::c_char,
    elem5: *const core::ffi::c_char,
    elem6: *const core::ffi::c_char,
    elem7: *const core::ffi::c_char,
    elem8: *const core::ffi::c_char,
    elem9: *const core::ffi::c_char,
    elem10: *const core::ffi::c_char,
    elem11: *const core::ffi::c_char,
    elem12: *const core::ffi::c_char,
    elem13: *const core::ffi::c_char,
    elem14: *const core::ffi::c_char,
    elem15: *const core::ffi::c_char,
    elem16: *const core::ffi::c_char,
    elem17: *const core::ffi::c_char,
    elem18: *const core::ffi::c_char,
    elem19: *const core::ffi::c_char,
    elem20: *const core::ffi::c_char,
    elem21: *const core::ffi::c_char,
    elem22: *const core::ffi::c_char,
    elem23: *const core::ffi::c_char,
    elem24: *const core::ffi::c_char,
    elem25: *const core::ffi::c_char,
    elem26: *const core::ffi::c_char,
    elem27: *const core::ffi::c_char,
    elem28: *const core::ffi::c_char,
    elem29: *const core::ffi::c_char,
    elem30: *const core::ffi::c_char,
    elem31: *const core::ffi::c_char,
    elem32: *const core::ffi::c_char,
    elem33: *const core::ffi::c_char,
    elem34: *const core::ffi::c_char,
    elem35: *const core::ffi::c_char,
    elem36: *const core::ffi::c_char,
    elem37: *const core::ffi::c_char,
    elem38: *const core::ffi::c_char,
    elem39: *const core::ffi::c_char,
    elem40: *const core::ffi::c_char,
    elem41: *const core::ffi::c_char,
}
#[repr(C)]
#[randomize_layout]
struct TestStruct_42 {
    elem1: *const core::ffi::c_char,
    elem2: *const core::ffi::c_char,
    elem3: *const core::ffi::c_char,
    elem4: *const core::ffi::c_char,
    elem5: *const core::ffi::c_char,
    elem6: *const core::ffi::c_char,
    elem7: *const core::ffi::c_char,
    elem8: *const core::ffi::c_char,
    elem9: *const core::ffi::c_char,
    elem10: *const core::ffi::c_char,
    elem11: *const core::ffi::c_char,
    elem12: *const core::ffi::c_char,
    elem13: *const core::ffi::c_char,
    elem14: *const core::ffi::c_char,
    elem15: *const core::ffi::c_char,
    elem16: *const core::ffi::c_char,
    elem17: *const core::ffi::c_char,
    elem18: *const core::ffi::c_char,
    elem19: *const core::ffi::c_char,
    elem20: *const core::ffi::c_char,
    elem21: *const core::ffi::c_char,
    elem22: *const core::ffi::c_char,
    elem23: *const core::ffi::c_char,
    elem24: *const core::ffi::c_char,
    elem25: *const core::ffi::c_char,
    elem26: *const core::ffi::c_char,
    elem27: *const core::ffi::c_char,
    elem28: *const core::ffi::c_char,
    elem29: *const core::ffi::c_char,
    elem30: *const core::ffi::c_char,
    elem31: *const core::ffi::c_char,
    elem32: *const core::ffi::c_char,
    elem33: *const core::ffi::c_char,
    elem34: *const core::ffi::c_char,
    elem35: *const core::ffi::c_char,
    elem36: *const core::ffi::c_char,
    elem37: *const core::ffi::c_char,
    elem38: *const core::ffi::c_char,
    elem39: *const core::ffi::c_char,
    elem40: *const core::ffi::c_char,
    elem41: *const core::ffi::c_char,
    elem42: *const core::ffi::c_char,
}
#[repr(C)]
#[randomize_layout]
struct TestStruct_43 {
    elem1: *const core::ffi::c_char,
    elem2: *const core::ffi::c_char,
    elem3: *const core::ffi::c_char,
    elem4: *const core::ffi::c_char,
    elem5: *const core::ffi::c_char,
    elem6: *const core::ffi::c_char,
    elem7: *const core::ffi::c_char,
    elem8: *const core::ffi::c_char,
    elem9: *const core::ffi::c_char,
    elem10: *const core::ffi::c_char,
    elem11: *const core::ffi::c_char,
    elem12: *const core::ffi::c_char,
    elem13: *const core::ffi::c_char,
    elem14: *const core::ffi::c_char,
    elem15: *const core::ffi::c_char,
    elem16: *const core::ffi::c_char,
    elem17: *const core::ffi::c_char,
    elem18: *const core::ffi::c_char,
    elem19: *const core::ffi::c_char,
    elem20: *const core::ffi::c_char,
    elem21: *const core::ffi::c_char,
    elem22: *const core::ffi::c_char,
    elem23: *const core::ffi::c_char,
    elem24: *const core::ffi::c_char,
    elem25: *const core::ffi::c_char,
    elem26: *const core::ffi::c_char,
    elem27: *const core::ffi::c_char,
    elem28: *const core::ffi::c_char,
    elem29: *const core::ffi::c_char,
    elem30: *const core::ffi::c_char,
    elem31: *const core::ffi::c_char,
    elem32: *const core::ffi::c_char,
    elem33: *const core::ffi::c_char,
    elem34: *const core::ffi::c_char,
    elem35: *const core::ffi::c_char,
    elem36: *const core::ffi::c_char,
    elem37: *const core::ffi::c_char,
    elem38: *const core::ffi::c_char,
    elem39: *const core::ffi::c_char,
    elem40: *const core::ffi::c_char,
    elem41: *const core::ffi::c_char,
    elem42: *const core::ffi::c_char,
    elem43: *const core::ffi::c_char,
}
#[repr(C)]
#[randomize_layout]
struct TestStruct_44 {
    elem1: *const core::ffi::c_char,
    elem2: *const core::ffi::c_char,
    elem3: *const core::ffi::c_char,
    elem4: *const core::ffi::c_char,
    elem5: *const core::ffi::c_char,
    elem6: *const core::ffi::c_char,
    elem7: *const core::ffi::c_char,
    elem8: *const core::ffi::c_char,
    elem9: *const core::ffi::c_char,
    elem10: *const core::ffi::c_char,
    elem11: *const core::ffi::c_char,
    elem12: *const core::ffi::c_char,
    elem13: *const core::ffi::c_char,
    elem14: *const core::ffi::c_char,
    elem15: *const core::ffi::c_char,
    elem16: *const core::ffi::c_char,
    elem17: *const core::ffi::c_char,
    elem18: *const core::ffi::c_char,
    elem19: *const core::ffi::c_char,
    elem20: *const core::ffi::c_char,
    elem21: *const core::ffi::c_char,
    elem22: *const core::ffi::c_char,
    elem23: *const core::ffi::c_char,
    elem24: *const core::ffi::c_char,
    elem25: *const core::ffi::c_char,
    elem26: *const core::ffi::c_char,
    elem27: *const core::ffi::c_char,
    elem28: *const core::ffi::c_char,
    elem29: *const core::ffi::c_char,
    elem30: *const core::ffi::c_char,
    elem31: *const core::ffi::c_char,
    elem32: *const core::ffi::c_char,
    elem33: *const core::ffi::c_char,
    elem34: *const core::ffi::c_char,
    elem35: *const core::ffi::c_char,
    elem36: *const core::ffi::c_char,
    elem37: *const core::ffi::c_char,
    elem38: *const core::ffi::c_char,
    elem39: *const core::ffi::c_char,
    elem40: *const core::ffi::c_char,
    elem41: *const core::ffi::c_char,
    elem42: *const core::ffi::c_char,
    elem43: *const core::ffi::c_char,
    elem44: *const core::ffi::c_char,
}
#[repr(C)]
#[randomize_layout]
struct TestStruct_45 {
    elem1: *const core::ffi::c_char,
    elem2: *const core::ffi::c_char,
    elem3: *const core::ffi::c_char,
    elem4: *const core::ffi::c_char,
    elem5: *const core::ffi::c_char,
    elem6: *const core::ffi::c_char,
    elem7: *const core::ffi::c_char,
    elem8: *const core::ffi::c_char,
    elem9: *const core::ffi::c_char,
    elem10: *const core::ffi::c_char,
    elem11: *const core::ffi::c_char,
    elem12: *const core::ffi::c_char,
    elem13: *const core::ffi::c_char,
    elem14: *const core::ffi::c_char,
    elem15: *const core::ffi::c_char,
    elem16: *const core::ffi::c_char,
    elem17: *const core::ffi::c_char,
    elem18: *const core::ffi::c_char,
    elem19: *const core::ffi::c_char,
    elem20: *const core::ffi::c_char,
    elem21: *const core::ffi::c_char,
    elem22: *const core::ffi::c_char,
    elem23: *const core::ffi::c_char,
    elem24: *const core::ffi::c_char,
    elem25: *const core::ffi::c_char,
    elem26: *const core::ffi::c_char,
    elem27: *const core::ffi::c_char,
    elem28: *const core::ffi::c_char,
    elem29: *const core::ffi::c_char,
    elem30: *const core::ffi::c_char,
    elem31: *const core::ffi::c_char,
    elem32: *const core::ffi::c_char,
    elem33: *const core::ffi::c_char,
    elem34: *const core::ffi::c_char,
    elem35: *const core::ffi::c_char,
    elem36: *const core::ffi::c_char,
    elem37: *const core::ffi::c_char,
    elem38: *const core::ffi::c_char,
    elem39: *const core::ffi::c_char,
    elem40: *const core::ffi::c_char,
    elem41: *const core::ffi::c_char,
    elem42: *const core::ffi::c_char,
    elem43: *const core::ffi::c_char,
    elem44: *const core::ffi::c_char,
    elem45: *const core::ffi::c_char,
}
#[repr(C)]
#[randomize_layout]
struct TestStruct_46 {
    elem1: *const core::ffi::c_char,
    elem2: *const core::ffi::c_char,
    elem3: *const core::ffi::c_char,
    elem4: *const core::ffi::c_char,
    elem5: *const core::ffi::c_char,
    elem6: *const core::ffi::c_char,
    elem7: *const core::ffi::c_char,
    elem8: *const core::ffi::c_char,
    elem9: *const core::ffi::c_char,
    elem10: *const core::ffi::c_char,
    elem11: *const core::ffi::c_char,
    elem12: *const core::ffi::c_char,
    elem13: *const core::ffi::c_char,
    elem14: *const core::ffi::c_char,
    elem15: *const core::ffi::c_char,
    elem16: *const core::ffi::c_char,
    elem17: *const core::ffi::c_char,
    elem18: *const core::ffi::c_char,
    elem19: *const core::ffi::c_char,
    elem20: *const core::ffi::c_char,
    elem21: *const core::ffi::c_char,
    elem22: *const core::ffi::c_char,
    elem23: *const core::ffi::c_char,
    elem24: *const core::ffi::c_char,
    elem25: *const core::ffi::c_char,
    elem26: *const core::ffi::c_char,
    elem27: *const core::ffi::c_char,
    elem28: *const core::ffi::c_char,
    elem29: *const core::ffi::c_char,
    elem30: *const core::ffi::c_char,
    elem31: *const core::ffi::c_char,
    elem32: *const core::ffi::c_char,
    elem33: *const core::ffi::c_char,
    elem34: *const core::ffi::c_char,
    elem35: *const core::ffi::c_char,
    elem36: *const core::ffi::c_char,
    elem37: *const core::ffi::c_char,
    elem38: *const core::ffi::c_char,
    elem39: *const core::ffi::c_char,
    elem40: *const core::ffi::c_char,
    elem41: *const core::ffi::c_char,
    elem42: *const core::ffi::c_char,
    elem43: *const core::ffi::c_char,
    elem44: *const core::ffi::c_char,
    elem45: *const core::ffi::c_char,
    elem46: *const core::ffi::c_char,
}
#[repr(C)]
#[randomize_layout]
struct TestStruct_47 {
    elem1: *const core::ffi::c_char,
    elem2: *const core::ffi::c_char,
    elem3: *const core::ffi::c_char,
    elem4: *const core::ffi::c_char,
    elem5: *const core::ffi::c_char,
    elem6: *const core::ffi::c_char,
    elem7: *const core::ffi::c_char,
    elem8: *const core::ffi::c_char,
    elem9: *const core::ffi::c_char,
    elem10: *const core::ffi::c_char,
    elem11: *const core::ffi::c_char,
    elem12: *const core::ffi::c_char,
    elem13: *const core::ffi::c_char,
    elem14: *const core::ffi::c_char,
    elem15: *const core::ffi::c_char,
    elem16: *const core::ffi::c_char,
    elem17: *const core::ffi::c_char,
    elem18: *const core::ffi::c_char,
    elem19: *const core::ffi::c_char,
    elem20: *const core::ffi::c_char,
    elem21: *const core::ffi::c_char,
    elem22: *const core::ffi::c_char,
    elem23: *const core::ffi::c_char,
    elem24: *const core::ffi::c_char,
    elem25: *const core::ffi::c_char,
    elem26: *const core::ffi::c_char,
    elem27: *const core::ffi::c_char,
    elem28: *const core::ffi::c_char,
    elem29: *const core::ffi::c_char,
    elem30: *const core::ffi::c_char,
    elem31: *const core::ffi::c_char,
    elem32: *const core::ffi::c_char,
    elem33: *const core::ffi::c_char,
    elem34: *const core::ffi::c_char,
    elem35: *const core::ffi::c_char,
    elem36: *const core::ffi::c_char,
    elem37: *const core::ffi::c_char,
    elem38: *const core::ffi::c_char,
    elem39: *const core::ffi::c_char,
    elem40: *const core::ffi::c_char,
    elem41: *const core::ffi::c_char,
    elem42: *const core::ffi::c_char,
    elem43: *const core::ffi::c_char,
    elem44: *const core::ffi::c_char,
    elem45: *const core::ffi::c_char,
    elem46: *const core::ffi::c_char,
    elem47: *const core::ffi::c_char,
}
#[repr(C)]
#[randomize_layout]
struct TestStruct_48 {
    elem1: *const core::ffi::c_char,
    elem2: *const core::ffi::c_char,
    elem3: *const core::ffi::c_char,
    elem4: *const core::ffi::c_char,
    elem5: *const core::ffi::c_char,
    elem6: *const core::ffi::c_char,
    elem7: *const core::ffi::c_char,
    elem8: *const core::ffi::c_char,
    elem9: *const core::ffi::c_char,
    elem10: *const core::ffi::c_char,
    elem11: *const core::ffi::c_char,
    elem12: *const core::ffi::c_char,
    elem13: *const core::ffi::c_char,
    elem14: *const core::ffi::c_char,
    elem15: *const core::ffi::c_char,
    elem16: *const core::ffi::c_char,
    elem17: *const core::ffi::c_char,
    elem18: *const core::ffi::c_char,
    elem19: *const core::ffi::c_char,
    elem20: *const core::ffi::c_char,
    elem21: *const core::ffi::c_char,
    elem22: *const core::ffi::c_char,
    elem23: *const core::ffi::c_char,
    elem24: *const core::ffi::c_char,
    elem25: *const core::ffi::c_char,
    elem26: *const core::ffi::c_char,
    elem27: *const core::ffi::c_char,
    elem28: *const core::ffi::c_char,
    elem29: *const core::ffi::c_char,
    elem30: *const core::ffi::c_char,
    elem31: *const core::ffi::c_char,
    elem32: *const core::ffi::c_char,
    elem33: *const core::ffi::c_char,
    elem34: *const core::ffi::c_char,
    elem35: *const core::ffi::c_char,
    elem36: *const core::ffi::c_char,
    elem37: *const core::ffi::c_char,
    elem38: *const core::ffi::c_char,
    elem39: *const core::ffi::c_char,
    elem40: *const core::ffi::c_char,
    elem41: *const core::ffi::c_char,
    elem42: *const core::ffi::c_char,
    elem43: *const core::ffi::c_char,
    elem44: *const core::ffi::c_char,
    elem45: *const core::ffi::c_char,
    elem46: *const core::ffi::c_char,
    elem47: *const core::ffi::c_char,
    elem48: *const core::ffi::c_char,
}
#[repr(C)]
#[randomize_layout]
struct TestStruct_49 {
    elem1: *const core::ffi::c_char,
    elem2: *const core::ffi::c_char,
    elem3: *const core::ffi::c_char,
    elem4: *const core::ffi::c_char,
    elem5: *const core::ffi::c_char,
    elem6: *const core::ffi::c_char,
    elem7: *const core::ffi::c_char,
    elem8: *const core::ffi::c_char,
    elem9: *const core::ffi::c_char,
    elem10: *const core::ffi::c_char,
    elem11: *const core::ffi::c_char,
    elem12: *const core::ffi::c_char,
    elem13: *const core::ffi::c_char,
    elem14: *const core::ffi::c_char,
    elem15: *const core::ffi::c_char,
    elem16: *const core::ffi::c_char,
    elem17: *const core::ffi::c_char,
    elem18: *const core::ffi::c_char,
    elem19: *const core::ffi::c_char,
    elem20: *const core::ffi::c_char,
    elem21: *const core::ffi::c_char,
    elem22: *const core::ffi::c_char,
    elem23: *const core::ffi::c_char,
    elem24: *const core::ffi::c_char,
    elem25: *const core::ffi::c_char,
    elem26: *const core::ffi::c_char,
    elem27: *const core::ffi::c_char,
    elem28: *const core::ffi::c_char,
    elem29: *const core::ffi::c_char,
    elem30: *const core::ffi::c_char,
    elem31: *const core::ffi::c_char,
    elem32: *const core::ffi::c_char,
    elem33: *const core::ffi::c_char,
    elem34: *const core::ffi::c_char,
    elem35: *const core::ffi::c_char,
    elem36: *const core::ffi::c_char,
    elem37: *const core::ffi::c_char,
    elem38: *const core::ffi::c_char,
    elem39: *const core::ffi::c_char,
    elem40: *const core::ffi::c_char,
    elem41: *const core::ffi::c_char,
    elem42: *const core::ffi::c_char,
    elem43: *const core::ffi::c_char,
    elem44: *const core::ffi::c_char,
    elem45: *const core::ffi::c_char,
    elem46: *const core::ffi::c_char,
    elem47: *const core::ffi::c_char,
    elem48: *const core::ffi::c_char,
    elem49: *const core::ffi::c_char,
}
#[repr(C)]
#[randomize_layout]
struct TestStruct_50 {
    elem1: *const core::ffi::c_char,
    elem2: *const core::ffi::c_char,
    elem3: *const core::ffi::c_char,
    elem4: *const core::ffi::c_char,
    elem5: *const core::ffi::c_char,
    elem6: *const core::ffi::c_char,
    elem7: *const core::ffi::c_char,
    elem8: *const core::ffi::c_char,
    elem9: *const core::ffi::c_char,
    elem10: *const core::ffi::c_char,
    elem11: *const core::ffi::c_char,
    elem12: *const core::ffi::c_char,
    elem13: *const core::ffi::c_char,
    elem14: *const core::ffi::c_char,
    elem15: *const core::ffi::c_char,
    elem16: *const core::ffi::c_char,
    elem17: *const core::ffi::c_char,
    elem18: *const core::ffi::c_char,
    elem19: *const core::ffi::c_char,
    elem20: *const core::ffi::c_char,
    elem21: *const core::ffi::c_char,
    elem22: *const core::ffi::c_char,
    elem23: *const core::ffi::c_char,
    elem24: *const core::ffi::c_char,
    elem25: *const core::ffi::c_char,
    elem26: *const core::ffi::c_char,
    elem27: *const core::ffi::c_char,
    elem28: *const core::ffi::c_char,
    elem29: *const core::ffi::c_char,
    elem30: *const core::ffi::c_char,
    elem31: *const core::ffi::c_char,
    elem32: *const core::ffi::c_char,
    elem33: *const core::ffi::c_char,
    elem34: *const core::ffi::c_char,
    elem35: *const core::ffi::c_char,
    elem36: *const core::ffi::c_char,
    elem37: *const core::ffi::c_char,
    elem38: *const core::ffi::c_char,
    elem39: *const core::ffi::c_char,
    elem40: *const core::ffi::c_char,
    elem41: *const core::ffi::c_char,
    elem42: *const core::ffi::c_char,
    elem43: *const core::ffi::c_char,
    elem44: *const core::ffi::c_char,
    elem45: *const core::ffi::c_char,
    elem46: *const core::ffi::c_char,
    elem47: *const core::ffi::c_char,
    elem48: *const core::ffi::c_char,
    elem49: *const core::ffi::c_char,
    elem50: *const core::ffi::c_char,
}

fn main() {
    let randstruct_seed = env::var("RANDSTRUCT_SEED").unwrap();
    let file = File::open("test_cases.json").expect("Failed to open test case file");
    let json: serde_json::Value = serde_json::from_reader(file).expect("Failed to parse json from file");
    let test_case: &serde_json::Value = json.get(randstruct_seed).unwrap();

assert_eq!(offset_of!(TestStruct_1, elem1) as u64, test_case.get("TestStruct_1").unwrap().get("elem1_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_2, elem1) as u64, test_case.get("TestStruct_2").unwrap().get("elem1_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_2, elem2) as u64, test_case.get("TestStruct_2").unwrap().get("elem2_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_3, elem1) as u64, test_case.get("TestStruct_3").unwrap().get("elem1_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_3, elem2) as u64, test_case.get("TestStruct_3").unwrap().get("elem2_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_3, elem3) as u64, test_case.get("TestStruct_3").unwrap().get("elem3_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_4, elem1) as u64, test_case.get("TestStruct_4").unwrap().get("elem1_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_4, elem2) as u64, test_case.get("TestStruct_4").unwrap().get("elem2_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_4, elem3) as u64, test_case.get("TestStruct_4").unwrap().get("elem3_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_4, elem4) as u64, test_case.get("TestStruct_4").unwrap().get("elem4_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_5, elem1) as u64, test_case.get("TestStruct_5").unwrap().get("elem1_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_5, elem2) as u64, test_case.get("TestStruct_5").unwrap().get("elem2_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_5, elem3) as u64, test_case.get("TestStruct_5").unwrap().get("elem3_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_5, elem4) as u64, test_case.get("TestStruct_5").unwrap().get("elem4_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_5, elem5) as u64, test_case.get("TestStruct_5").unwrap().get("elem5_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_6, elem1) as u64, test_case.get("TestStruct_6").unwrap().get("elem1_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_6, elem2) as u64, test_case.get("TestStruct_6").unwrap().get("elem2_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_6, elem3) as u64, test_case.get("TestStruct_6").unwrap().get("elem3_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_6, elem4) as u64, test_case.get("TestStruct_6").unwrap().get("elem4_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_6, elem5) as u64, test_case.get("TestStruct_6").unwrap().get("elem5_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_6, elem6) as u64, test_case.get("TestStruct_6").unwrap().get("elem6_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_7, elem1) as u64, test_case.get("TestStruct_7").unwrap().get("elem1_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_7, elem2) as u64, test_case.get("TestStruct_7").unwrap().get("elem2_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_7, elem3) as u64, test_case.get("TestStruct_7").unwrap().get("elem3_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_7, elem4) as u64, test_case.get("TestStruct_7").unwrap().get("elem4_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_7, elem5) as u64, test_case.get("TestStruct_7").unwrap().get("elem5_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_7, elem6) as u64, test_case.get("TestStruct_7").unwrap().get("elem6_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_7, elem7) as u64, test_case.get("TestStruct_7").unwrap().get("elem7_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_8, elem1) as u64, test_case.get("TestStruct_8").unwrap().get("elem1_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_8, elem2) as u64, test_case.get("TestStruct_8").unwrap().get("elem2_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_8, elem3) as u64, test_case.get("TestStruct_8").unwrap().get("elem3_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_8, elem4) as u64, test_case.get("TestStruct_8").unwrap().get("elem4_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_8, elem5) as u64, test_case.get("TestStruct_8").unwrap().get("elem5_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_8, elem6) as u64, test_case.get("TestStruct_8").unwrap().get("elem6_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_8, elem7) as u64, test_case.get("TestStruct_8").unwrap().get("elem7_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_8, elem8) as u64, test_case.get("TestStruct_8").unwrap().get("elem8_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_9, elem1) as u64, test_case.get("TestStruct_9").unwrap().get("elem1_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_9, elem2) as u64, test_case.get("TestStruct_9").unwrap().get("elem2_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_9, elem3) as u64, test_case.get("TestStruct_9").unwrap().get("elem3_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_9, elem4) as u64, test_case.get("TestStruct_9").unwrap().get("elem4_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_9, elem5) as u64, test_case.get("TestStruct_9").unwrap().get("elem5_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_9, elem6) as u64, test_case.get("TestStruct_9").unwrap().get("elem6_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_9, elem7) as u64, test_case.get("TestStruct_9").unwrap().get("elem7_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_9, elem8) as u64, test_case.get("TestStruct_9").unwrap().get("elem8_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_9, elem9) as u64, test_case.get("TestStruct_9").unwrap().get("elem9_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_10, elem1) as u64, test_case.get("TestStruct_10").unwrap().get("elem1_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_10, elem2) as u64, test_case.get("TestStruct_10").unwrap().get("elem2_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_10, elem3) as u64, test_case.get("TestStruct_10").unwrap().get("elem3_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_10, elem4) as u64, test_case.get("TestStruct_10").unwrap().get("elem4_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_10, elem5) as u64, test_case.get("TestStruct_10").unwrap().get("elem5_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_10, elem6) as u64, test_case.get("TestStruct_10").unwrap().get("elem6_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_10, elem7) as u64, test_case.get("TestStruct_10").unwrap().get("elem7_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_10, elem8) as u64, test_case.get("TestStruct_10").unwrap().get("elem8_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_10, elem9) as u64, test_case.get("TestStruct_10").unwrap().get("elem9_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_10, elem10) as u64, test_case.get("TestStruct_10").unwrap().get("elem10_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_11, elem1) as u64, test_case.get("TestStruct_11").unwrap().get("elem1_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_11, elem2) as u64, test_case.get("TestStruct_11").unwrap().get("elem2_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_11, elem3) as u64, test_case.get("TestStruct_11").unwrap().get("elem3_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_11, elem4) as u64, test_case.get("TestStruct_11").unwrap().get("elem4_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_11, elem5) as u64, test_case.get("TestStruct_11").unwrap().get("elem5_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_11, elem6) as u64, test_case.get("TestStruct_11").unwrap().get("elem6_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_11, elem7) as u64, test_case.get("TestStruct_11").unwrap().get("elem7_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_11, elem8) as u64, test_case.get("TestStruct_11").unwrap().get("elem8_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_11, elem9) as u64, test_case.get("TestStruct_11").unwrap().get("elem9_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_11, elem10) as u64, test_case.get("TestStruct_11").unwrap().get("elem10_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_11, elem11) as u64, test_case.get("TestStruct_11").unwrap().get("elem11_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_12, elem1) as u64, test_case.get("TestStruct_12").unwrap().get("elem1_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_12, elem2) as u64, test_case.get("TestStruct_12").unwrap().get("elem2_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_12, elem3) as u64, test_case.get("TestStruct_12").unwrap().get("elem3_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_12, elem4) as u64, test_case.get("TestStruct_12").unwrap().get("elem4_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_12, elem5) as u64, test_case.get("TestStruct_12").unwrap().get("elem5_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_12, elem6) as u64, test_case.get("TestStruct_12").unwrap().get("elem6_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_12, elem7) as u64, test_case.get("TestStruct_12").unwrap().get("elem7_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_12, elem8) as u64, test_case.get("TestStruct_12").unwrap().get("elem8_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_12, elem9) as u64, test_case.get("TestStruct_12").unwrap().get("elem9_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_12, elem10) as u64, test_case.get("TestStruct_12").unwrap().get("elem10_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_12, elem11) as u64, test_case.get("TestStruct_12").unwrap().get("elem11_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_12, elem12) as u64, test_case.get("TestStruct_12").unwrap().get("elem12_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_13, elem1) as u64, test_case.get("TestStruct_13").unwrap().get("elem1_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_13, elem2) as u64, test_case.get("TestStruct_13").unwrap().get("elem2_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_13, elem3) as u64, test_case.get("TestStruct_13").unwrap().get("elem3_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_13, elem4) as u64, test_case.get("TestStruct_13").unwrap().get("elem4_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_13, elem5) as u64, test_case.get("TestStruct_13").unwrap().get("elem5_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_13, elem6) as u64, test_case.get("TestStruct_13").unwrap().get("elem6_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_13, elem7) as u64, test_case.get("TestStruct_13").unwrap().get("elem7_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_13, elem8) as u64, test_case.get("TestStruct_13").unwrap().get("elem8_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_13, elem9) as u64, test_case.get("TestStruct_13").unwrap().get("elem9_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_13, elem10) as u64, test_case.get("TestStruct_13").unwrap().get("elem10_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_13, elem11) as u64, test_case.get("TestStruct_13").unwrap().get("elem11_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_13, elem12) as u64, test_case.get("TestStruct_13").unwrap().get("elem12_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_13, elem13) as u64, test_case.get("TestStruct_13").unwrap().get("elem13_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_14, elem1) as u64, test_case.get("TestStruct_14").unwrap().get("elem1_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_14, elem2) as u64, test_case.get("TestStruct_14").unwrap().get("elem2_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_14, elem3) as u64, test_case.get("TestStruct_14").unwrap().get("elem3_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_14, elem4) as u64, test_case.get("TestStruct_14").unwrap().get("elem4_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_14, elem5) as u64, test_case.get("TestStruct_14").unwrap().get("elem5_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_14, elem6) as u64, test_case.get("TestStruct_14").unwrap().get("elem6_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_14, elem7) as u64, test_case.get("TestStruct_14").unwrap().get("elem7_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_14, elem8) as u64, test_case.get("TestStruct_14").unwrap().get("elem8_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_14, elem9) as u64, test_case.get("TestStruct_14").unwrap().get("elem9_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_14, elem10) as u64, test_case.get("TestStruct_14").unwrap().get("elem10_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_14, elem11) as u64, test_case.get("TestStruct_14").unwrap().get("elem11_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_14, elem12) as u64, test_case.get("TestStruct_14").unwrap().get("elem12_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_14, elem13) as u64, test_case.get("TestStruct_14").unwrap().get("elem13_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_14, elem14) as u64, test_case.get("TestStruct_14").unwrap().get("elem14_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_15, elem1) as u64, test_case.get("TestStruct_15").unwrap().get("elem1_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_15, elem2) as u64, test_case.get("TestStruct_15").unwrap().get("elem2_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_15, elem3) as u64, test_case.get("TestStruct_15").unwrap().get("elem3_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_15, elem4) as u64, test_case.get("TestStruct_15").unwrap().get("elem4_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_15, elem5) as u64, test_case.get("TestStruct_15").unwrap().get("elem5_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_15, elem6) as u64, test_case.get("TestStruct_15").unwrap().get("elem6_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_15, elem7) as u64, test_case.get("TestStruct_15").unwrap().get("elem7_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_15, elem8) as u64, test_case.get("TestStruct_15").unwrap().get("elem8_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_15, elem9) as u64, test_case.get("TestStruct_15").unwrap().get("elem9_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_15, elem10) as u64, test_case.get("TestStruct_15").unwrap().get("elem10_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_15, elem11) as u64, test_case.get("TestStruct_15").unwrap().get("elem11_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_15, elem12) as u64, test_case.get("TestStruct_15").unwrap().get("elem12_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_15, elem13) as u64, test_case.get("TestStruct_15").unwrap().get("elem13_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_15, elem14) as u64, test_case.get("TestStruct_15").unwrap().get("elem14_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_15, elem15) as u64, test_case.get("TestStruct_15").unwrap().get("elem15_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_16, elem1) as u64, test_case.get("TestStruct_16").unwrap().get("elem1_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_16, elem2) as u64, test_case.get("TestStruct_16").unwrap().get("elem2_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_16, elem3) as u64, test_case.get("TestStruct_16").unwrap().get("elem3_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_16, elem4) as u64, test_case.get("TestStruct_16").unwrap().get("elem4_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_16, elem5) as u64, test_case.get("TestStruct_16").unwrap().get("elem5_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_16, elem6) as u64, test_case.get("TestStruct_16").unwrap().get("elem6_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_16, elem7) as u64, test_case.get("TestStruct_16").unwrap().get("elem7_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_16, elem8) as u64, test_case.get("TestStruct_16").unwrap().get("elem8_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_16, elem9) as u64, test_case.get("TestStruct_16").unwrap().get("elem9_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_16, elem10) as u64, test_case.get("TestStruct_16").unwrap().get("elem10_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_16, elem11) as u64, test_case.get("TestStruct_16").unwrap().get("elem11_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_16, elem12) as u64, test_case.get("TestStruct_16").unwrap().get("elem12_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_16, elem13) as u64, test_case.get("TestStruct_16").unwrap().get("elem13_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_16, elem14) as u64, test_case.get("TestStruct_16").unwrap().get("elem14_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_16, elem15) as u64, test_case.get("TestStruct_16").unwrap().get("elem15_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_16, elem16) as u64, test_case.get("TestStruct_16").unwrap().get("elem16_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_17, elem1) as u64, test_case.get("TestStruct_17").unwrap().get("elem1_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_17, elem2) as u64, test_case.get("TestStruct_17").unwrap().get("elem2_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_17, elem3) as u64, test_case.get("TestStruct_17").unwrap().get("elem3_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_17, elem4) as u64, test_case.get("TestStruct_17").unwrap().get("elem4_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_17, elem5) as u64, test_case.get("TestStruct_17").unwrap().get("elem5_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_17, elem6) as u64, test_case.get("TestStruct_17").unwrap().get("elem6_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_17, elem7) as u64, test_case.get("TestStruct_17").unwrap().get("elem7_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_17, elem8) as u64, test_case.get("TestStruct_17").unwrap().get("elem8_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_17, elem9) as u64, test_case.get("TestStruct_17").unwrap().get("elem9_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_17, elem10) as u64, test_case.get("TestStruct_17").unwrap().get("elem10_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_17, elem11) as u64, test_case.get("TestStruct_17").unwrap().get("elem11_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_17, elem12) as u64, test_case.get("TestStruct_17").unwrap().get("elem12_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_17, elem13) as u64, test_case.get("TestStruct_17").unwrap().get("elem13_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_17, elem14) as u64, test_case.get("TestStruct_17").unwrap().get("elem14_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_17, elem15) as u64, test_case.get("TestStruct_17").unwrap().get("elem15_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_17, elem16) as u64, test_case.get("TestStruct_17").unwrap().get("elem16_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_17, elem17) as u64, test_case.get("TestStruct_17").unwrap().get("elem17_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_18, elem1) as u64, test_case.get("TestStruct_18").unwrap().get("elem1_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_18, elem2) as u64, test_case.get("TestStruct_18").unwrap().get("elem2_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_18, elem3) as u64, test_case.get("TestStruct_18").unwrap().get("elem3_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_18, elem4) as u64, test_case.get("TestStruct_18").unwrap().get("elem4_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_18, elem5) as u64, test_case.get("TestStruct_18").unwrap().get("elem5_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_18, elem6) as u64, test_case.get("TestStruct_18").unwrap().get("elem6_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_18, elem7) as u64, test_case.get("TestStruct_18").unwrap().get("elem7_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_18, elem8) as u64, test_case.get("TestStruct_18").unwrap().get("elem8_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_18, elem9) as u64, test_case.get("TestStruct_18").unwrap().get("elem9_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_18, elem10) as u64, test_case.get("TestStruct_18").unwrap().get("elem10_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_18, elem11) as u64, test_case.get("TestStruct_18").unwrap().get("elem11_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_18, elem12) as u64, test_case.get("TestStruct_18").unwrap().get("elem12_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_18, elem13) as u64, test_case.get("TestStruct_18").unwrap().get("elem13_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_18, elem14) as u64, test_case.get("TestStruct_18").unwrap().get("elem14_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_18, elem15) as u64, test_case.get("TestStruct_18").unwrap().get("elem15_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_18, elem16) as u64, test_case.get("TestStruct_18").unwrap().get("elem16_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_18, elem17) as u64, test_case.get("TestStruct_18").unwrap().get("elem17_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_18, elem18) as u64, test_case.get("TestStruct_18").unwrap().get("elem18_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_19, elem1) as u64, test_case.get("TestStruct_19").unwrap().get("elem1_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_19, elem2) as u64, test_case.get("TestStruct_19").unwrap().get("elem2_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_19, elem3) as u64, test_case.get("TestStruct_19").unwrap().get("elem3_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_19, elem4) as u64, test_case.get("TestStruct_19").unwrap().get("elem4_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_19, elem5) as u64, test_case.get("TestStruct_19").unwrap().get("elem5_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_19, elem6) as u64, test_case.get("TestStruct_19").unwrap().get("elem6_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_19, elem7) as u64, test_case.get("TestStruct_19").unwrap().get("elem7_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_19, elem8) as u64, test_case.get("TestStruct_19").unwrap().get("elem8_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_19, elem9) as u64, test_case.get("TestStruct_19").unwrap().get("elem9_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_19, elem10) as u64, test_case.get("TestStruct_19").unwrap().get("elem10_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_19, elem11) as u64, test_case.get("TestStruct_19").unwrap().get("elem11_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_19, elem12) as u64, test_case.get("TestStruct_19").unwrap().get("elem12_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_19, elem13) as u64, test_case.get("TestStruct_19").unwrap().get("elem13_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_19, elem14) as u64, test_case.get("TestStruct_19").unwrap().get("elem14_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_19, elem15) as u64, test_case.get("TestStruct_19").unwrap().get("elem15_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_19, elem16) as u64, test_case.get("TestStruct_19").unwrap().get("elem16_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_19, elem17) as u64, test_case.get("TestStruct_19").unwrap().get("elem17_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_19, elem18) as u64, test_case.get("TestStruct_19").unwrap().get("elem18_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_19, elem19) as u64, test_case.get("TestStruct_19").unwrap().get("elem19_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_20, elem1) as u64, test_case.get("TestStruct_20").unwrap().get("elem1_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_20, elem2) as u64, test_case.get("TestStruct_20").unwrap().get("elem2_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_20, elem3) as u64, test_case.get("TestStruct_20").unwrap().get("elem3_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_20, elem4) as u64, test_case.get("TestStruct_20").unwrap().get("elem4_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_20, elem5) as u64, test_case.get("TestStruct_20").unwrap().get("elem5_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_20, elem6) as u64, test_case.get("TestStruct_20").unwrap().get("elem6_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_20, elem7) as u64, test_case.get("TestStruct_20").unwrap().get("elem7_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_20, elem8) as u64, test_case.get("TestStruct_20").unwrap().get("elem8_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_20, elem9) as u64, test_case.get("TestStruct_20").unwrap().get("elem9_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_20, elem10) as u64, test_case.get("TestStruct_20").unwrap().get("elem10_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_20, elem11) as u64, test_case.get("TestStruct_20").unwrap().get("elem11_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_20, elem12) as u64, test_case.get("TestStruct_20").unwrap().get("elem12_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_20, elem13) as u64, test_case.get("TestStruct_20").unwrap().get("elem13_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_20, elem14) as u64, test_case.get("TestStruct_20").unwrap().get("elem14_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_20, elem15) as u64, test_case.get("TestStruct_20").unwrap().get("elem15_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_20, elem16) as u64, test_case.get("TestStruct_20").unwrap().get("elem16_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_20, elem17) as u64, test_case.get("TestStruct_20").unwrap().get("elem17_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_20, elem18) as u64, test_case.get("TestStruct_20").unwrap().get("elem18_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_20, elem19) as u64, test_case.get("TestStruct_20").unwrap().get("elem19_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_20, elem20) as u64, test_case.get("TestStruct_20").unwrap().get("elem20_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_21, elem1) as u64, test_case.get("TestStruct_21").unwrap().get("elem1_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_21, elem2) as u64, test_case.get("TestStruct_21").unwrap().get("elem2_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_21, elem3) as u64, test_case.get("TestStruct_21").unwrap().get("elem3_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_21, elem4) as u64, test_case.get("TestStruct_21").unwrap().get("elem4_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_21, elem5) as u64, test_case.get("TestStruct_21").unwrap().get("elem5_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_21, elem6) as u64, test_case.get("TestStruct_21").unwrap().get("elem6_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_21, elem7) as u64, test_case.get("TestStruct_21").unwrap().get("elem7_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_21, elem8) as u64, test_case.get("TestStruct_21").unwrap().get("elem8_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_21, elem9) as u64, test_case.get("TestStruct_21").unwrap().get("elem9_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_21, elem10) as u64, test_case.get("TestStruct_21").unwrap().get("elem10_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_21, elem11) as u64, test_case.get("TestStruct_21").unwrap().get("elem11_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_21, elem12) as u64, test_case.get("TestStruct_21").unwrap().get("elem12_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_21, elem13) as u64, test_case.get("TestStruct_21").unwrap().get("elem13_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_21, elem14) as u64, test_case.get("TestStruct_21").unwrap().get("elem14_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_21, elem15) as u64, test_case.get("TestStruct_21").unwrap().get("elem15_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_21, elem16) as u64, test_case.get("TestStruct_21").unwrap().get("elem16_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_21, elem17) as u64, test_case.get("TestStruct_21").unwrap().get("elem17_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_21, elem18) as u64, test_case.get("TestStruct_21").unwrap().get("elem18_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_21, elem19) as u64, test_case.get("TestStruct_21").unwrap().get("elem19_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_21, elem20) as u64, test_case.get("TestStruct_21").unwrap().get("elem20_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_21, elem21) as u64, test_case.get("TestStruct_21").unwrap().get("elem21_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_22, elem1) as u64, test_case.get("TestStruct_22").unwrap().get("elem1_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_22, elem2) as u64, test_case.get("TestStruct_22").unwrap().get("elem2_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_22, elem3) as u64, test_case.get("TestStruct_22").unwrap().get("elem3_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_22, elem4) as u64, test_case.get("TestStruct_22").unwrap().get("elem4_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_22, elem5) as u64, test_case.get("TestStruct_22").unwrap().get("elem5_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_22, elem6) as u64, test_case.get("TestStruct_22").unwrap().get("elem6_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_22, elem7) as u64, test_case.get("TestStruct_22").unwrap().get("elem7_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_22, elem8) as u64, test_case.get("TestStruct_22").unwrap().get("elem8_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_22, elem9) as u64, test_case.get("TestStruct_22").unwrap().get("elem9_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_22, elem10) as u64, test_case.get("TestStruct_22").unwrap().get("elem10_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_22, elem11) as u64, test_case.get("TestStruct_22").unwrap().get("elem11_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_22, elem12) as u64, test_case.get("TestStruct_22").unwrap().get("elem12_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_22, elem13) as u64, test_case.get("TestStruct_22").unwrap().get("elem13_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_22, elem14) as u64, test_case.get("TestStruct_22").unwrap().get("elem14_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_22, elem15) as u64, test_case.get("TestStruct_22").unwrap().get("elem15_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_22, elem16) as u64, test_case.get("TestStruct_22").unwrap().get("elem16_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_22, elem17) as u64, test_case.get("TestStruct_22").unwrap().get("elem17_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_22, elem18) as u64, test_case.get("TestStruct_22").unwrap().get("elem18_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_22, elem19) as u64, test_case.get("TestStruct_22").unwrap().get("elem19_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_22, elem20) as u64, test_case.get("TestStruct_22").unwrap().get("elem20_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_22, elem21) as u64, test_case.get("TestStruct_22").unwrap().get("elem21_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_22, elem22) as u64, test_case.get("TestStruct_22").unwrap().get("elem22_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_23, elem1) as u64, test_case.get("TestStruct_23").unwrap().get("elem1_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_23, elem2) as u64, test_case.get("TestStruct_23").unwrap().get("elem2_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_23, elem3) as u64, test_case.get("TestStruct_23").unwrap().get("elem3_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_23, elem4) as u64, test_case.get("TestStruct_23").unwrap().get("elem4_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_23, elem5) as u64, test_case.get("TestStruct_23").unwrap().get("elem5_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_23, elem6) as u64, test_case.get("TestStruct_23").unwrap().get("elem6_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_23, elem7) as u64, test_case.get("TestStruct_23").unwrap().get("elem7_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_23, elem8) as u64, test_case.get("TestStruct_23").unwrap().get("elem8_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_23, elem9) as u64, test_case.get("TestStruct_23").unwrap().get("elem9_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_23, elem10) as u64, test_case.get("TestStruct_23").unwrap().get("elem10_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_23, elem11) as u64, test_case.get("TestStruct_23").unwrap().get("elem11_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_23, elem12) as u64, test_case.get("TestStruct_23").unwrap().get("elem12_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_23, elem13) as u64, test_case.get("TestStruct_23").unwrap().get("elem13_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_23, elem14) as u64, test_case.get("TestStruct_23").unwrap().get("elem14_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_23, elem15) as u64, test_case.get("TestStruct_23").unwrap().get("elem15_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_23, elem16) as u64, test_case.get("TestStruct_23").unwrap().get("elem16_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_23, elem17) as u64, test_case.get("TestStruct_23").unwrap().get("elem17_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_23, elem18) as u64, test_case.get("TestStruct_23").unwrap().get("elem18_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_23, elem19) as u64, test_case.get("TestStruct_23").unwrap().get("elem19_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_23, elem20) as u64, test_case.get("TestStruct_23").unwrap().get("elem20_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_23, elem21) as u64, test_case.get("TestStruct_23").unwrap().get("elem21_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_23, elem22) as u64, test_case.get("TestStruct_23").unwrap().get("elem22_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_23, elem23) as u64, test_case.get("TestStruct_23").unwrap().get("elem23_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_24, elem1) as u64, test_case.get("TestStruct_24").unwrap().get("elem1_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_24, elem2) as u64, test_case.get("TestStruct_24").unwrap().get("elem2_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_24, elem3) as u64, test_case.get("TestStruct_24").unwrap().get("elem3_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_24, elem4) as u64, test_case.get("TestStruct_24").unwrap().get("elem4_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_24, elem5) as u64, test_case.get("TestStruct_24").unwrap().get("elem5_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_24, elem6) as u64, test_case.get("TestStruct_24").unwrap().get("elem6_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_24, elem7) as u64, test_case.get("TestStruct_24").unwrap().get("elem7_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_24, elem8) as u64, test_case.get("TestStruct_24").unwrap().get("elem8_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_24, elem9) as u64, test_case.get("TestStruct_24").unwrap().get("elem9_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_24, elem10) as u64, test_case.get("TestStruct_24").unwrap().get("elem10_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_24, elem11) as u64, test_case.get("TestStruct_24").unwrap().get("elem11_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_24, elem12) as u64, test_case.get("TestStruct_24").unwrap().get("elem12_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_24, elem13) as u64, test_case.get("TestStruct_24").unwrap().get("elem13_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_24, elem14) as u64, test_case.get("TestStruct_24").unwrap().get("elem14_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_24, elem15) as u64, test_case.get("TestStruct_24").unwrap().get("elem15_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_24, elem16) as u64, test_case.get("TestStruct_24").unwrap().get("elem16_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_24, elem17) as u64, test_case.get("TestStruct_24").unwrap().get("elem17_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_24, elem18) as u64, test_case.get("TestStruct_24").unwrap().get("elem18_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_24, elem19) as u64, test_case.get("TestStruct_24").unwrap().get("elem19_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_24, elem20) as u64, test_case.get("TestStruct_24").unwrap().get("elem20_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_24, elem21) as u64, test_case.get("TestStruct_24").unwrap().get("elem21_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_24, elem22) as u64, test_case.get("TestStruct_24").unwrap().get("elem22_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_24, elem23) as u64, test_case.get("TestStruct_24").unwrap().get("elem23_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_24, elem24) as u64, test_case.get("TestStruct_24").unwrap().get("elem24_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_25, elem1) as u64, test_case.get("TestStruct_25").unwrap().get("elem1_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_25, elem2) as u64, test_case.get("TestStruct_25").unwrap().get("elem2_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_25, elem3) as u64, test_case.get("TestStruct_25").unwrap().get("elem3_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_25, elem4) as u64, test_case.get("TestStruct_25").unwrap().get("elem4_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_25, elem5) as u64, test_case.get("TestStruct_25").unwrap().get("elem5_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_25, elem6) as u64, test_case.get("TestStruct_25").unwrap().get("elem6_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_25, elem7) as u64, test_case.get("TestStruct_25").unwrap().get("elem7_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_25, elem8) as u64, test_case.get("TestStruct_25").unwrap().get("elem8_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_25, elem9) as u64, test_case.get("TestStruct_25").unwrap().get("elem9_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_25, elem10) as u64, test_case.get("TestStruct_25").unwrap().get("elem10_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_25, elem11) as u64, test_case.get("TestStruct_25").unwrap().get("elem11_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_25, elem12) as u64, test_case.get("TestStruct_25").unwrap().get("elem12_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_25, elem13) as u64, test_case.get("TestStruct_25").unwrap().get("elem13_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_25, elem14) as u64, test_case.get("TestStruct_25").unwrap().get("elem14_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_25, elem15) as u64, test_case.get("TestStruct_25").unwrap().get("elem15_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_25, elem16) as u64, test_case.get("TestStruct_25").unwrap().get("elem16_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_25, elem17) as u64, test_case.get("TestStruct_25").unwrap().get("elem17_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_25, elem18) as u64, test_case.get("TestStruct_25").unwrap().get("elem18_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_25, elem19) as u64, test_case.get("TestStruct_25").unwrap().get("elem19_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_25, elem20) as u64, test_case.get("TestStruct_25").unwrap().get("elem20_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_25, elem21) as u64, test_case.get("TestStruct_25").unwrap().get("elem21_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_25, elem22) as u64, test_case.get("TestStruct_25").unwrap().get("elem22_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_25, elem23) as u64, test_case.get("TestStruct_25").unwrap().get("elem23_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_25, elem24) as u64, test_case.get("TestStruct_25").unwrap().get("elem24_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_25, elem25) as u64, test_case.get("TestStruct_25").unwrap().get("elem25_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_26, elem1) as u64, test_case.get("TestStruct_26").unwrap().get("elem1_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_26, elem2) as u64, test_case.get("TestStruct_26").unwrap().get("elem2_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_26, elem3) as u64, test_case.get("TestStruct_26").unwrap().get("elem3_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_26, elem4) as u64, test_case.get("TestStruct_26").unwrap().get("elem4_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_26, elem5) as u64, test_case.get("TestStruct_26").unwrap().get("elem5_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_26, elem6) as u64, test_case.get("TestStruct_26").unwrap().get("elem6_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_26, elem7) as u64, test_case.get("TestStruct_26").unwrap().get("elem7_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_26, elem8) as u64, test_case.get("TestStruct_26").unwrap().get("elem8_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_26, elem9) as u64, test_case.get("TestStruct_26").unwrap().get("elem9_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_26, elem10) as u64, test_case.get("TestStruct_26").unwrap().get("elem10_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_26, elem11) as u64, test_case.get("TestStruct_26").unwrap().get("elem11_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_26, elem12) as u64, test_case.get("TestStruct_26").unwrap().get("elem12_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_26, elem13) as u64, test_case.get("TestStruct_26").unwrap().get("elem13_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_26, elem14) as u64, test_case.get("TestStruct_26").unwrap().get("elem14_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_26, elem15) as u64, test_case.get("TestStruct_26").unwrap().get("elem15_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_26, elem16) as u64, test_case.get("TestStruct_26").unwrap().get("elem16_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_26, elem17) as u64, test_case.get("TestStruct_26").unwrap().get("elem17_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_26, elem18) as u64, test_case.get("TestStruct_26").unwrap().get("elem18_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_26, elem19) as u64, test_case.get("TestStruct_26").unwrap().get("elem19_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_26, elem20) as u64, test_case.get("TestStruct_26").unwrap().get("elem20_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_26, elem21) as u64, test_case.get("TestStruct_26").unwrap().get("elem21_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_26, elem22) as u64, test_case.get("TestStruct_26").unwrap().get("elem22_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_26, elem23) as u64, test_case.get("TestStruct_26").unwrap().get("elem23_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_26, elem24) as u64, test_case.get("TestStruct_26").unwrap().get("elem24_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_26, elem25) as u64, test_case.get("TestStruct_26").unwrap().get("elem25_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_26, elem26) as u64, test_case.get("TestStruct_26").unwrap().get("elem26_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_27, elem1) as u64, test_case.get("TestStruct_27").unwrap().get("elem1_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_27, elem2) as u64, test_case.get("TestStruct_27").unwrap().get("elem2_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_27, elem3) as u64, test_case.get("TestStruct_27").unwrap().get("elem3_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_27, elem4) as u64, test_case.get("TestStruct_27").unwrap().get("elem4_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_27, elem5) as u64, test_case.get("TestStruct_27").unwrap().get("elem5_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_27, elem6) as u64, test_case.get("TestStruct_27").unwrap().get("elem6_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_27, elem7) as u64, test_case.get("TestStruct_27").unwrap().get("elem7_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_27, elem8) as u64, test_case.get("TestStruct_27").unwrap().get("elem8_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_27, elem9) as u64, test_case.get("TestStruct_27").unwrap().get("elem9_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_27, elem10) as u64, test_case.get("TestStruct_27").unwrap().get("elem10_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_27, elem11) as u64, test_case.get("TestStruct_27").unwrap().get("elem11_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_27, elem12) as u64, test_case.get("TestStruct_27").unwrap().get("elem12_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_27, elem13) as u64, test_case.get("TestStruct_27").unwrap().get("elem13_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_27, elem14) as u64, test_case.get("TestStruct_27").unwrap().get("elem14_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_27, elem15) as u64, test_case.get("TestStruct_27").unwrap().get("elem15_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_27, elem16) as u64, test_case.get("TestStruct_27").unwrap().get("elem16_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_27, elem17) as u64, test_case.get("TestStruct_27").unwrap().get("elem17_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_27, elem18) as u64, test_case.get("TestStruct_27").unwrap().get("elem18_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_27, elem19) as u64, test_case.get("TestStruct_27").unwrap().get("elem19_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_27, elem20) as u64, test_case.get("TestStruct_27").unwrap().get("elem20_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_27, elem21) as u64, test_case.get("TestStruct_27").unwrap().get("elem21_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_27, elem22) as u64, test_case.get("TestStruct_27").unwrap().get("elem22_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_27, elem23) as u64, test_case.get("TestStruct_27").unwrap().get("elem23_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_27, elem24) as u64, test_case.get("TestStruct_27").unwrap().get("elem24_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_27, elem25) as u64, test_case.get("TestStruct_27").unwrap().get("elem25_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_27, elem26) as u64, test_case.get("TestStruct_27").unwrap().get("elem26_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_27, elem27) as u64, test_case.get("TestStruct_27").unwrap().get("elem27_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_28, elem1) as u64, test_case.get("TestStruct_28").unwrap().get("elem1_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_28, elem2) as u64, test_case.get("TestStruct_28").unwrap().get("elem2_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_28, elem3) as u64, test_case.get("TestStruct_28").unwrap().get("elem3_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_28, elem4) as u64, test_case.get("TestStruct_28").unwrap().get("elem4_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_28, elem5) as u64, test_case.get("TestStruct_28").unwrap().get("elem5_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_28, elem6) as u64, test_case.get("TestStruct_28").unwrap().get("elem6_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_28, elem7) as u64, test_case.get("TestStruct_28").unwrap().get("elem7_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_28, elem8) as u64, test_case.get("TestStruct_28").unwrap().get("elem8_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_28, elem9) as u64, test_case.get("TestStruct_28").unwrap().get("elem9_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_28, elem10) as u64, test_case.get("TestStruct_28").unwrap().get("elem10_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_28, elem11) as u64, test_case.get("TestStruct_28").unwrap().get("elem11_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_28, elem12) as u64, test_case.get("TestStruct_28").unwrap().get("elem12_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_28, elem13) as u64, test_case.get("TestStruct_28").unwrap().get("elem13_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_28, elem14) as u64, test_case.get("TestStruct_28").unwrap().get("elem14_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_28, elem15) as u64, test_case.get("TestStruct_28").unwrap().get("elem15_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_28, elem16) as u64, test_case.get("TestStruct_28").unwrap().get("elem16_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_28, elem17) as u64, test_case.get("TestStruct_28").unwrap().get("elem17_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_28, elem18) as u64, test_case.get("TestStruct_28").unwrap().get("elem18_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_28, elem19) as u64, test_case.get("TestStruct_28").unwrap().get("elem19_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_28, elem20) as u64, test_case.get("TestStruct_28").unwrap().get("elem20_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_28, elem21) as u64, test_case.get("TestStruct_28").unwrap().get("elem21_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_28, elem22) as u64, test_case.get("TestStruct_28").unwrap().get("elem22_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_28, elem23) as u64, test_case.get("TestStruct_28").unwrap().get("elem23_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_28, elem24) as u64, test_case.get("TestStruct_28").unwrap().get("elem24_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_28, elem25) as u64, test_case.get("TestStruct_28").unwrap().get("elem25_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_28, elem26) as u64, test_case.get("TestStruct_28").unwrap().get("elem26_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_28, elem27) as u64, test_case.get("TestStruct_28").unwrap().get("elem27_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_28, elem28) as u64, test_case.get("TestStruct_28").unwrap().get("elem28_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_29, elem1) as u64, test_case.get("TestStruct_29").unwrap().get("elem1_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_29, elem2) as u64, test_case.get("TestStruct_29").unwrap().get("elem2_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_29, elem3) as u64, test_case.get("TestStruct_29").unwrap().get("elem3_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_29, elem4) as u64, test_case.get("TestStruct_29").unwrap().get("elem4_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_29, elem5) as u64, test_case.get("TestStruct_29").unwrap().get("elem5_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_29, elem6) as u64, test_case.get("TestStruct_29").unwrap().get("elem6_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_29, elem7) as u64, test_case.get("TestStruct_29").unwrap().get("elem7_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_29, elem8) as u64, test_case.get("TestStruct_29").unwrap().get("elem8_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_29, elem9) as u64, test_case.get("TestStruct_29").unwrap().get("elem9_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_29, elem10) as u64, test_case.get("TestStruct_29").unwrap().get("elem10_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_29, elem11) as u64, test_case.get("TestStruct_29").unwrap().get("elem11_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_29, elem12) as u64, test_case.get("TestStruct_29").unwrap().get("elem12_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_29, elem13) as u64, test_case.get("TestStruct_29").unwrap().get("elem13_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_29, elem14) as u64, test_case.get("TestStruct_29").unwrap().get("elem14_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_29, elem15) as u64, test_case.get("TestStruct_29").unwrap().get("elem15_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_29, elem16) as u64, test_case.get("TestStruct_29").unwrap().get("elem16_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_29, elem17) as u64, test_case.get("TestStruct_29").unwrap().get("elem17_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_29, elem18) as u64, test_case.get("TestStruct_29").unwrap().get("elem18_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_29, elem19) as u64, test_case.get("TestStruct_29").unwrap().get("elem19_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_29, elem20) as u64, test_case.get("TestStruct_29").unwrap().get("elem20_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_29, elem21) as u64, test_case.get("TestStruct_29").unwrap().get("elem21_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_29, elem22) as u64, test_case.get("TestStruct_29").unwrap().get("elem22_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_29, elem23) as u64, test_case.get("TestStruct_29").unwrap().get("elem23_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_29, elem24) as u64, test_case.get("TestStruct_29").unwrap().get("elem24_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_29, elem25) as u64, test_case.get("TestStruct_29").unwrap().get("elem25_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_29, elem26) as u64, test_case.get("TestStruct_29").unwrap().get("elem26_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_29, elem27) as u64, test_case.get("TestStruct_29").unwrap().get("elem27_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_29, elem28) as u64, test_case.get("TestStruct_29").unwrap().get("elem28_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_29, elem29) as u64, test_case.get("TestStruct_29").unwrap().get("elem29_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_30, elem1) as u64, test_case.get("TestStruct_30").unwrap().get("elem1_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_30, elem2) as u64, test_case.get("TestStruct_30").unwrap().get("elem2_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_30, elem3) as u64, test_case.get("TestStruct_30").unwrap().get("elem3_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_30, elem4) as u64, test_case.get("TestStruct_30").unwrap().get("elem4_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_30, elem5) as u64, test_case.get("TestStruct_30").unwrap().get("elem5_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_30, elem6) as u64, test_case.get("TestStruct_30").unwrap().get("elem6_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_30, elem7) as u64, test_case.get("TestStruct_30").unwrap().get("elem7_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_30, elem8) as u64, test_case.get("TestStruct_30").unwrap().get("elem8_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_30, elem9) as u64, test_case.get("TestStruct_30").unwrap().get("elem9_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_30, elem10) as u64, test_case.get("TestStruct_30").unwrap().get("elem10_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_30, elem11) as u64, test_case.get("TestStruct_30").unwrap().get("elem11_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_30, elem12) as u64, test_case.get("TestStruct_30").unwrap().get("elem12_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_30, elem13) as u64, test_case.get("TestStruct_30").unwrap().get("elem13_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_30, elem14) as u64, test_case.get("TestStruct_30").unwrap().get("elem14_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_30, elem15) as u64, test_case.get("TestStruct_30").unwrap().get("elem15_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_30, elem16) as u64, test_case.get("TestStruct_30").unwrap().get("elem16_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_30, elem17) as u64, test_case.get("TestStruct_30").unwrap().get("elem17_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_30, elem18) as u64, test_case.get("TestStruct_30").unwrap().get("elem18_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_30, elem19) as u64, test_case.get("TestStruct_30").unwrap().get("elem19_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_30, elem20) as u64, test_case.get("TestStruct_30").unwrap().get("elem20_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_30, elem21) as u64, test_case.get("TestStruct_30").unwrap().get("elem21_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_30, elem22) as u64, test_case.get("TestStruct_30").unwrap().get("elem22_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_30, elem23) as u64, test_case.get("TestStruct_30").unwrap().get("elem23_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_30, elem24) as u64, test_case.get("TestStruct_30").unwrap().get("elem24_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_30, elem25) as u64, test_case.get("TestStruct_30").unwrap().get("elem25_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_30, elem26) as u64, test_case.get("TestStruct_30").unwrap().get("elem26_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_30, elem27) as u64, test_case.get("TestStruct_30").unwrap().get("elem27_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_30, elem28) as u64, test_case.get("TestStruct_30").unwrap().get("elem28_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_30, elem29) as u64, test_case.get("TestStruct_30").unwrap().get("elem29_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_30, elem30) as u64, test_case.get("TestStruct_30").unwrap().get("elem30_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_31, elem1) as u64, test_case.get("TestStruct_31").unwrap().get("elem1_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_31, elem2) as u64, test_case.get("TestStruct_31").unwrap().get("elem2_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_31, elem3) as u64, test_case.get("TestStruct_31").unwrap().get("elem3_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_31, elem4) as u64, test_case.get("TestStruct_31").unwrap().get("elem4_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_31, elem5) as u64, test_case.get("TestStruct_31").unwrap().get("elem5_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_31, elem6) as u64, test_case.get("TestStruct_31").unwrap().get("elem6_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_31, elem7) as u64, test_case.get("TestStruct_31").unwrap().get("elem7_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_31, elem8) as u64, test_case.get("TestStruct_31").unwrap().get("elem8_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_31, elem9) as u64, test_case.get("TestStruct_31").unwrap().get("elem9_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_31, elem10) as u64, test_case.get("TestStruct_31").unwrap().get("elem10_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_31, elem11) as u64, test_case.get("TestStruct_31").unwrap().get("elem11_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_31, elem12) as u64, test_case.get("TestStruct_31").unwrap().get("elem12_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_31, elem13) as u64, test_case.get("TestStruct_31").unwrap().get("elem13_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_31, elem14) as u64, test_case.get("TestStruct_31").unwrap().get("elem14_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_31, elem15) as u64, test_case.get("TestStruct_31").unwrap().get("elem15_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_31, elem16) as u64, test_case.get("TestStruct_31").unwrap().get("elem16_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_31, elem17) as u64, test_case.get("TestStruct_31").unwrap().get("elem17_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_31, elem18) as u64, test_case.get("TestStruct_31").unwrap().get("elem18_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_31, elem19) as u64, test_case.get("TestStruct_31").unwrap().get("elem19_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_31, elem20) as u64, test_case.get("TestStruct_31").unwrap().get("elem20_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_31, elem21) as u64, test_case.get("TestStruct_31").unwrap().get("elem21_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_31, elem22) as u64, test_case.get("TestStruct_31").unwrap().get("elem22_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_31, elem23) as u64, test_case.get("TestStruct_31").unwrap().get("elem23_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_31, elem24) as u64, test_case.get("TestStruct_31").unwrap().get("elem24_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_31, elem25) as u64, test_case.get("TestStruct_31").unwrap().get("elem25_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_31, elem26) as u64, test_case.get("TestStruct_31").unwrap().get("elem26_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_31, elem27) as u64, test_case.get("TestStruct_31").unwrap().get("elem27_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_31, elem28) as u64, test_case.get("TestStruct_31").unwrap().get("elem28_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_31, elem29) as u64, test_case.get("TestStruct_31").unwrap().get("elem29_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_31, elem30) as u64, test_case.get("TestStruct_31").unwrap().get("elem30_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_31, elem31) as u64, test_case.get("TestStruct_31").unwrap().get("elem31_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_32, elem1) as u64, test_case.get("TestStruct_32").unwrap().get("elem1_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_32, elem2) as u64, test_case.get("TestStruct_32").unwrap().get("elem2_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_32, elem3) as u64, test_case.get("TestStruct_32").unwrap().get("elem3_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_32, elem4) as u64, test_case.get("TestStruct_32").unwrap().get("elem4_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_32, elem5) as u64, test_case.get("TestStruct_32").unwrap().get("elem5_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_32, elem6) as u64, test_case.get("TestStruct_32").unwrap().get("elem6_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_32, elem7) as u64, test_case.get("TestStruct_32").unwrap().get("elem7_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_32, elem8) as u64, test_case.get("TestStruct_32").unwrap().get("elem8_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_32, elem9) as u64, test_case.get("TestStruct_32").unwrap().get("elem9_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_32, elem10) as u64, test_case.get("TestStruct_32").unwrap().get("elem10_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_32, elem11) as u64, test_case.get("TestStruct_32").unwrap().get("elem11_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_32, elem12) as u64, test_case.get("TestStruct_32").unwrap().get("elem12_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_32, elem13) as u64, test_case.get("TestStruct_32").unwrap().get("elem13_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_32, elem14) as u64, test_case.get("TestStruct_32").unwrap().get("elem14_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_32, elem15) as u64, test_case.get("TestStruct_32").unwrap().get("elem15_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_32, elem16) as u64, test_case.get("TestStruct_32").unwrap().get("elem16_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_32, elem17) as u64, test_case.get("TestStruct_32").unwrap().get("elem17_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_32, elem18) as u64, test_case.get("TestStruct_32").unwrap().get("elem18_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_32, elem19) as u64, test_case.get("TestStruct_32").unwrap().get("elem19_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_32, elem20) as u64, test_case.get("TestStruct_32").unwrap().get("elem20_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_32, elem21) as u64, test_case.get("TestStruct_32").unwrap().get("elem21_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_32, elem22) as u64, test_case.get("TestStruct_32").unwrap().get("elem22_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_32, elem23) as u64, test_case.get("TestStruct_32").unwrap().get("elem23_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_32, elem24) as u64, test_case.get("TestStruct_32").unwrap().get("elem24_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_32, elem25) as u64, test_case.get("TestStruct_32").unwrap().get("elem25_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_32, elem26) as u64, test_case.get("TestStruct_32").unwrap().get("elem26_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_32, elem27) as u64, test_case.get("TestStruct_32").unwrap().get("elem27_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_32, elem28) as u64, test_case.get("TestStruct_32").unwrap().get("elem28_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_32, elem29) as u64, test_case.get("TestStruct_32").unwrap().get("elem29_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_32, elem30) as u64, test_case.get("TestStruct_32").unwrap().get("elem30_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_32, elem31) as u64, test_case.get("TestStruct_32").unwrap().get("elem31_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_32, elem32) as u64, test_case.get("TestStruct_32").unwrap().get("elem32_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_33, elem1) as u64, test_case.get("TestStruct_33").unwrap().get("elem1_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_33, elem2) as u64, test_case.get("TestStruct_33").unwrap().get("elem2_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_33, elem3) as u64, test_case.get("TestStruct_33").unwrap().get("elem3_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_33, elem4) as u64, test_case.get("TestStruct_33").unwrap().get("elem4_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_33, elem5) as u64, test_case.get("TestStruct_33").unwrap().get("elem5_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_33, elem6) as u64, test_case.get("TestStruct_33").unwrap().get("elem6_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_33, elem7) as u64, test_case.get("TestStruct_33").unwrap().get("elem7_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_33, elem8) as u64, test_case.get("TestStruct_33").unwrap().get("elem8_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_33, elem9) as u64, test_case.get("TestStruct_33").unwrap().get("elem9_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_33, elem10) as u64, test_case.get("TestStruct_33").unwrap().get("elem10_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_33, elem11) as u64, test_case.get("TestStruct_33").unwrap().get("elem11_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_33, elem12) as u64, test_case.get("TestStruct_33").unwrap().get("elem12_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_33, elem13) as u64, test_case.get("TestStruct_33").unwrap().get("elem13_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_33, elem14) as u64, test_case.get("TestStruct_33").unwrap().get("elem14_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_33, elem15) as u64, test_case.get("TestStruct_33").unwrap().get("elem15_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_33, elem16) as u64, test_case.get("TestStruct_33").unwrap().get("elem16_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_33, elem17) as u64, test_case.get("TestStruct_33").unwrap().get("elem17_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_33, elem18) as u64, test_case.get("TestStruct_33").unwrap().get("elem18_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_33, elem19) as u64, test_case.get("TestStruct_33").unwrap().get("elem19_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_33, elem20) as u64, test_case.get("TestStruct_33").unwrap().get("elem20_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_33, elem21) as u64, test_case.get("TestStruct_33").unwrap().get("elem21_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_33, elem22) as u64, test_case.get("TestStruct_33").unwrap().get("elem22_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_33, elem23) as u64, test_case.get("TestStruct_33").unwrap().get("elem23_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_33, elem24) as u64, test_case.get("TestStruct_33").unwrap().get("elem24_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_33, elem25) as u64, test_case.get("TestStruct_33").unwrap().get("elem25_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_33, elem26) as u64, test_case.get("TestStruct_33").unwrap().get("elem26_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_33, elem27) as u64, test_case.get("TestStruct_33").unwrap().get("elem27_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_33, elem28) as u64, test_case.get("TestStruct_33").unwrap().get("elem28_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_33, elem29) as u64, test_case.get("TestStruct_33").unwrap().get("elem29_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_33, elem30) as u64, test_case.get("TestStruct_33").unwrap().get("elem30_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_33, elem31) as u64, test_case.get("TestStruct_33").unwrap().get("elem31_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_33, elem32) as u64, test_case.get("TestStruct_33").unwrap().get("elem32_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_33, elem33) as u64, test_case.get("TestStruct_33").unwrap().get("elem33_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_34, elem1) as u64, test_case.get("TestStruct_34").unwrap().get("elem1_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_34, elem2) as u64, test_case.get("TestStruct_34").unwrap().get("elem2_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_34, elem3) as u64, test_case.get("TestStruct_34").unwrap().get("elem3_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_34, elem4) as u64, test_case.get("TestStruct_34").unwrap().get("elem4_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_34, elem5) as u64, test_case.get("TestStruct_34").unwrap().get("elem5_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_34, elem6) as u64, test_case.get("TestStruct_34").unwrap().get("elem6_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_34, elem7) as u64, test_case.get("TestStruct_34").unwrap().get("elem7_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_34, elem8) as u64, test_case.get("TestStruct_34").unwrap().get("elem8_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_34, elem9) as u64, test_case.get("TestStruct_34").unwrap().get("elem9_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_34, elem10) as u64, test_case.get("TestStruct_34").unwrap().get("elem10_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_34, elem11) as u64, test_case.get("TestStruct_34").unwrap().get("elem11_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_34, elem12) as u64, test_case.get("TestStruct_34").unwrap().get("elem12_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_34, elem13) as u64, test_case.get("TestStruct_34").unwrap().get("elem13_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_34, elem14) as u64, test_case.get("TestStruct_34").unwrap().get("elem14_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_34, elem15) as u64, test_case.get("TestStruct_34").unwrap().get("elem15_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_34, elem16) as u64, test_case.get("TestStruct_34").unwrap().get("elem16_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_34, elem17) as u64, test_case.get("TestStruct_34").unwrap().get("elem17_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_34, elem18) as u64, test_case.get("TestStruct_34").unwrap().get("elem18_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_34, elem19) as u64, test_case.get("TestStruct_34").unwrap().get("elem19_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_34, elem20) as u64, test_case.get("TestStruct_34").unwrap().get("elem20_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_34, elem21) as u64, test_case.get("TestStruct_34").unwrap().get("elem21_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_34, elem22) as u64, test_case.get("TestStruct_34").unwrap().get("elem22_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_34, elem23) as u64, test_case.get("TestStruct_34").unwrap().get("elem23_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_34, elem24) as u64, test_case.get("TestStruct_34").unwrap().get("elem24_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_34, elem25) as u64, test_case.get("TestStruct_34").unwrap().get("elem25_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_34, elem26) as u64, test_case.get("TestStruct_34").unwrap().get("elem26_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_34, elem27) as u64, test_case.get("TestStruct_34").unwrap().get("elem27_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_34, elem28) as u64, test_case.get("TestStruct_34").unwrap().get("elem28_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_34, elem29) as u64, test_case.get("TestStruct_34").unwrap().get("elem29_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_34, elem30) as u64, test_case.get("TestStruct_34").unwrap().get("elem30_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_34, elem31) as u64, test_case.get("TestStruct_34").unwrap().get("elem31_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_34, elem32) as u64, test_case.get("TestStruct_34").unwrap().get("elem32_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_34, elem33) as u64, test_case.get("TestStruct_34").unwrap().get("elem33_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_34, elem34) as u64, test_case.get("TestStruct_34").unwrap().get("elem34_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_35, elem1) as u64, test_case.get("TestStruct_35").unwrap().get("elem1_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_35, elem2) as u64, test_case.get("TestStruct_35").unwrap().get("elem2_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_35, elem3) as u64, test_case.get("TestStruct_35").unwrap().get("elem3_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_35, elem4) as u64, test_case.get("TestStruct_35").unwrap().get("elem4_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_35, elem5) as u64, test_case.get("TestStruct_35").unwrap().get("elem5_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_35, elem6) as u64, test_case.get("TestStruct_35").unwrap().get("elem6_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_35, elem7) as u64, test_case.get("TestStruct_35").unwrap().get("elem7_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_35, elem8) as u64, test_case.get("TestStruct_35").unwrap().get("elem8_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_35, elem9) as u64, test_case.get("TestStruct_35").unwrap().get("elem9_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_35, elem10) as u64, test_case.get("TestStruct_35").unwrap().get("elem10_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_35, elem11) as u64, test_case.get("TestStruct_35").unwrap().get("elem11_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_35, elem12) as u64, test_case.get("TestStruct_35").unwrap().get("elem12_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_35, elem13) as u64, test_case.get("TestStruct_35").unwrap().get("elem13_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_35, elem14) as u64, test_case.get("TestStruct_35").unwrap().get("elem14_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_35, elem15) as u64, test_case.get("TestStruct_35").unwrap().get("elem15_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_35, elem16) as u64, test_case.get("TestStruct_35").unwrap().get("elem16_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_35, elem17) as u64, test_case.get("TestStruct_35").unwrap().get("elem17_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_35, elem18) as u64, test_case.get("TestStruct_35").unwrap().get("elem18_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_35, elem19) as u64, test_case.get("TestStruct_35").unwrap().get("elem19_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_35, elem20) as u64, test_case.get("TestStruct_35").unwrap().get("elem20_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_35, elem21) as u64, test_case.get("TestStruct_35").unwrap().get("elem21_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_35, elem22) as u64, test_case.get("TestStruct_35").unwrap().get("elem22_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_35, elem23) as u64, test_case.get("TestStruct_35").unwrap().get("elem23_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_35, elem24) as u64, test_case.get("TestStruct_35").unwrap().get("elem24_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_35, elem25) as u64, test_case.get("TestStruct_35").unwrap().get("elem25_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_35, elem26) as u64, test_case.get("TestStruct_35").unwrap().get("elem26_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_35, elem27) as u64, test_case.get("TestStruct_35").unwrap().get("elem27_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_35, elem28) as u64, test_case.get("TestStruct_35").unwrap().get("elem28_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_35, elem29) as u64, test_case.get("TestStruct_35").unwrap().get("elem29_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_35, elem30) as u64, test_case.get("TestStruct_35").unwrap().get("elem30_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_35, elem31) as u64, test_case.get("TestStruct_35").unwrap().get("elem31_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_35, elem32) as u64, test_case.get("TestStruct_35").unwrap().get("elem32_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_35, elem33) as u64, test_case.get("TestStruct_35").unwrap().get("elem33_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_35, elem34) as u64, test_case.get("TestStruct_35").unwrap().get("elem34_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_35, elem35) as u64, test_case.get("TestStruct_35").unwrap().get("elem35_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_36, elem1) as u64, test_case.get("TestStruct_36").unwrap().get("elem1_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_36, elem2) as u64, test_case.get("TestStruct_36").unwrap().get("elem2_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_36, elem3) as u64, test_case.get("TestStruct_36").unwrap().get("elem3_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_36, elem4) as u64, test_case.get("TestStruct_36").unwrap().get("elem4_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_36, elem5) as u64, test_case.get("TestStruct_36").unwrap().get("elem5_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_36, elem6) as u64, test_case.get("TestStruct_36").unwrap().get("elem6_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_36, elem7) as u64, test_case.get("TestStruct_36").unwrap().get("elem7_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_36, elem8) as u64, test_case.get("TestStruct_36").unwrap().get("elem8_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_36, elem9) as u64, test_case.get("TestStruct_36").unwrap().get("elem9_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_36, elem10) as u64, test_case.get("TestStruct_36").unwrap().get("elem10_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_36, elem11) as u64, test_case.get("TestStruct_36").unwrap().get("elem11_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_36, elem12) as u64, test_case.get("TestStruct_36").unwrap().get("elem12_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_36, elem13) as u64, test_case.get("TestStruct_36").unwrap().get("elem13_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_36, elem14) as u64, test_case.get("TestStruct_36").unwrap().get("elem14_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_36, elem15) as u64, test_case.get("TestStruct_36").unwrap().get("elem15_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_36, elem16) as u64, test_case.get("TestStruct_36").unwrap().get("elem16_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_36, elem17) as u64, test_case.get("TestStruct_36").unwrap().get("elem17_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_36, elem18) as u64, test_case.get("TestStruct_36").unwrap().get("elem18_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_36, elem19) as u64, test_case.get("TestStruct_36").unwrap().get("elem19_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_36, elem20) as u64, test_case.get("TestStruct_36").unwrap().get("elem20_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_36, elem21) as u64, test_case.get("TestStruct_36").unwrap().get("elem21_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_36, elem22) as u64, test_case.get("TestStruct_36").unwrap().get("elem22_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_36, elem23) as u64, test_case.get("TestStruct_36").unwrap().get("elem23_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_36, elem24) as u64, test_case.get("TestStruct_36").unwrap().get("elem24_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_36, elem25) as u64, test_case.get("TestStruct_36").unwrap().get("elem25_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_36, elem26) as u64, test_case.get("TestStruct_36").unwrap().get("elem26_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_36, elem27) as u64, test_case.get("TestStruct_36").unwrap().get("elem27_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_36, elem28) as u64, test_case.get("TestStruct_36").unwrap().get("elem28_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_36, elem29) as u64, test_case.get("TestStruct_36").unwrap().get("elem29_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_36, elem30) as u64, test_case.get("TestStruct_36").unwrap().get("elem30_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_36, elem31) as u64, test_case.get("TestStruct_36").unwrap().get("elem31_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_36, elem32) as u64, test_case.get("TestStruct_36").unwrap().get("elem32_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_36, elem33) as u64, test_case.get("TestStruct_36").unwrap().get("elem33_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_36, elem34) as u64, test_case.get("TestStruct_36").unwrap().get("elem34_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_36, elem35) as u64, test_case.get("TestStruct_36").unwrap().get("elem35_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_36, elem36) as u64, test_case.get("TestStruct_36").unwrap().get("elem36_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_37, elem1) as u64, test_case.get("TestStruct_37").unwrap().get("elem1_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_37, elem2) as u64, test_case.get("TestStruct_37").unwrap().get("elem2_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_37, elem3) as u64, test_case.get("TestStruct_37").unwrap().get("elem3_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_37, elem4) as u64, test_case.get("TestStruct_37").unwrap().get("elem4_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_37, elem5) as u64, test_case.get("TestStruct_37").unwrap().get("elem5_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_37, elem6) as u64, test_case.get("TestStruct_37").unwrap().get("elem6_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_37, elem7) as u64, test_case.get("TestStruct_37").unwrap().get("elem7_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_37, elem8) as u64, test_case.get("TestStruct_37").unwrap().get("elem8_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_37, elem9) as u64, test_case.get("TestStruct_37").unwrap().get("elem9_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_37, elem10) as u64, test_case.get("TestStruct_37").unwrap().get("elem10_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_37, elem11) as u64, test_case.get("TestStruct_37").unwrap().get("elem11_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_37, elem12) as u64, test_case.get("TestStruct_37").unwrap().get("elem12_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_37, elem13) as u64, test_case.get("TestStruct_37").unwrap().get("elem13_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_37, elem14) as u64, test_case.get("TestStruct_37").unwrap().get("elem14_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_37, elem15) as u64, test_case.get("TestStruct_37").unwrap().get("elem15_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_37, elem16) as u64, test_case.get("TestStruct_37").unwrap().get("elem16_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_37, elem17) as u64, test_case.get("TestStruct_37").unwrap().get("elem17_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_37, elem18) as u64, test_case.get("TestStruct_37").unwrap().get("elem18_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_37, elem19) as u64, test_case.get("TestStruct_37").unwrap().get("elem19_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_37, elem20) as u64, test_case.get("TestStruct_37").unwrap().get("elem20_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_37, elem21) as u64, test_case.get("TestStruct_37").unwrap().get("elem21_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_37, elem22) as u64, test_case.get("TestStruct_37").unwrap().get("elem22_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_37, elem23) as u64, test_case.get("TestStruct_37").unwrap().get("elem23_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_37, elem24) as u64, test_case.get("TestStruct_37").unwrap().get("elem24_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_37, elem25) as u64, test_case.get("TestStruct_37").unwrap().get("elem25_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_37, elem26) as u64, test_case.get("TestStruct_37").unwrap().get("elem26_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_37, elem27) as u64, test_case.get("TestStruct_37").unwrap().get("elem27_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_37, elem28) as u64, test_case.get("TestStruct_37").unwrap().get("elem28_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_37, elem29) as u64, test_case.get("TestStruct_37").unwrap().get("elem29_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_37, elem30) as u64, test_case.get("TestStruct_37").unwrap().get("elem30_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_37, elem31) as u64, test_case.get("TestStruct_37").unwrap().get("elem31_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_37, elem32) as u64, test_case.get("TestStruct_37").unwrap().get("elem32_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_37, elem33) as u64, test_case.get("TestStruct_37").unwrap().get("elem33_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_37, elem34) as u64, test_case.get("TestStruct_37").unwrap().get("elem34_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_37, elem35) as u64, test_case.get("TestStruct_37").unwrap().get("elem35_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_37, elem36) as u64, test_case.get("TestStruct_37").unwrap().get("elem36_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_37, elem37) as u64, test_case.get("TestStruct_37").unwrap().get("elem37_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_38, elem1) as u64, test_case.get("TestStruct_38").unwrap().get("elem1_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_38, elem2) as u64, test_case.get("TestStruct_38").unwrap().get("elem2_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_38, elem3) as u64, test_case.get("TestStruct_38").unwrap().get("elem3_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_38, elem4) as u64, test_case.get("TestStruct_38").unwrap().get("elem4_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_38, elem5) as u64, test_case.get("TestStruct_38").unwrap().get("elem5_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_38, elem6) as u64, test_case.get("TestStruct_38").unwrap().get("elem6_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_38, elem7) as u64, test_case.get("TestStruct_38").unwrap().get("elem7_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_38, elem8) as u64, test_case.get("TestStruct_38").unwrap().get("elem8_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_38, elem9) as u64, test_case.get("TestStruct_38").unwrap().get("elem9_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_38, elem10) as u64, test_case.get("TestStruct_38").unwrap().get("elem10_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_38, elem11) as u64, test_case.get("TestStruct_38").unwrap().get("elem11_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_38, elem12) as u64, test_case.get("TestStruct_38").unwrap().get("elem12_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_38, elem13) as u64, test_case.get("TestStruct_38").unwrap().get("elem13_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_38, elem14) as u64, test_case.get("TestStruct_38").unwrap().get("elem14_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_38, elem15) as u64, test_case.get("TestStruct_38").unwrap().get("elem15_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_38, elem16) as u64, test_case.get("TestStruct_38").unwrap().get("elem16_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_38, elem17) as u64, test_case.get("TestStruct_38").unwrap().get("elem17_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_38, elem18) as u64, test_case.get("TestStruct_38").unwrap().get("elem18_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_38, elem19) as u64, test_case.get("TestStruct_38").unwrap().get("elem19_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_38, elem20) as u64, test_case.get("TestStruct_38").unwrap().get("elem20_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_38, elem21) as u64, test_case.get("TestStruct_38").unwrap().get("elem21_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_38, elem22) as u64, test_case.get("TestStruct_38").unwrap().get("elem22_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_38, elem23) as u64, test_case.get("TestStruct_38").unwrap().get("elem23_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_38, elem24) as u64, test_case.get("TestStruct_38").unwrap().get("elem24_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_38, elem25) as u64, test_case.get("TestStruct_38").unwrap().get("elem25_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_38, elem26) as u64, test_case.get("TestStruct_38").unwrap().get("elem26_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_38, elem27) as u64, test_case.get("TestStruct_38").unwrap().get("elem27_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_38, elem28) as u64, test_case.get("TestStruct_38").unwrap().get("elem28_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_38, elem29) as u64, test_case.get("TestStruct_38").unwrap().get("elem29_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_38, elem30) as u64, test_case.get("TestStruct_38").unwrap().get("elem30_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_38, elem31) as u64, test_case.get("TestStruct_38").unwrap().get("elem31_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_38, elem32) as u64, test_case.get("TestStruct_38").unwrap().get("elem32_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_38, elem33) as u64, test_case.get("TestStruct_38").unwrap().get("elem33_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_38, elem34) as u64, test_case.get("TestStruct_38").unwrap().get("elem34_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_38, elem35) as u64, test_case.get("TestStruct_38").unwrap().get("elem35_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_38, elem36) as u64, test_case.get("TestStruct_38").unwrap().get("elem36_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_38, elem37) as u64, test_case.get("TestStruct_38").unwrap().get("elem37_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_38, elem38) as u64, test_case.get("TestStruct_38").unwrap().get("elem38_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_39, elem1) as u64, test_case.get("TestStruct_39").unwrap().get("elem1_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_39, elem2) as u64, test_case.get("TestStruct_39").unwrap().get("elem2_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_39, elem3) as u64, test_case.get("TestStruct_39").unwrap().get("elem3_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_39, elem4) as u64, test_case.get("TestStruct_39").unwrap().get("elem4_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_39, elem5) as u64, test_case.get("TestStruct_39").unwrap().get("elem5_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_39, elem6) as u64, test_case.get("TestStruct_39").unwrap().get("elem6_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_39, elem7) as u64, test_case.get("TestStruct_39").unwrap().get("elem7_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_39, elem8) as u64, test_case.get("TestStruct_39").unwrap().get("elem8_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_39, elem9) as u64, test_case.get("TestStruct_39").unwrap().get("elem9_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_39, elem10) as u64, test_case.get("TestStruct_39").unwrap().get("elem10_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_39, elem11) as u64, test_case.get("TestStruct_39").unwrap().get("elem11_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_39, elem12) as u64, test_case.get("TestStruct_39").unwrap().get("elem12_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_39, elem13) as u64, test_case.get("TestStruct_39").unwrap().get("elem13_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_39, elem14) as u64, test_case.get("TestStruct_39").unwrap().get("elem14_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_39, elem15) as u64, test_case.get("TestStruct_39").unwrap().get("elem15_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_39, elem16) as u64, test_case.get("TestStruct_39").unwrap().get("elem16_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_39, elem17) as u64, test_case.get("TestStruct_39").unwrap().get("elem17_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_39, elem18) as u64, test_case.get("TestStruct_39").unwrap().get("elem18_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_39, elem19) as u64, test_case.get("TestStruct_39").unwrap().get("elem19_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_39, elem20) as u64, test_case.get("TestStruct_39").unwrap().get("elem20_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_39, elem21) as u64, test_case.get("TestStruct_39").unwrap().get("elem21_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_39, elem22) as u64, test_case.get("TestStruct_39").unwrap().get("elem22_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_39, elem23) as u64, test_case.get("TestStruct_39").unwrap().get("elem23_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_39, elem24) as u64, test_case.get("TestStruct_39").unwrap().get("elem24_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_39, elem25) as u64, test_case.get("TestStruct_39").unwrap().get("elem25_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_39, elem26) as u64, test_case.get("TestStruct_39").unwrap().get("elem26_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_39, elem27) as u64, test_case.get("TestStruct_39").unwrap().get("elem27_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_39, elem28) as u64, test_case.get("TestStruct_39").unwrap().get("elem28_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_39, elem29) as u64, test_case.get("TestStruct_39").unwrap().get("elem29_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_39, elem30) as u64, test_case.get("TestStruct_39").unwrap().get("elem30_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_39, elem31) as u64, test_case.get("TestStruct_39").unwrap().get("elem31_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_39, elem32) as u64, test_case.get("TestStruct_39").unwrap().get("elem32_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_39, elem33) as u64, test_case.get("TestStruct_39").unwrap().get("elem33_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_39, elem34) as u64, test_case.get("TestStruct_39").unwrap().get("elem34_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_39, elem35) as u64, test_case.get("TestStruct_39").unwrap().get("elem35_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_39, elem36) as u64, test_case.get("TestStruct_39").unwrap().get("elem36_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_39, elem37) as u64, test_case.get("TestStruct_39").unwrap().get("elem37_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_39, elem38) as u64, test_case.get("TestStruct_39").unwrap().get("elem38_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_39, elem39) as u64, test_case.get("TestStruct_39").unwrap().get("elem39_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_40, elem1) as u64, test_case.get("TestStruct_40").unwrap().get("elem1_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_40, elem2) as u64, test_case.get("TestStruct_40").unwrap().get("elem2_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_40, elem3) as u64, test_case.get("TestStruct_40").unwrap().get("elem3_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_40, elem4) as u64, test_case.get("TestStruct_40").unwrap().get("elem4_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_40, elem5) as u64, test_case.get("TestStruct_40").unwrap().get("elem5_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_40, elem6) as u64, test_case.get("TestStruct_40").unwrap().get("elem6_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_40, elem7) as u64, test_case.get("TestStruct_40").unwrap().get("elem7_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_40, elem8) as u64, test_case.get("TestStruct_40").unwrap().get("elem8_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_40, elem9) as u64, test_case.get("TestStruct_40").unwrap().get("elem9_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_40, elem10) as u64, test_case.get("TestStruct_40").unwrap().get("elem10_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_40, elem11) as u64, test_case.get("TestStruct_40").unwrap().get("elem11_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_40, elem12) as u64, test_case.get("TestStruct_40").unwrap().get("elem12_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_40, elem13) as u64, test_case.get("TestStruct_40").unwrap().get("elem13_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_40, elem14) as u64, test_case.get("TestStruct_40").unwrap().get("elem14_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_40, elem15) as u64, test_case.get("TestStruct_40").unwrap().get("elem15_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_40, elem16) as u64, test_case.get("TestStruct_40").unwrap().get("elem16_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_40, elem17) as u64, test_case.get("TestStruct_40").unwrap().get("elem17_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_40, elem18) as u64, test_case.get("TestStruct_40").unwrap().get("elem18_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_40, elem19) as u64, test_case.get("TestStruct_40").unwrap().get("elem19_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_40, elem20) as u64, test_case.get("TestStruct_40").unwrap().get("elem20_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_40, elem21) as u64, test_case.get("TestStruct_40").unwrap().get("elem21_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_40, elem22) as u64, test_case.get("TestStruct_40").unwrap().get("elem22_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_40, elem23) as u64, test_case.get("TestStruct_40").unwrap().get("elem23_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_40, elem24) as u64, test_case.get("TestStruct_40").unwrap().get("elem24_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_40, elem25) as u64, test_case.get("TestStruct_40").unwrap().get("elem25_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_40, elem26) as u64, test_case.get("TestStruct_40").unwrap().get("elem26_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_40, elem27) as u64, test_case.get("TestStruct_40").unwrap().get("elem27_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_40, elem28) as u64, test_case.get("TestStruct_40").unwrap().get("elem28_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_40, elem29) as u64, test_case.get("TestStruct_40").unwrap().get("elem29_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_40, elem30) as u64, test_case.get("TestStruct_40").unwrap().get("elem30_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_40, elem31) as u64, test_case.get("TestStruct_40").unwrap().get("elem31_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_40, elem32) as u64, test_case.get("TestStruct_40").unwrap().get("elem32_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_40, elem33) as u64, test_case.get("TestStruct_40").unwrap().get("elem33_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_40, elem34) as u64, test_case.get("TestStruct_40").unwrap().get("elem34_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_40, elem35) as u64, test_case.get("TestStruct_40").unwrap().get("elem35_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_40, elem36) as u64, test_case.get("TestStruct_40").unwrap().get("elem36_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_40, elem37) as u64, test_case.get("TestStruct_40").unwrap().get("elem37_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_40, elem38) as u64, test_case.get("TestStruct_40").unwrap().get("elem38_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_40, elem39) as u64, test_case.get("TestStruct_40").unwrap().get("elem39_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_40, elem40) as u64, test_case.get("TestStruct_40").unwrap().get("elem40_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_41, elem1) as u64, test_case.get("TestStruct_41").unwrap().get("elem1_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_41, elem2) as u64, test_case.get("TestStruct_41").unwrap().get("elem2_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_41, elem3) as u64, test_case.get("TestStruct_41").unwrap().get("elem3_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_41, elem4) as u64, test_case.get("TestStruct_41").unwrap().get("elem4_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_41, elem5) as u64, test_case.get("TestStruct_41").unwrap().get("elem5_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_41, elem6) as u64, test_case.get("TestStruct_41").unwrap().get("elem6_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_41, elem7) as u64, test_case.get("TestStruct_41").unwrap().get("elem7_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_41, elem8) as u64, test_case.get("TestStruct_41").unwrap().get("elem8_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_41, elem9) as u64, test_case.get("TestStruct_41").unwrap().get("elem9_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_41, elem10) as u64, test_case.get("TestStruct_41").unwrap().get("elem10_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_41, elem11) as u64, test_case.get("TestStruct_41").unwrap().get("elem11_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_41, elem12) as u64, test_case.get("TestStruct_41").unwrap().get("elem12_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_41, elem13) as u64, test_case.get("TestStruct_41").unwrap().get("elem13_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_41, elem14) as u64, test_case.get("TestStruct_41").unwrap().get("elem14_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_41, elem15) as u64, test_case.get("TestStruct_41").unwrap().get("elem15_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_41, elem16) as u64, test_case.get("TestStruct_41").unwrap().get("elem16_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_41, elem17) as u64, test_case.get("TestStruct_41").unwrap().get("elem17_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_41, elem18) as u64, test_case.get("TestStruct_41").unwrap().get("elem18_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_41, elem19) as u64, test_case.get("TestStruct_41").unwrap().get("elem19_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_41, elem20) as u64, test_case.get("TestStruct_41").unwrap().get("elem20_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_41, elem21) as u64, test_case.get("TestStruct_41").unwrap().get("elem21_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_41, elem22) as u64, test_case.get("TestStruct_41").unwrap().get("elem22_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_41, elem23) as u64, test_case.get("TestStruct_41").unwrap().get("elem23_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_41, elem24) as u64, test_case.get("TestStruct_41").unwrap().get("elem24_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_41, elem25) as u64, test_case.get("TestStruct_41").unwrap().get("elem25_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_41, elem26) as u64, test_case.get("TestStruct_41").unwrap().get("elem26_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_41, elem27) as u64, test_case.get("TestStruct_41").unwrap().get("elem27_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_41, elem28) as u64, test_case.get("TestStruct_41").unwrap().get("elem28_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_41, elem29) as u64, test_case.get("TestStruct_41").unwrap().get("elem29_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_41, elem30) as u64, test_case.get("TestStruct_41").unwrap().get("elem30_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_41, elem31) as u64, test_case.get("TestStruct_41").unwrap().get("elem31_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_41, elem32) as u64, test_case.get("TestStruct_41").unwrap().get("elem32_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_41, elem33) as u64, test_case.get("TestStruct_41").unwrap().get("elem33_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_41, elem34) as u64, test_case.get("TestStruct_41").unwrap().get("elem34_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_41, elem35) as u64, test_case.get("TestStruct_41").unwrap().get("elem35_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_41, elem36) as u64, test_case.get("TestStruct_41").unwrap().get("elem36_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_41, elem37) as u64, test_case.get("TestStruct_41").unwrap().get("elem37_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_41, elem38) as u64, test_case.get("TestStruct_41").unwrap().get("elem38_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_41, elem39) as u64, test_case.get("TestStruct_41").unwrap().get("elem39_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_41, elem40) as u64, test_case.get("TestStruct_41").unwrap().get("elem40_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_41, elem41) as u64, test_case.get("TestStruct_41").unwrap().get("elem41_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_42, elem1) as u64, test_case.get("TestStruct_42").unwrap().get("elem1_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_42, elem2) as u64, test_case.get("TestStruct_42").unwrap().get("elem2_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_42, elem3) as u64, test_case.get("TestStruct_42").unwrap().get("elem3_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_42, elem4) as u64, test_case.get("TestStruct_42").unwrap().get("elem4_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_42, elem5) as u64, test_case.get("TestStruct_42").unwrap().get("elem5_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_42, elem6) as u64, test_case.get("TestStruct_42").unwrap().get("elem6_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_42, elem7) as u64, test_case.get("TestStruct_42").unwrap().get("elem7_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_42, elem8) as u64, test_case.get("TestStruct_42").unwrap().get("elem8_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_42, elem9) as u64, test_case.get("TestStruct_42").unwrap().get("elem9_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_42, elem10) as u64, test_case.get("TestStruct_42").unwrap().get("elem10_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_42, elem11) as u64, test_case.get("TestStruct_42").unwrap().get("elem11_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_42, elem12) as u64, test_case.get("TestStruct_42").unwrap().get("elem12_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_42, elem13) as u64, test_case.get("TestStruct_42").unwrap().get("elem13_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_42, elem14) as u64, test_case.get("TestStruct_42").unwrap().get("elem14_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_42, elem15) as u64, test_case.get("TestStruct_42").unwrap().get("elem15_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_42, elem16) as u64, test_case.get("TestStruct_42").unwrap().get("elem16_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_42, elem17) as u64, test_case.get("TestStruct_42").unwrap().get("elem17_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_42, elem18) as u64, test_case.get("TestStruct_42").unwrap().get("elem18_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_42, elem19) as u64, test_case.get("TestStruct_42").unwrap().get("elem19_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_42, elem20) as u64, test_case.get("TestStruct_42").unwrap().get("elem20_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_42, elem21) as u64, test_case.get("TestStruct_42").unwrap().get("elem21_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_42, elem22) as u64, test_case.get("TestStruct_42").unwrap().get("elem22_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_42, elem23) as u64, test_case.get("TestStruct_42").unwrap().get("elem23_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_42, elem24) as u64, test_case.get("TestStruct_42").unwrap().get("elem24_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_42, elem25) as u64, test_case.get("TestStruct_42").unwrap().get("elem25_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_42, elem26) as u64, test_case.get("TestStruct_42").unwrap().get("elem26_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_42, elem27) as u64, test_case.get("TestStruct_42").unwrap().get("elem27_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_42, elem28) as u64, test_case.get("TestStruct_42").unwrap().get("elem28_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_42, elem29) as u64, test_case.get("TestStruct_42").unwrap().get("elem29_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_42, elem30) as u64, test_case.get("TestStruct_42").unwrap().get("elem30_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_42, elem31) as u64, test_case.get("TestStruct_42").unwrap().get("elem31_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_42, elem32) as u64, test_case.get("TestStruct_42").unwrap().get("elem32_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_42, elem33) as u64, test_case.get("TestStruct_42").unwrap().get("elem33_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_42, elem34) as u64, test_case.get("TestStruct_42").unwrap().get("elem34_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_42, elem35) as u64, test_case.get("TestStruct_42").unwrap().get("elem35_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_42, elem36) as u64, test_case.get("TestStruct_42").unwrap().get("elem36_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_42, elem37) as u64, test_case.get("TestStruct_42").unwrap().get("elem37_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_42, elem38) as u64, test_case.get("TestStruct_42").unwrap().get("elem38_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_42, elem39) as u64, test_case.get("TestStruct_42").unwrap().get("elem39_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_42, elem40) as u64, test_case.get("TestStruct_42").unwrap().get("elem40_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_42, elem41) as u64, test_case.get("TestStruct_42").unwrap().get("elem41_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_42, elem42) as u64, test_case.get("TestStruct_42").unwrap().get("elem42_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_43, elem1) as u64, test_case.get("TestStruct_43").unwrap().get("elem1_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_43, elem2) as u64, test_case.get("TestStruct_43").unwrap().get("elem2_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_43, elem3) as u64, test_case.get("TestStruct_43").unwrap().get("elem3_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_43, elem4) as u64, test_case.get("TestStruct_43").unwrap().get("elem4_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_43, elem5) as u64, test_case.get("TestStruct_43").unwrap().get("elem5_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_43, elem6) as u64, test_case.get("TestStruct_43").unwrap().get("elem6_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_43, elem7) as u64, test_case.get("TestStruct_43").unwrap().get("elem7_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_43, elem8) as u64, test_case.get("TestStruct_43").unwrap().get("elem8_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_43, elem9) as u64, test_case.get("TestStruct_43").unwrap().get("elem9_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_43, elem10) as u64, test_case.get("TestStruct_43").unwrap().get("elem10_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_43, elem11) as u64, test_case.get("TestStruct_43").unwrap().get("elem11_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_43, elem12) as u64, test_case.get("TestStruct_43").unwrap().get("elem12_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_43, elem13) as u64, test_case.get("TestStruct_43").unwrap().get("elem13_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_43, elem14) as u64, test_case.get("TestStruct_43").unwrap().get("elem14_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_43, elem15) as u64, test_case.get("TestStruct_43").unwrap().get("elem15_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_43, elem16) as u64, test_case.get("TestStruct_43").unwrap().get("elem16_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_43, elem17) as u64, test_case.get("TestStruct_43").unwrap().get("elem17_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_43, elem18) as u64, test_case.get("TestStruct_43").unwrap().get("elem18_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_43, elem19) as u64, test_case.get("TestStruct_43").unwrap().get("elem19_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_43, elem20) as u64, test_case.get("TestStruct_43").unwrap().get("elem20_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_43, elem21) as u64, test_case.get("TestStruct_43").unwrap().get("elem21_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_43, elem22) as u64, test_case.get("TestStruct_43").unwrap().get("elem22_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_43, elem23) as u64, test_case.get("TestStruct_43").unwrap().get("elem23_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_43, elem24) as u64, test_case.get("TestStruct_43").unwrap().get("elem24_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_43, elem25) as u64, test_case.get("TestStruct_43").unwrap().get("elem25_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_43, elem26) as u64, test_case.get("TestStruct_43").unwrap().get("elem26_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_43, elem27) as u64, test_case.get("TestStruct_43").unwrap().get("elem27_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_43, elem28) as u64, test_case.get("TestStruct_43").unwrap().get("elem28_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_43, elem29) as u64, test_case.get("TestStruct_43").unwrap().get("elem29_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_43, elem30) as u64, test_case.get("TestStruct_43").unwrap().get("elem30_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_43, elem31) as u64, test_case.get("TestStruct_43").unwrap().get("elem31_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_43, elem32) as u64, test_case.get("TestStruct_43").unwrap().get("elem32_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_43, elem33) as u64, test_case.get("TestStruct_43").unwrap().get("elem33_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_43, elem34) as u64, test_case.get("TestStruct_43").unwrap().get("elem34_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_43, elem35) as u64, test_case.get("TestStruct_43").unwrap().get("elem35_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_43, elem36) as u64, test_case.get("TestStruct_43").unwrap().get("elem36_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_43, elem37) as u64, test_case.get("TestStruct_43").unwrap().get("elem37_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_43, elem38) as u64, test_case.get("TestStruct_43").unwrap().get("elem38_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_43, elem39) as u64, test_case.get("TestStruct_43").unwrap().get("elem39_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_43, elem40) as u64, test_case.get("TestStruct_43").unwrap().get("elem40_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_43, elem41) as u64, test_case.get("TestStruct_43").unwrap().get("elem41_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_43, elem42) as u64, test_case.get("TestStruct_43").unwrap().get("elem42_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_43, elem43) as u64, test_case.get("TestStruct_43").unwrap().get("elem43_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_44, elem1) as u64, test_case.get("TestStruct_44").unwrap().get("elem1_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_44, elem2) as u64, test_case.get("TestStruct_44").unwrap().get("elem2_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_44, elem3) as u64, test_case.get("TestStruct_44").unwrap().get("elem3_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_44, elem4) as u64, test_case.get("TestStruct_44").unwrap().get("elem4_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_44, elem5) as u64, test_case.get("TestStruct_44").unwrap().get("elem5_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_44, elem6) as u64, test_case.get("TestStruct_44").unwrap().get("elem6_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_44, elem7) as u64, test_case.get("TestStruct_44").unwrap().get("elem7_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_44, elem8) as u64, test_case.get("TestStruct_44").unwrap().get("elem8_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_44, elem9) as u64, test_case.get("TestStruct_44").unwrap().get("elem9_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_44, elem10) as u64, test_case.get("TestStruct_44").unwrap().get("elem10_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_44, elem11) as u64, test_case.get("TestStruct_44").unwrap().get("elem11_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_44, elem12) as u64, test_case.get("TestStruct_44").unwrap().get("elem12_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_44, elem13) as u64, test_case.get("TestStruct_44").unwrap().get("elem13_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_44, elem14) as u64, test_case.get("TestStruct_44").unwrap().get("elem14_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_44, elem15) as u64, test_case.get("TestStruct_44").unwrap().get("elem15_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_44, elem16) as u64, test_case.get("TestStruct_44").unwrap().get("elem16_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_44, elem17) as u64, test_case.get("TestStruct_44").unwrap().get("elem17_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_44, elem18) as u64, test_case.get("TestStruct_44").unwrap().get("elem18_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_44, elem19) as u64, test_case.get("TestStruct_44").unwrap().get("elem19_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_44, elem20) as u64, test_case.get("TestStruct_44").unwrap().get("elem20_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_44, elem21) as u64, test_case.get("TestStruct_44").unwrap().get("elem21_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_44, elem22) as u64, test_case.get("TestStruct_44").unwrap().get("elem22_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_44, elem23) as u64, test_case.get("TestStruct_44").unwrap().get("elem23_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_44, elem24) as u64, test_case.get("TestStruct_44").unwrap().get("elem24_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_44, elem25) as u64, test_case.get("TestStruct_44").unwrap().get("elem25_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_44, elem26) as u64, test_case.get("TestStruct_44").unwrap().get("elem26_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_44, elem27) as u64, test_case.get("TestStruct_44").unwrap().get("elem27_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_44, elem28) as u64, test_case.get("TestStruct_44").unwrap().get("elem28_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_44, elem29) as u64, test_case.get("TestStruct_44").unwrap().get("elem29_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_44, elem30) as u64, test_case.get("TestStruct_44").unwrap().get("elem30_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_44, elem31) as u64, test_case.get("TestStruct_44").unwrap().get("elem31_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_44, elem32) as u64, test_case.get("TestStruct_44").unwrap().get("elem32_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_44, elem33) as u64, test_case.get("TestStruct_44").unwrap().get("elem33_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_44, elem34) as u64, test_case.get("TestStruct_44").unwrap().get("elem34_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_44, elem35) as u64, test_case.get("TestStruct_44").unwrap().get("elem35_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_44, elem36) as u64, test_case.get("TestStruct_44").unwrap().get("elem36_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_44, elem37) as u64, test_case.get("TestStruct_44").unwrap().get("elem37_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_44, elem38) as u64, test_case.get("TestStruct_44").unwrap().get("elem38_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_44, elem39) as u64, test_case.get("TestStruct_44").unwrap().get("elem39_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_44, elem40) as u64, test_case.get("TestStruct_44").unwrap().get("elem40_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_44, elem41) as u64, test_case.get("TestStruct_44").unwrap().get("elem41_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_44, elem42) as u64, test_case.get("TestStruct_44").unwrap().get("elem42_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_44, elem43) as u64, test_case.get("TestStruct_44").unwrap().get("elem43_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_44, elem44) as u64, test_case.get("TestStruct_44").unwrap().get("elem44_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_45, elem1) as u64, test_case.get("TestStruct_45").unwrap().get("elem1_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_45, elem2) as u64, test_case.get("TestStruct_45").unwrap().get("elem2_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_45, elem3) as u64, test_case.get("TestStruct_45").unwrap().get("elem3_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_45, elem4) as u64, test_case.get("TestStruct_45").unwrap().get("elem4_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_45, elem5) as u64, test_case.get("TestStruct_45").unwrap().get("elem5_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_45, elem6) as u64, test_case.get("TestStruct_45").unwrap().get("elem6_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_45, elem7) as u64, test_case.get("TestStruct_45").unwrap().get("elem7_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_45, elem8) as u64, test_case.get("TestStruct_45").unwrap().get("elem8_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_45, elem9) as u64, test_case.get("TestStruct_45").unwrap().get("elem9_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_45, elem10) as u64, test_case.get("TestStruct_45").unwrap().get("elem10_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_45, elem11) as u64, test_case.get("TestStruct_45").unwrap().get("elem11_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_45, elem12) as u64, test_case.get("TestStruct_45").unwrap().get("elem12_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_45, elem13) as u64, test_case.get("TestStruct_45").unwrap().get("elem13_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_45, elem14) as u64, test_case.get("TestStruct_45").unwrap().get("elem14_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_45, elem15) as u64, test_case.get("TestStruct_45").unwrap().get("elem15_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_45, elem16) as u64, test_case.get("TestStruct_45").unwrap().get("elem16_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_45, elem17) as u64, test_case.get("TestStruct_45").unwrap().get("elem17_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_45, elem18) as u64, test_case.get("TestStruct_45").unwrap().get("elem18_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_45, elem19) as u64, test_case.get("TestStruct_45").unwrap().get("elem19_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_45, elem20) as u64, test_case.get("TestStruct_45").unwrap().get("elem20_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_45, elem21) as u64, test_case.get("TestStruct_45").unwrap().get("elem21_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_45, elem22) as u64, test_case.get("TestStruct_45").unwrap().get("elem22_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_45, elem23) as u64, test_case.get("TestStruct_45").unwrap().get("elem23_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_45, elem24) as u64, test_case.get("TestStruct_45").unwrap().get("elem24_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_45, elem25) as u64, test_case.get("TestStruct_45").unwrap().get("elem25_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_45, elem26) as u64, test_case.get("TestStruct_45").unwrap().get("elem26_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_45, elem27) as u64, test_case.get("TestStruct_45").unwrap().get("elem27_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_45, elem28) as u64, test_case.get("TestStruct_45").unwrap().get("elem28_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_45, elem29) as u64, test_case.get("TestStruct_45").unwrap().get("elem29_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_45, elem30) as u64, test_case.get("TestStruct_45").unwrap().get("elem30_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_45, elem31) as u64, test_case.get("TestStruct_45").unwrap().get("elem31_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_45, elem32) as u64, test_case.get("TestStruct_45").unwrap().get("elem32_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_45, elem33) as u64, test_case.get("TestStruct_45").unwrap().get("elem33_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_45, elem34) as u64, test_case.get("TestStruct_45").unwrap().get("elem34_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_45, elem35) as u64, test_case.get("TestStruct_45").unwrap().get("elem35_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_45, elem36) as u64, test_case.get("TestStruct_45").unwrap().get("elem36_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_45, elem37) as u64, test_case.get("TestStruct_45").unwrap().get("elem37_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_45, elem38) as u64, test_case.get("TestStruct_45").unwrap().get("elem38_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_45, elem39) as u64, test_case.get("TestStruct_45").unwrap().get("elem39_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_45, elem40) as u64, test_case.get("TestStruct_45").unwrap().get("elem40_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_45, elem41) as u64, test_case.get("TestStruct_45").unwrap().get("elem41_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_45, elem42) as u64, test_case.get("TestStruct_45").unwrap().get("elem42_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_45, elem43) as u64, test_case.get("TestStruct_45").unwrap().get("elem43_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_45, elem44) as u64, test_case.get("TestStruct_45").unwrap().get("elem44_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_45, elem45) as u64, test_case.get("TestStruct_45").unwrap().get("elem45_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_46, elem1) as u64, test_case.get("TestStruct_46").unwrap().get("elem1_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_46, elem2) as u64, test_case.get("TestStruct_46").unwrap().get("elem2_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_46, elem3) as u64, test_case.get("TestStruct_46").unwrap().get("elem3_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_46, elem4) as u64, test_case.get("TestStruct_46").unwrap().get("elem4_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_46, elem5) as u64, test_case.get("TestStruct_46").unwrap().get("elem5_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_46, elem6) as u64, test_case.get("TestStruct_46").unwrap().get("elem6_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_46, elem7) as u64, test_case.get("TestStruct_46").unwrap().get("elem7_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_46, elem8) as u64, test_case.get("TestStruct_46").unwrap().get("elem8_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_46, elem9) as u64, test_case.get("TestStruct_46").unwrap().get("elem9_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_46, elem10) as u64, test_case.get("TestStruct_46").unwrap().get("elem10_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_46, elem11) as u64, test_case.get("TestStruct_46").unwrap().get("elem11_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_46, elem12) as u64, test_case.get("TestStruct_46").unwrap().get("elem12_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_46, elem13) as u64, test_case.get("TestStruct_46").unwrap().get("elem13_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_46, elem14) as u64, test_case.get("TestStruct_46").unwrap().get("elem14_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_46, elem15) as u64, test_case.get("TestStruct_46").unwrap().get("elem15_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_46, elem16) as u64, test_case.get("TestStruct_46").unwrap().get("elem16_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_46, elem17) as u64, test_case.get("TestStruct_46").unwrap().get("elem17_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_46, elem18) as u64, test_case.get("TestStruct_46").unwrap().get("elem18_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_46, elem19) as u64, test_case.get("TestStruct_46").unwrap().get("elem19_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_46, elem20) as u64, test_case.get("TestStruct_46").unwrap().get("elem20_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_46, elem21) as u64, test_case.get("TestStruct_46").unwrap().get("elem21_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_46, elem22) as u64, test_case.get("TestStruct_46").unwrap().get("elem22_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_46, elem23) as u64, test_case.get("TestStruct_46").unwrap().get("elem23_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_46, elem24) as u64, test_case.get("TestStruct_46").unwrap().get("elem24_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_46, elem25) as u64, test_case.get("TestStruct_46").unwrap().get("elem25_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_46, elem26) as u64, test_case.get("TestStruct_46").unwrap().get("elem26_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_46, elem27) as u64, test_case.get("TestStruct_46").unwrap().get("elem27_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_46, elem28) as u64, test_case.get("TestStruct_46").unwrap().get("elem28_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_46, elem29) as u64, test_case.get("TestStruct_46").unwrap().get("elem29_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_46, elem30) as u64, test_case.get("TestStruct_46").unwrap().get("elem30_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_46, elem31) as u64, test_case.get("TestStruct_46").unwrap().get("elem31_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_46, elem32) as u64, test_case.get("TestStruct_46").unwrap().get("elem32_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_46, elem33) as u64, test_case.get("TestStruct_46").unwrap().get("elem33_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_46, elem34) as u64, test_case.get("TestStruct_46").unwrap().get("elem34_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_46, elem35) as u64, test_case.get("TestStruct_46").unwrap().get("elem35_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_46, elem36) as u64, test_case.get("TestStruct_46").unwrap().get("elem36_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_46, elem37) as u64, test_case.get("TestStruct_46").unwrap().get("elem37_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_46, elem38) as u64, test_case.get("TestStruct_46").unwrap().get("elem38_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_46, elem39) as u64, test_case.get("TestStruct_46").unwrap().get("elem39_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_46, elem40) as u64, test_case.get("TestStruct_46").unwrap().get("elem40_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_46, elem41) as u64, test_case.get("TestStruct_46").unwrap().get("elem41_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_46, elem42) as u64, test_case.get("TestStruct_46").unwrap().get("elem42_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_46, elem43) as u64, test_case.get("TestStruct_46").unwrap().get("elem43_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_46, elem44) as u64, test_case.get("TestStruct_46").unwrap().get("elem44_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_46, elem45) as u64, test_case.get("TestStruct_46").unwrap().get("elem45_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_46, elem46) as u64, test_case.get("TestStruct_46").unwrap().get("elem46_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_47, elem1) as u64, test_case.get("TestStruct_47").unwrap().get("elem1_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_47, elem2) as u64, test_case.get("TestStruct_47").unwrap().get("elem2_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_47, elem3) as u64, test_case.get("TestStruct_47").unwrap().get("elem3_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_47, elem4) as u64, test_case.get("TestStruct_47").unwrap().get("elem4_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_47, elem5) as u64, test_case.get("TestStruct_47").unwrap().get("elem5_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_47, elem6) as u64, test_case.get("TestStruct_47").unwrap().get("elem6_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_47, elem7) as u64, test_case.get("TestStruct_47").unwrap().get("elem7_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_47, elem8) as u64, test_case.get("TestStruct_47").unwrap().get("elem8_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_47, elem9) as u64, test_case.get("TestStruct_47").unwrap().get("elem9_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_47, elem10) as u64, test_case.get("TestStruct_47").unwrap().get("elem10_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_47, elem11) as u64, test_case.get("TestStruct_47").unwrap().get("elem11_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_47, elem12) as u64, test_case.get("TestStruct_47").unwrap().get("elem12_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_47, elem13) as u64, test_case.get("TestStruct_47").unwrap().get("elem13_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_47, elem14) as u64, test_case.get("TestStruct_47").unwrap().get("elem14_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_47, elem15) as u64, test_case.get("TestStruct_47").unwrap().get("elem15_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_47, elem16) as u64, test_case.get("TestStruct_47").unwrap().get("elem16_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_47, elem17) as u64, test_case.get("TestStruct_47").unwrap().get("elem17_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_47, elem18) as u64, test_case.get("TestStruct_47").unwrap().get("elem18_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_47, elem19) as u64, test_case.get("TestStruct_47").unwrap().get("elem19_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_47, elem20) as u64, test_case.get("TestStruct_47").unwrap().get("elem20_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_47, elem21) as u64, test_case.get("TestStruct_47").unwrap().get("elem21_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_47, elem22) as u64, test_case.get("TestStruct_47").unwrap().get("elem22_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_47, elem23) as u64, test_case.get("TestStruct_47").unwrap().get("elem23_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_47, elem24) as u64, test_case.get("TestStruct_47").unwrap().get("elem24_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_47, elem25) as u64, test_case.get("TestStruct_47").unwrap().get("elem25_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_47, elem26) as u64, test_case.get("TestStruct_47").unwrap().get("elem26_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_47, elem27) as u64, test_case.get("TestStruct_47").unwrap().get("elem27_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_47, elem28) as u64, test_case.get("TestStruct_47").unwrap().get("elem28_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_47, elem29) as u64, test_case.get("TestStruct_47").unwrap().get("elem29_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_47, elem30) as u64, test_case.get("TestStruct_47").unwrap().get("elem30_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_47, elem31) as u64, test_case.get("TestStruct_47").unwrap().get("elem31_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_47, elem32) as u64, test_case.get("TestStruct_47").unwrap().get("elem32_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_47, elem33) as u64, test_case.get("TestStruct_47").unwrap().get("elem33_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_47, elem34) as u64, test_case.get("TestStruct_47").unwrap().get("elem34_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_47, elem35) as u64, test_case.get("TestStruct_47").unwrap().get("elem35_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_47, elem36) as u64, test_case.get("TestStruct_47").unwrap().get("elem36_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_47, elem37) as u64, test_case.get("TestStruct_47").unwrap().get("elem37_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_47, elem38) as u64, test_case.get("TestStruct_47").unwrap().get("elem38_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_47, elem39) as u64, test_case.get("TestStruct_47").unwrap().get("elem39_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_47, elem40) as u64, test_case.get("TestStruct_47").unwrap().get("elem40_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_47, elem41) as u64, test_case.get("TestStruct_47").unwrap().get("elem41_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_47, elem42) as u64, test_case.get("TestStruct_47").unwrap().get("elem42_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_47, elem43) as u64, test_case.get("TestStruct_47").unwrap().get("elem43_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_47, elem44) as u64, test_case.get("TestStruct_47").unwrap().get("elem44_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_47, elem45) as u64, test_case.get("TestStruct_47").unwrap().get("elem45_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_47, elem46) as u64, test_case.get("TestStruct_47").unwrap().get("elem46_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_47, elem47) as u64, test_case.get("TestStruct_47").unwrap().get("elem47_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_48, elem1) as u64, test_case.get("TestStruct_48").unwrap().get("elem1_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_48, elem2) as u64, test_case.get("TestStruct_48").unwrap().get("elem2_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_48, elem3) as u64, test_case.get("TestStruct_48").unwrap().get("elem3_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_48, elem4) as u64, test_case.get("TestStruct_48").unwrap().get("elem4_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_48, elem5) as u64, test_case.get("TestStruct_48").unwrap().get("elem5_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_48, elem6) as u64, test_case.get("TestStruct_48").unwrap().get("elem6_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_48, elem7) as u64, test_case.get("TestStruct_48").unwrap().get("elem7_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_48, elem8) as u64, test_case.get("TestStruct_48").unwrap().get("elem8_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_48, elem9) as u64, test_case.get("TestStruct_48").unwrap().get("elem9_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_48, elem10) as u64, test_case.get("TestStruct_48").unwrap().get("elem10_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_48, elem11) as u64, test_case.get("TestStruct_48").unwrap().get("elem11_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_48, elem12) as u64, test_case.get("TestStruct_48").unwrap().get("elem12_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_48, elem13) as u64, test_case.get("TestStruct_48").unwrap().get("elem13_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_48, elem14) as u64, test_case.get("TestStruct_48").unwrap().get("elem14_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_48, elem15) as u64, test_case.get("TestStruct_48").unwrap().get("elem15_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_48, elem16) as u64, test_case.get("TestStruct_48").unwrap().get("elem16_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_48, elem17) as u64, test_case.get("TestStruct_48").unwrap().get("elem17_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_48, elem18) as u64, test_case.get("TestStruct_48").unwrap().get("elem18_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_48, elem19) as u64, test_case.get("TestStruct_48").unwrap().get("elem19_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_48, elem20) as u64, test_case.get("TestStruct_48").unwrap().get("elem20_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_48, elem21) as u64, test_case.get("TestStruct_48").unwrap().get("elem21_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_48, elem22) as u64, test_case.get("TestStruct_48").unwrap().get("elem22_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_48, elem23) as u64, test_case.get("TestStruct_48").unwrap().get("elem23_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_48, elem24) as u64, test_case.get("TestStruct_48").unwrap().get("elem24_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_48, elem25) as u64, test_case.get("TestStruct_48").unwrap().get("elem25_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_48, elem26) as u64, test_case.get("TestStruct_48").unwrap().get("elem26_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_48, elem27) as u64, test_case.get("TestStruct_48").unwrap().get("elem27_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_48, elem28) as u64, test_case.get("TestStruct_48").unwrap().get("elem28_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_48, elem29) as u64, test_case.get("TestStruct_48").unwrap().get("elem29_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_48, elem30) as u64, test_case.get("TestStruct_48").unwrap().get("elem30_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_48, elem31) as u64, test_case.get("TestStruct_48").unwrap().get("elem31_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_48, elem32) as u64, test_case.get("TestStruct_48").unwrap().get("elem32_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_48, elem33) as u64, test_case.get("TestStruct_48").unwrap().get("elem33_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_48, elem34) as u64, test_case.get("TestStruct_48").unwrap().get("elem34_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_48, elem35) as u64, test_case.get("TestStruct_48").unwrap().get("elem35_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_48, elem36) as u64, test_case.get("TestStruct_48").unwrap().get("elem36_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_48, elem37) as u64, test_case.get("TestStruct_48").unwrap().get("elem37_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_48, elem38) as u64, test_case.get("TestStruct_48").unwrap().get("elem38_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_48, elem39) as u64, test_case.get("TestStruct_48").unwrap().get("elem39_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_48, elem40) as u64, test_case.get("TestStruct_48").unwrap().get("elem40_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_48, elem41) as u64, test_case.get("TestStruct_48").unwrap().get("elem41_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_48, elem42) as u64, test_case.get("TestStruct_48").unwrap().get("elem42_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_48, elem43) as u64, test_case.get("TestStruct_48").unwrap().get("elem43_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_48, elem44) as u64, test_case.get("TestStruct_48").unwrap().get("elem44_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_48, elem45) as u64, test_case.get("TestStruct_48").unwrap().get("elem45_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_48, elem46) as u64, test_case.get("TestStruct_48").unwrap().get("elem46_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_48, elem47) as u64, test_case.get("TestStruct_48").unwrap().get("elem47_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_48, elem48) as u64, test_case.get("TestStruct_48").unwrap().get("elem48_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_49, elem1) as u64, test_case.get("TestStruct_49").unwrap().get("elem1_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_49, elem2) as u64, test_case.get("TestStruct_49").unwrap().get("elem2_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_49, elem3) as u64, test_case.get("TestStruct_49").unwrap().get("elem3_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_49, elem4) as u64, test_case.get("TestStruct_49").unwrap().get("elem4_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_49, elem5) as u64, test_case.get("TestStruct_49").unwrap().get("elem5_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_49, elem6) as u64, test_case.get("TestStruct_49").unwrap().get("elem6_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_49, elem7) as u64, test_case.get("TestStruct_49").unwrap().get("elem7_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_49, elem8) as u64, test_case.get("TestStruct_49").unwrap().get("elem8_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_49, elem9) as u64, test_case.get("TestStruct_49").unwrap().get("elem9_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_49, elem10) as u64, test_case.get("TestStruct_49").unwrap().get("elem10_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_49, elem11) as u64, test_case.get("TestStruct_49").unwrap().get("elem11_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_49, elem12) as u64, test_case.get("TestStruct_49").unwrap().get("elem12_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_49, elem13) as u64, test_case.get("TestStruct_49").unwrap().get("elem13_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_49, elem14) as u64, test_case.get("TestStruct_49").unwrap().get("elem14_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_49, elem15) as u64, test_case.get("TestStruct_49").unwrap().get("elem15_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_49, elem16) as u64, test_case.get("TestStruct_49").unwrap().get("elem16_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_49, elem17) as u64, test_case.get("TestStruct_49").unwrap().get("elem17_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_49, elem18) as u64, test_case.get("TestStruct_49").unwrap().get("elem18_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_49, elem19) as u64, test_case.get("TestStruct_49").unwrap().get("elem19_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_49, elem20) as u64, test_case.get("TestStruct_49").unwrap().get("elem20_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_49, elem21) as u64, test_case.get("TestStruct_49").unwrap().get("elem21_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_49, elem22) as u64, test_case.get("TestStruct_49").unwrap().get("elem22_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_49, elem23) as u64, test_case.get("TestStruct_49").unwrap().get("elem23_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_49, elem24) as u64, test_case.get("TestStruct_49").unwrap().get("elem24_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_49, elem25) as u64, test_case.get("TestStruct_49").unwrap().get("elem25_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_49, elem26) as u64, test_case.get("TestStruct_49").unwrap().get("elem26_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_49, elem27) as u64, test_case.get("TestStruct_49").unwrap().get("elem27_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_49, elem28) as u64, test_case.get("TestStruct_49").unwrap().get("elem28_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_49, elem29) as u64, test_case.get("TestStruct_49").unwrap().get("elem29_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_49, elem30) as u64, test_case.get("TestStruct_49").unwrap().get("elem30_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_49, elem31) as u64, test_case.get("TestStruct_49").unwrap().get("elem31_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_49, elem32) as u64, test_case.get("TestStruct_49").unwrap().get("elem32_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_49, elem33) as u64, test_case.get("TestStruct_49").unwrap().get("elem33_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_49, elem34) as u64, test_case.get("TestStruct_49").unwrap().get("elem34_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_49, elem35) as u64, test_case.get("TestStruct_49").unwrap().get("elem35_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_49, elem36) as u64, test_case.get("TestStruct_49").unwrap().get("elem36_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_49, elem37) as u64, test_case.get("TestStruct_49").unwrap().get("elem37_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_49, elem38) as u64, test_case.get("TestStruct_49").unwrap().get("elem38_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_49, elem39) as u64, test_case.get("TestStruct_49").unwrap().get("elem39_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_49, elem40) as u64, test_case.get("TestStruct_49").unwrap().get("elem40_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_49, elem41) as u64, test_case.get("TestStruct_49").unwrap().get("elem41_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_49, elem42) as u64, test_case.get("TestStruct_49").unwrap().get("elem42_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_49, elem43) as u64, test_case.get("TestStruct_49").unwrap().get("elem43_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_49, elem44) as u64, test_case.get("TestStruct_49").unwrap().get("elem44_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_49, elem45) as u64, test_case.get("TestStruct_49").unwrap().get("elem45_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_49, elem46) as u64, test_case.get("TestStruct_49").unwrap().get("elem46_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_49, elem47) as u64, test_case.get("TestStruct_49").unwrap().get("elem47_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_49, elem48) as u64, test_case.get("TestStruct_49").unwrap().get("elem48_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_49, elem49) as u64, test_case.get("TestStruct_49").unwrap().get("elem49_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_50, elem1) as u64, test_case.get("TestStruct_50").unwrap().get("elem1_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_50, elem2) as u64, test_case.get("TestStruct_50").unwrap().get("elem2_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_50, elem3) as u64, test_case.get("TestStruct_50").unwrap().get("elem3_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_50, elem4) as u64, test_case.get("TestStruct_50").unwrap().get("elem4_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_50, elem5) as u64, test_case.get("TestStruct_50").unwrap().get("elem5_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_50, elem6) as u64, test_case.get("TestStruct_50").unwrap().get("elem6_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_50, elem7) as u64, test_case.get("TestStruct_50").unwrap().get("elem7_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_50, elem8) as u64, test_case.get("TestStruct_50").unwrap().get("elem8_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_50, elem9) as u64, test_case.get("TestStruct_50").unwrap().get("elem9_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_50, elem10) as u64, test_case.get("TestStruct_50").unwrap().get("elem10_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_50, elem11) as u64, test_case.get("TestStruct_50").unwrap().get("elem11_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_50, elem12) as u64, test_case.get("TestStruct_50").unwrap().get("elem12_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_50, elem13) as u64, test_case.get("TestStruct_50").unwrap().get("elem13_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_50, elem14) as u64, test_case.get("TestStruct_50").unwrap().get("elem14_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_50, elem15) as u64, test_case.get("TestStruct_50").unwrap().get("elem15_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_50, elem16) as u64, test_case.get("TestStruct_50").unwrap().get("elem16_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_50, elem17) as u64, test_case.get("TestStruct_50").unwrap().get("elem17_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_50, elem18) as u64, test_case.get("TestStruct_50").unwrap().get("elem18_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_50, elem19) as u64, test_case.get("TestStruct_50").unwrap().get("elem19_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_50, elem20) as u64, test_case.get("TestStruct_50").unwrap().get("elem20_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_50, elem21) as u64, test_case.get("TestStruct_50").unwrap().get("elem21_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_50, elem22) as u64, test_case.get("TestStruct_50").unwrap().get("elem22_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_50, elem23) as u64, test_case.get("TestStruct_50").unwrap().get("elem23_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_50, elem24) as u64, test_case.get("TestStruct_50").unwrap().get("elem24_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_50, elem25) as u64, test_case.get("TestStruct_50").unwrap().get("elem25_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_50, elem26) as u64, test_case.get("TestStruct_50").unwrap().get("elem26_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_50, elem27) as u64, test_case.get("TestStruct_50").unwrap().get("elem27_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_50, elem28) as u64, test_case.get("TestStruct_50").unwrap().get("elem28_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_50, elem29) as u64, test_case.get("TestStruct_50").unwrap().get("elem29_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_50, elem30) as u64, test_case.get("TestStruct_50").unwrap().get("elem30_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_50, elem31) as u64, test_case.get("TestStruct_50").unwrap().get("elem31_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_50, elem32) as u64, test_case.get("TestStruct_50").unwrap().get("elem32_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_50, elem33) as u64, test_case.get("TestStruct_50").unwrap().get("elem33_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_50, elem34) as u64, test_case.get("TestStruct_50").unwrap().get("elem34_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_50, elem35) as u64, test_case.get("TestStruct_50").unwrap().get("elem35_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_50, elem36) as u64, test_case.get("TestStruct_50").unwrap().get("elem36_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_50, elem37) as u64, test_case.get("TestStruct_50").unwrap().get("elem37_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_50, elem38) as u64, test_case.get("TestStruct_50").unwrap().get("elem38_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_50, elem39) as u64, test_case.get("TestStruct_50").unwrap().get("elem39_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_50, elem40) as u64, test_case.get("TestStruct_50").unwrap().get("elem40_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_50, elem41) as u64, test_case.get("TestStruct_50").unwrap().get("elem41_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_50, elem42) as u64, test_case.get("TestStruct_50").unwrap().get("elem42_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_50, elem43) as u64, test_case.get("TestStruct_50").unwrap().get("elem43_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_50, elem44) as u64, test_case.get("TestStruct_50").unwrap().get("elem44_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_50, elem45) as u64, test_case.get("TestStruct_50").unwrap().get("elem45_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_50, elem46) as u64, test_case.get("TestStruct_50").unwrap().get("elem46_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_50, elem47) as u64, test_case.get("TestStruct_50").unwrap().get("elem47_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_50, elem48) as u64, test_case.get("TestStruct_50").unwrap().get("elem48_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_50, elem49) as u64, test_case.get("TestStruct_50").unwrap().get("elem49_offset").unwrap().as_u64().unwrap());
assert_eq!(offset_of!(TestStruct_50, elem50) as u64, test_case.get("TestStruct_50").unwrap().get("elem50_offset").unwrap().as_u64().unwrap());
}
