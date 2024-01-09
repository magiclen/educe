// These types test that we don't have name clashes between field names
// and our local variables and parameter names.
//
// We don't need to actually use these anywhere - just compiling them shows things are OK.
#![allow(dead_code)]

use educe::Educe;

#[derive(Educe, Eq)]
#[educe(Clone, Debug, Hash, Default, PartialEq, PartialOrd, Ord)]
pub enum NameClashesEnum {
    #[educe(Default)]
    TestDefault {},
    TestClone {
        source:    i16,
        v_source_: i16,
    },
    TestDebug1 {
        f:            i8,
        v_formatter_: i8,
    },
    TestDebug2 {
        builder:    i32,
        v_builder_: i32,
    },
    TestHHash {
        state:    i32,
        v_state_: i32,
    },
    TestPartialEq {
        other:    i64,
        v_other_: i64,
    },
}

#[derive(Educe, Eq)]
#[educe(Clone, Debug, Hash, Default, PartialEq, PartialOrd, Ord)]
pub struct NameClashes {
    source:       i16,
    v_source_:    i16,
    f:            i8,
    v_formatter_: i8,
    builder:      i32,
    v_builder_:   i32,
    state:        i32,
    v_state_:     i32,
    other:        i64,
    v_other_:     i64,
}

#[derive(Educe)]
#[educe(Debug(unsafe), Hash(unsafe), PartialEq(unsafe), Default)]
pub union NameClashesUnion {
    source:       i16,
    v_source_:    i16,
    f:            i8,
    v_formatter_: i8,
    builder:      i32,
    v_builder_:   i32,
    state:        i32,
    v_state_:     i32,
    other:        i64,
    v_other_:     i64,
    #[educe(Default)]
    def:          (),
}
