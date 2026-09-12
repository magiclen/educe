#![cfg(all(feature = "Default", feature = "full"))]
#![no_std]

use educe::Educe;

#[test]
fn type_expression() {
    #[derive(Educe)]
    #[educe(Default(expression = match 1u8 {
        1 => Struct {
            f1: 10
        },
        _ => Struct {
            f1: 0
        },
    }))]
    struct Struct {
        f1: u8,
    }

    assert_eq!(10, Struct::default().f1);
}

#[test]
fn field_expression() {
    #[derive(Educe)]
    #[educe(Default)]
    struct Struct {
        #[educe(Default = {
            let value: u8 = 2;

            value * 3
        })]
        f1: u8,
    }

    assert_eq!(6, Struct::default().f1);
}
