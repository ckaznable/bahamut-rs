use std::fmt::Display;

use scraper::{Selector, ElementRef};

#[derive(Clone)]
pub enum UserRace {
    Human,
    Elf,
    Dwarf,
    Ork,
    Unknown,
}

impl Display for UserRace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserRace::Human => write!(f, "人類"),
            UserRace::Elf => write!(f, "妖精"),
            UserRace::Dwarf => write!(f, "矮人"),
            UserRace::Ork => write!(f, "獸人"),
            UserRace::Unknown => write!(f, "未知"),
        }
    }
}

#[derive(Clone)]
pub struct User {
    pub id: String,
    pub race: UserRace,
    pub name: String,
    pub lv: u8,
}

impl Default for User {
    fn default() -> Self {
        let empty = "";
        User {
            id: empty.to_string(),
            race: UserRace::Dwarf,
            name: empty.to_string(),
            lv: 1,
        }
    }
}

impl User {
    fn try_level_from_html(document: &ElementRef) -> Option<u8> {
        let selector = Selector::parse(".userlevel").unwrap();
        let lv = document
            .select(&selector)
            .next()?
            .text()
            .last()?
            .to_string();

        lv.trim().parse::<u8>().map_or(Some(0u8), |x|Some(x))
    }

    fn try_id_from_html(document: &ElementRef) -> Option<String> {
        let selector = Selector::parse(".c-user__avatar").unwrap();
        let id = document
            .select(&selector)
            .next()?
            .value()
            .attr("data-gamercard-userid")?
            .to_string();

        Some(id)
    }

    fn try_name_from_html(document: &ElementRef) -> Option<String> {
        let selector = Selector::parse(".username").unwrap();
        let name = document
            .select(&selector)
            .next()?
            .text()
            .next()?
            .to_string();

        Some(name)
    }

    fn try_race_from_html(document: &ElementRef) -> Option<UserRace> {
        let selector = Selector::parse(".userrace").unwrap();
        let race = document
            .select(&selector)
            .next()?
            .value()
            .attr("data-original-title")?;

        let race = match race {
            "人類" => UserRace::Human,
            "矮人" => UserRace::Dwarf,
            "妖精" => UserRace::Elf,
            "獸人" => UserRace::Ork,
            _ => UserRace::Unknown
        };

        Some(race)
    }
}

impl TryFrom<&ElementRef<'_>> for User {
    type Error = &'static str;

    fn try_from(document: &ElementRef) -> Result<Self, Self::Error> {
        let user = User {
            lv: User::try_level_from_html(&document).ok_or("user lv invalid")?,
            id: User::try_id_from_html(&document).ok_or("user id invalid")?,
            name: User::try_name_from_html(&document).ok_or("user name invalid")?,
            race: User::try_race_from_html(&document).map_or(UserRace::Unknown, |x|x),
        };

        Ok(user)
    }
}