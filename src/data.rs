use std::{collections::{BTreeMap, BTreeSet, HashMap, HashSet}, fmt::Display};

use crate::ToPrettyTree;

#[derive(Debug, Clone)]
pub enum PrettyTree {
    Empty,
    /// A terminal leaf node.
    Value(String),
    /// A terminal leaf node.
    String(String),
    /// A branch node.
    Branch(PrettyBranch),
    /// A fragment node.
    Fragment(PrettyFragment),
}

impl PrettyTree {
    pub fn empty() -> Self { Self::Empty }
    pub fn value(value: impl ToString) -> Self {
        let value = value.to_string();
        Self::Value(format!("{value}"))
    }
    pub fn string<T: ToString>(value: T) -> Self {
        let value = value.to_string();
        Self::Value(format!("{value:?}"))
    }
    pub fn str(value: impl AsRef<str>) -> Self {
        let value = value.as_ref();
        Self::Value(format!("{value:?}"))
    }
    pub fn leaf(value: impl AsRef<str>) -> Self {
        Self::Value(value.as_ref().to_owned())
    }
    pub fn fragment<T: ToPrettyTree>(list: impl IntoIterator<Item = T>) -> Self {
        Self::Fragment(PrettyFragment { nodes: list.into_iter().map(|x| x.to_pretty_tree()).collect() })
    }
    pub fn branch_of<Type: ToPrettyTree>(
        label: impl AsRef<str>,
        children: impl IntoIterator<Item = Type>
    ) -> Self {
        Self::Branch(PrettyBranch {
            label: label.as_ref().to_string(),
            children: children.into_iter().map(|x| x.to_pretty_tree()).collect(),
        })
    }
    pub fn key_value(
        key: impl AsRef<str>,
        value: impl ToPrettyTree
    ) -> Self {
        let key = key.as_ref().to_string();
        match value.to_pretty_tree() {
            PrettyTree::Value(text) => PrettyTree::Value(format!("{key}: {text}")),
            PrettyTree::String(text) => PrettyTree::Value(format!("{key}: {text:?}")),
            tree => {
                Self::Branch(PrettyBranch {
                    label: key,
                    children: vec![ tree ],
                })
            }
        }

    }
    pub fn some_value(value: impl Into<PrettyValue>) -> Self {
        Self::Value(value.into().0)
    }
    pub fn some_branch(branch: impl Into<PrettyBranch>) -> Self {
        Self::Branch(branch.into())
    }
    pub fn some_fragment(fragment: impl Into<PrettyFragment>) -> Self {
        Self::Fragment(fragment.into())
    }
}

impl Default for PrettyTree {
    fn default() -> Self { PrettyTree::Empty }
}

#[derive(Debug, Clone)]
pub struct PrettyValue(String);

impl PrettyValue {
    pub fn from_str(value: impl AsRef<str>) -> Self {
        Self(value.as_ref().to_string())
    }
    pub fn from_string(value: impl Display) -> Self {
        Self(value.to_string())
    }
}

#[derive(Debug, Clone)]
pub struct PrettyBranch {
    pub label: String,
    pub children: Vec<PrettyTree>,
}

impl PrettyBranch {
    pub fn new(label: impl AsRef<str>) -> Self {
        let label = label.as_ref().to_string();
        Self { label, children: Vec::default() }
    }
    pub fn with_child(&self, child: impl ToPrettyTree) -> Self {
        let mut copy = self.clone();
        copy.children.push(child.to_pretty_tree());
        copy
    }
    pub fn with_child_ref<T: ToPrettyTree>(&self, child: &T) -> Self {
        let mut copy = self.clone();
        copy.children.push(child.to_pretty_tree());
        copy
    }

    pub fn with_field(&self, key: impl AsRef<str>, child: impl ToPrettyTree) -> Self {
        let mut copy = self.clone();
        copy.children.push(PrettyTree::key_value(key, child.to_pretty_tree()));
        copy
    }
    pub fn with_field_ref<T: ToPrettyTree>(&self, key: impl AsRef<str>, child: &T) -> Self {
        let mut copy = self.clone();
        copy.children.push(PrettyTree::key_value(key, child.to_pretty_tree()));
        copy
    }

    pub fn with_children<T: ToPrettyTree>(&self, children: impl IntoIterator<Item=T>) -> Self {
        let mut copy = self.clone();
        let children = children.into_iter().map(|x| x.to_pretty_tree());
        copy.children.extend(children);
        copy
    }
    pub fn with_children_slice<T: ToPrettyTree>(&self, children: impl AsRef<[T]>) -> Self {
        let mut copy = self.clone();
        let children = children.as_ref().into_iter().map(|x| x.to_pretty_tree());
        copy.children.extend(children);
        copy
    }
    pub fn with_children_iter<'a, T: ToPrettyTree + Clone + 'a>(&self, children: impl IntoIterator<Item=&'a T>) -> Self {
        let mut copy = self.clone();
        let children = children.into_iter().cloned().map(|x| x.to_pretty_tree());
        copy.children.extend(children);
        copy
    }
    pub fn from_iter<Child: ToPrettyTree>(
        label: impl ToString,
        children: impl IntoIterator<Item = Child>
    ) -> Self {
        PrettyBranch {
            label: label.to_string(),
            children: children.into_iter().map(|x| x.to_pretty_tree()).collect(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PrettyFragment {
    pub nodes: Vec<PrettyTree>
}

impl PrettyFragment {
    pub fn from_iter<Value: ToPrettyTree>(list: impl IntoIterator<Item = Value>) -> Self {
        Self { nodes: list.into_iter().map(|x| x.to_pretty_tree()).collect() }
    }
}
