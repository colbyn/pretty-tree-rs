use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

use crate::PrettyBranch;

use super::PrettyTree;

pub trait ToPrettyTree {
    fn to_pretty_tree(&self) -> PrettyTree;
}

impl ToPrettyTree for PrettyBranch {
    fn to_pretty_tree(&self) -> PrettyTree {
        PrettyTree::Branch(self.clone())
    }
}

pub trait PrettyTreePrinter {
    fn print_pretty_tree(&self);
}

impl ToPrettyTree for () {
    fn to_pretty_tree(&self) -> PrettyTree {
        PrettyTree::Value(String::from("()"))
    }
}
impl ToPrettyTree for bool {
    fn to_pretty_tree(&self) -> PrettyTree {
        match self {
            true => PrettyTree::Value(String::from("true")),
            false => PrettyTree::Value(String::from("false")),
        }
    }
}
impl ToPrettyTree for usize {
    fn to_pretty_tree(&self) -> PrettyTree {
        PrettyTree::Value(format!("{}", self))
    }
}
impl ToPrettyTree for u8 {
    fn to_pretty_tree(&self) -> PrettyTree {
        PrettyTree::Value(format!("{}", self))
    }
}
impl ToPrettyTree for u32 {
    fn to_pretty_tree(&self) -> PrettyTree {
        PrettyTree::Value(format!("{}", self))
    }
}
impl<Type> PrettyTreePrinter for Type where Type: ToPrettyTree {
    fn print_pretty_tree(&self) {
        let tree = self.to_pretty_tree().format(&Default::default());
        println!("{tree}")
    }
}

impl ToPrettyTree for PrettyTree {
    fn to_pretty_tree(&self) -> PrettyTree { self.clone() }
}
impl ToPrettyTree for String {
    fn to_pretty_tree(&self) -> PrettyTree { PrettyTree::string(self) }
}
impl<T> ToPrettyTree for &T where T: ToPrettyTree {
    fn to_pretty_tree(&self) -> PrettyTree { (*self).to_pretty_tree() }
}
// impl<Key, Value> ToPrettyTree for (Key, Value) where Key: ToString, Value: ToPrettyTree {
//     fn to_pretty_tree(&self) -> PrettyTree {
//         PrettyTree::branch_of(self.0.to_string(), &[ self.1.to_pretty_tree() ])
//     }
// }
impl ToPrettyTree for &str {
    fn to_pretty_tree(&self) -> PrettyTree {
        PrettyTree::string(self)
    }
}
impl<T: ToPrettyTree> ToPrettyTree for Option<T> {
    fn to_pretty_tree(&self) -> PrettyTree {
        match self {
            Self::None => PrettyTree::Value(String::from("None")),
            Self::Some(x) => x.to_pretty_tree(),
        }
    }
}
impl<T: ToPrettyTree> ToPrettyTree for Vec<T> {
    fn to_pretty_tree(&self) -> PrettyTree {
        let name = format!(
            "Vec<{}>",
            std::any::type_name::<T>()
        );
        let children = self.iter().map(ToPrettyTree::to_pretty_tree).collect::<Vec<_>>();
        if children.is_empty() {
            return PrettyTree::key_value(name, PrettyTree::Value(String::from("[]")))
        }
        // PrettyTree::branch_of(name, &children)
        PrettyTree::List(crate::PrettyList {
            name: Some(name),
            nodes: children,
        })
    }
}
impl<T: ToPrettyTree> ToPrettyTree for &[T] {
    fn to_pretty_tree(&self) -> PrettyTree {
        let name = std::any::type_name::<Self>();
        let children = self.iter().map(ToPrettyTree::to_pretty_tree).collect::<Vec<_>>();
        PrettyTree::branch_of(name, &children)
    }
}
impl<Key: ToString, Value: ToPrettyTree> ToPrettyTree for HashMap<Key, Value> {
    fn to_pretty_tree(&self) -> PrettyTree {
        let name = std::any::type_name::<Self>();
        let children = self
            .iter()
            .map(|(key, value)| {
                PrettyTree::branch_of(key.to_string(), &[ value.to_pretty_tree() ])
            })
            .collect::<Vec<_>>();
        PrettyTree::branch_of(name, &children)
    }
}
impl<Key: ToString, Value: ToPrettyTree> ToPrettyTree for BTreeMap<Key, Value> {
    fn to_pretty_tree(&self) -> PrettyTree {
        let name = std::any::type_name::<Self>();
        let children = self
            .iter()
            .map(|(key, value)| {
                PrettyTree::branch_of(key.to_string(), &[ value.to_pretty_tree() ])
            })
            .collect::<Vec<_>>();
        PrettyTree::branch_of(name, &children)
    }
}
impl<T: ToPrettyTree> ToPrettyTree for HashSet<T> {
    fn to_pretty_tree(&self) -> PrettyTree {
        let name = std::any::type_name::<Self>();
        let children = self.iter().map(ToPrettyTree::to_pretty_tree).collect::<Vec<_>>();
        PrettyTree::branch_of(name, &children)
    }
}
impl<T: ToPrettyTree> ToPrettyTree for BTreeSet<T> {
    fn to_pretty_tree(&self) -> PrettyTree {
        let name = std::any::type_name::<Self>();
        let children = self.iter().map(ToPrettyTree::to_pretty_tree).collect::<Vec<_>>();
        PrettyTree::branch_of(name, &children)
    }
}

impl<A: ToPrettyTree, B: ToPrettyTree> ToPrettyTree for (A, B) {
    fn to_pretty_tree(&self) -> PrettyTree {
        let name = format!(
            "({}, {})",
            std::any::type_name::<A>(),
            std::any::type_name::<B>(),
        );
        PrettyTree::branch_of(name, &[
            self.0.to_pretty_tree(),
            self.1.to_pretty_tree(),
        ])
    }
}
impl<A: ToPrettyTree, B: ToPrettyTree, C: ToPrettyTree> ToPrettyTree for (A, B, C) {
    fn to_pretty_tree(&self) -> PrettyTree {
        let name = format!(
            "({}, {})",
            std::any::type_name::<A>(),
            std::any::type_name::<B>(),
        );
        PrettyTree::branch_of(name, &[
            self.0.to_pretty_tree(),
            self.1.to_pretty_tree(),
            self.2.to_pretty_tree(),
        ])
    }
}

#[cfg(feature = "serde_json")]
impl ToPrettyTree for serde_json::Value {
    fn to_pretty_tree(&self) -> PrettyTree {
        match self {
            Self::Null => PrettyTree::str("Null"),
            Self::Bool(x) => PrettyTree::string(format!("Bool({x})")),
            Self::Number(x) => PrettyTree::string(format!("Number({x})")),
            Self::String(x) => PrettyTree::string(format!("String({x})")),
            Self::Array(xs) => PrettyTree::some_branch(PrettyBranch::from_iter("Array", xs.clone())),
            Self::Object(xs) => PrettyTree::some_branch(PrettyBranch::from_iter("Object", xs)),
        }
    }
}

#[cfg(feature = "indexmap")]
impl<Key: ToString, Value: ToPrettyTree> ToPrettyTree for indexmap::IndexMap<Key, Value> {
    fn to_pretty_tree(&self) -> PrettyTree {
        let name = std::any::type_name::<Self>();
        let children = self
            .iter()
            .map(|(key, value)| (key.to_string(), value.to_pretty_tree()))
            .collect::<Vec<_>>();
        PrettyTree::branch_of(name, children)
    }
}

#[cfg(feature = "indexmap")]
impl<Type: ToPrettyTree> ToPrettyTree for indexmap::IndexSet<Type> {
    fn to_pretty_tree(&self) -> PrettyTree {
        let name = std::any::type_name::<Self>();
        let children = self
            .iter()
            .map(|x| x.to_pretty_tree())
            .collect::<Vec<_>>();
        PrettyTree::branch_of(name, children)
    }
}
