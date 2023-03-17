use std::fmt::Display;

pub enum UserKind {
    Person,
    Elf,
    Dwarf,
    Ork,
}

impl Display for UserKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserKind::Person => write!(f, "人類"),
            UserKind::Elf => write!(f, "精靈"),
            UserKind::Dwarf => write!(f, "矮人"),
            UserKind::Ork => write!(f, "獸人"),
        }
    }
}

pub struct User {
    id: String,
    kind: UserKind,
    name: String,
    lv: u8,
    title: String,
}

impl Default for User {
    fn default() -> Self {
        let empty = "";
        User {
            id: empty.to_string(),
            kind: UserKind::Dwarf,
            name: empty.to_string(),
            lv: 1,
            title: empty.to_string(),
        }
    }
}
