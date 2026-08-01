#![cfg(feature = "Clone")]
#![no_std]
#![deny(unfulfilled_lint_expectations, clippy::allow_attributes)]

use educe::Educe;

#[expect(non_camel_case_types)]
#[derive(Educe)]
#[educe(Clone)]
struct not_camel_case {
    value: u8,
}

#[test]
fn expect_is_propagated_as_allow() {
    let cloned = not_camel_case {
        value: 1
    }
    .clone();

    assert_eq!(1, cloned.value);
}
