use crate::{PrettyBranch, PrettyFragment, PrettyTree};
use colored::Colorize;

#[derive(Debug, Clone)]
pub struct Formatter {
    columns: Vec<TreeColumn>,
    style: FormatterStyle,
}

impl Formatter {
    pub const COLUMN_LENGTH: usize = 4;
    pub fn new(style: FormatterStyle) -> Self {
        Self { columns: Default::default(), style }
    }
    pub fn map_formatter_style(self, f: impl FnOnce(FormatterStyle) -> FormatterStyle) -> Self {
        Self { columns: self.columns, style: f(self.style) }
    }
}
impl Default for Formatter {
    fn default() -> Self {
        Self {
            columns: Default::default(),
            style: FormatterStyle::default(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct FormatterStyle {
    use_color: bool,
    compact_mode: bool,
}

impl FormatterStyle {
    pub fn use_color(self, color: bool) -> Self {
        Self { use_color: color, compact_mode: self.compact_mode }
    }
    pub fn compact_mode(self, compact_mode: bool) -> Self {
        Self { use_color: self.use_color, compact_mode }
    }
}

impl Default for FormatterStyle {
    fn default() -> Self {
        Self { use_color: false, compact_mode: false }
    }
}

#[derive(Debug, Clone)]
pub(crate) enum TreeColumn {
    UpThenRight,
    VerticalBar,
    DownAndRight,
    DownThenRight,
    Empty,
}

impl std::fmt::Display for TreeColumn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UpThenRight => write!(f, "╭"),
            Self::VerticalBar => write!(f, "│"),
            Self::DownAndRight => write!(f, "├"),
            Self::DownThenRight => write!(f, "╰"),
            Self::Empty => write!(f, " ")
        }
    }
}

impl FormatterStyle {
    fn color(&self, depth: usize, string: impl ToString) -> impl ToString {
        let string = string.to_string();
        if !self.use_color {
            return string.into()
        }
        match depth % 4 {
            0 => string.truecolor(255, 20, 165), // PINK
            1 => string.truecolor(252, 255, 87), // YELLOW
            2 => string.truecolor(0, 255, 0), // GREEN
            _ => string.truecolor(102, 255, 252), // BLUE
        }
    }
}

impl Formatter {
    fn down_then_right(&self) -> Self {
        let mut columns = self.columns
            .clone()
            .into_iter()
            .map(|x| match x {
                TreeColumn::DownAndRight => TreeColumn::VerticalBar,
                TreeColumn::DownThenRight => TreeColumn::Empty,
                x => x
            })
            .collect::<Vec<_>>();
        columns.push(TreeColumn::DownThenRight);
        Self { columns, style: self.style }
    }
    fn down_and_right(&self) -> Self {
        let mut columns = self.columns
            .clone()
            .into_iter()
            .map(|x| match x {
                TreeColumn::DownAndRight => TreeColumn::VerticalBar,
                TreeColumn::DownThenRight => TreeColumn::Empty,
                x => x
            })
            .collect::<Vec<_>>();
        columns.push(TreeColumn::DownAndRight);
        Self { columns, style: self.style }
    }
    fn up_then_right(&self) -> Self {
        let mut columns = self.columns
            .clone()
            .into_iter()
            .map(|x| match x {
                TreeColumn::DownAndRight => TreeColumn::VerticalBar,
                TreeColumn::DownThenRight => TreeColumn::Empty,
                x => x
            })
            .collect::<Vec<_>>();
        columns.push(TreeColumn::UpThenRight);
        Self { columns, style: self.style }
    }
    fn with_column(&self, column: TreeColumn) -> Self {
        let mut columns = self.columns.clone();
        columns.push(column);
        Self { columns, style: self.style }
    }
    fn replace_last_column(mut self, column: TreeColumn) -> Self {
        if let Some(last) = self.columns.last_mut() {
            *last = column;
        }
        self
    }
    fn drop_last_column(mut self) -> Self {
        self.columns.pop();
        self
    }
    fn leading(&self) -> impl ToString {
        let depth = self.columns.len();
        let thin_space = "\u{2009}";
        let leading = self.columns
            .iter()
            .enumerate()
            .map(|(ix, c)| self.style.color(ix, c).to_string())
            .collect::<Vec<_>>()
            .join("  ");
        let sep = if self.columns.is_empty() {
            String::default()
        } else {
            let depth = if depth > 1 { depth - 1 } else { 0 };
            self.style.color(depth, format!("╼{thin_space}")).to_string()
        };
        format!("{leading}{sep}").dimmed()
    }
    fn leaf(&self, value: impl ToString) -> String {
        let value = value.to_string();
        let depth = self.columns.len();
        let leading = self.leading().to_string();
        let trailing = self.style.color(depth, value).to_string();
        format!("{leading}{trailing}")
    }
    fn branch(&self, label: impl ToString, children: &[PrettyTree]) -> String {
        let label = self.leaf(label);
        if children.is_empty() {
            return label
        }
        if children.len() == 1 {
            let child = children
                .first()
                .unwrap()
                .format(&self.down_then_right());
            return format!("{label}\n{child}")
        }
        let child_count = children.len();
        let last_child_index = child_count - 1;
        let children = children
            .iter()
            .enumerate()
            .map(|(ix, child)| {
                let is_first = ix == 0;
                let is_last = ix == last_child_index;
                if is_first {
                    return child.format(&self.down_and_right())
                }
                if is_last {
                    return child.format(&self.down_then_right())
                }
                return child.format(&self.down_and_right())
            })
            .collect::<Vec<_>>()
            .join("\n");
        format!("{label}\n{children}")
    }
    fn list(&self, label: Option<&str>, list: &[PrettyTree]) -> String {
        let compact_mode = self.style.compact_mode;
        let label = label.map(|x| self.leaf(x));
        match label {
            Some(label) if list.is_empty() => return label.to_owned(),
            Some(label) if list.len() == 1 => {
                let child = list
                    .first()
                    .unwrap()
                    .format(&self.down_then_right());
                return format!("{label}\n{child}")
            }
            None if list.len() == 1 => return list.first().unwrap().format(self),
            None if list.is_empty() => return "[]".to_owned(),
            _ => ()
        }
        match label {
            Some(label) => {
                let child_count = list.len();
                let last_child_index = child_count - 1;
                let children = list
                    .iter()
                    .enumerate()
                    .map(|(ix, child)| {
                        let is_first = ix == 0;
                        let is_last = ix == last_child_index;
                        if is_first {
                            return child.format(&self.down_and_right())
                        }
                        if is_last {
                            return child.format(&self.down_then_right())
                        }
                        return child.format(&self.down_and_right())
                    })
                    .collect::<Vec<_>>()
                    .join("\n");
                if compact_mode {
                    format!("{children}")
                } else {
                    format!("{label}\n{children}")
                }
            }
            None => {
                let child_count = list.len();
                let last_child_index = child_count - 1;
                let children = list
                    .iter()
                    .enumerate()
                    .map(|(ix, child)| {
                        let is_first = ix == 0;
                        let is_last = ix == last_child_index;
                        if is_first {
                            return child.format(&self.up_then_right())
                        }
                        if is_last {
                            return child.format(&self.down_then_right())
                        }
                        return child.format(&self.down_and_right())
                    })
                    .collect::<Vec<_>>()
                    .join("\n");
                return children
            }
        }
    }
    // fn fragment(&self, list: &[PrettyTree]) -> String {
    //     self.list(None, list)
    // }
    fn fragment(&self, list: &[PrettyTree]) -> String {
        if list.len() == 1 {
            return list.first().unwrap().format(self);
        }
        let child_count = list.len();
        let last_child_index = child_count - 1;
        list.iter()
            .enumerate()
            .map(|(ix, child)| {
                let is_last = ix == last_child_index;
                if is_last {
                    child.format(&self.down_then_right())
                } else {
                    child.format(&self.down_and_right())
                }
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}

impl PrettyTree {
    pub fn format(&self, formatter: &Formatter) -> String {
        match self {
            Self::Empty => String::default(),
            Self::Value(x) => formatter.leaf(x),
            Self::String(x) => formatter.leaf(format!("{x:?}")),
            Self::Branch(x) => x.format(formatter),
            Self::Fragment(x) => x.format(formatter),
            // Self::List(x) => x.format(formatter),
        }
    }
    pub fn render(&self) -> String {
        self.format(&Default::default())
    }
}
impl PrettyBranch {
    pub fn format(&self, formatter: &Formatter) -> String {
        formatter.branch(&self.label, &self.children)
    }
}
impl PrettyFragment {
    pub fn format(&self, formatter: &Formatter) -> String {
        formatter.fragment(&self.nodes)
    }
}
// impl PrettyList {
//     pub fn format(&self, formatter: &Formatter) -> String {
//         // if let Some(name) = self.name.as_ref() {
//         //     return formatter.branch(name, &self.nodes)
//         // }
//         // return formatter.fragment(&self.nodes)
//         formatter.list(self.data_type.as_ref().map(|x| x.as_str()), &self.nodes)
//     }
// }
impl std::fmt::Display for PrettyTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.format(&Default::default()))
    }
}