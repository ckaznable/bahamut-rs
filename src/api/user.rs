use std::fmt::Display;

pub enum UserClass {
    Person,
    Elf,
    Dwarf,
    Ork,
}

impl Display for UserClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserClass::Person => write!(f, "人類"),
            UserClass::Elf => write!(f, "精靈"),
            UserClass::Dwarf => write!(f, "矮人"),
            UserClass::Ork => write!(f, "獸人"),
        }
    }
}

pub struct User {
    id: String,
    class: UserClass,
    name: String,
    lv: u8,
    title: String,
}

impl Default for User {
    fn default() -> Self {
        let empty = "";
        User {
            id: empty.to_string(),
            class: UserClass::Dwarf,
            name: empty.to_string(),
            lv: 1,
            title: empty.to_string(),
        }
    }
}