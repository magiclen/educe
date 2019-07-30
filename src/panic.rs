#[inline]
pub fn reuse_a_trait(trait_name: &str) -> ! {
    panic!("The trait `{}` is duplicately used.", trait_name)
}

#[inline]
pub fn trait_not_used(trait_name: &str) -> ! {
    panic!("The `{}` trait is not used.", trait_name)
}

#[inline]
pub fn trait_not_support_union(trait_name: &str) -> ! {
    panic!("The `{}` trait does not support to a union.", trait_name)
}

#[inline]
pub fn attribute_incorrect_format(attribute_name: &str, correct_usage: &[&str]) -> ! {
    panic!("You are using an incorrect format of the `{}` attribute.{}", attribute_name, concat_string_slice_array(correct_usage))
}

#[inline]
pub fn parameter_incorrect_format(parameter_name: &str, correct_usage: &[&str]) -> ! {
    panic!("You are using an incorrect format of the `{}` parameter.{}", parameter_name, concat_string_slice_array(correct_usage))
}

#[inline]
pub fn derive_attribute_not_set_up_yet(attribute_name: &str) -> ! {
    panic!("You are using `{}` in the `derive` attribute, but it has not been set up yet.", attribute_name)
}

#[inline]
pub fn reset_parameter(parameter_name: &str) -> ! {
    panic!("Try to reset the `{}` parameter.", parameter_name)
}

#[inline]
pub fn unknown_parameter(attribute_name: &str, parameter_name: &str) -> ! {
    panic!("Unknown parameter `{}` used in the `{}` attribute.", parameter_name, attribute_name)
}

#[inline]
pub fn set_value_expression() -> ! {
    panic!("The default value and the expression parameter can not be set at the same time.")
}

#[inline]
pub fn disable_named_field_name() -> ! {
    panic!("You can't disable the name of a named field.")
}

#[inline]
pub fn empty_parameter(parameter_name: &str) -> ! {
    panic!("You can't set the `{}` parameter to empty.", parameter_name)
}

#[inline]
pub fn unit_struct_need_name() -> ! {
    panic!("A unit struct needs to have a name.")
}

#[inline]
pub fn unit_enum_need_name() -> ! {
    panic!("A unit enum needs to have a name.")
}

#[inline]
pub fn unit_variant_need_name() -> ! {
    panic!("A unit variant which doesn't use an enum name needs to have a name.")
}

// TODO patterns

#[inline]
pub fn educe_format_incorrect() -> ! {
    attribute_incorrect_format("educe", &[stringify!(#[educe(Trait1, Trait2, ..., TraitN)])])
}

fn concat_string_slice_array(array: &[&str]) -> String {
    let len = array.len();

    if len == 0 {
        String::new()
    } else {
        let mut string = String::from("It needs to be formed into ");

        let mut iter = array.iter();

        let first = iter.next().unwrap();

        string.push('`');
        string.push_str(&first.replace("\n", ""));
        string.push('`');

        if len > 2 {
            for s in iter.take(len - 2) {
                string.push_str(", `");
                string.push_str(&s.replace("\n", ""));
                string.push('`');
            }
        }

        if len > 1 {
            string.push_str(", or `");
            string.push_str(&array[len - 1].replace("\n", ""));
            string.push('`');
        }

        string.push('.');

        string
    }
}