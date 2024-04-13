#![allow(unused)]
mod formatter;
mod data;
mod interface;

use std::fmt::Display;

pub use formatter::*;
pub use data::*;
pub use interface::*;

pub fn branch_of<T: ToPrettyTree>(label: impl AsRef<str>, children: impl IntoIterator<Item=T>) -> PrettyTree {
    PrettyTree::branch_of(label, children)
}

pub fn field<T: ToPrettyTree>(key: impl AsRef<str>, value: &T) -> PrettyTree {
    PrettyTree::key_value(key.as_ref(), value)
}

pub fn string(value: impl AsRef<str>) -> PrettyTree {
    PrettyTree::String(value.as_ref().to_string())
}

pub fn value(value: impl Display) -> PrettyTree {
    PrettyTree::Value(value.to_string())
}

pub fn branch_builder(label: impl AsRef<str>) -> PrettyBranch {
    PrettyBranch::new(label)
}
