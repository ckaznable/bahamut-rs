use std::fmt::Display;

use scraper::{Selector, ElementRef};
use serde::Serialize;

#[derive(Clone, Serialize)]
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

#[derive(Clone, Serialize)]
pub enum UserCareer {
    Noob,
    Sword,
    Archer,
    Preist,
    Wizard,
    Fighter,
    Business,
    Assassin,
    Unknown,
}

impl Display for UserCareer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserCareer::Noob => write!(f, "初心者"),
            UserCareer::Sword => write!(f, "劍士"),
            UserCareer::Archer => write!(f, "弓箭手"),
            UserCareer::Preist => write!(f, "僧侶"),
            UserCareer::Wizard => write!(f, "法師"),
            UserCareer::Fighter => write!(f, "武鬥家"),
            UserCareer::Business => write!(f, "商人"),
            UserCareer::Assassin => write!(f, "盜賊"),
            UserCareer::Unknown => write!(f, "未知"),
        }
    }
}

#[derive(Clone, Serialize)]
pub struct User {
    pub id: String,
    pub race: UserRace,
    pub carrer: UserCareer,
    pub name: String,
    pub lv: u8,
}

impl Default for User {
    fn default() -> Self {
        let empty = "";
        User {
            id: empty.to_string(),
            race: UserRace::Dwarf,
            carrer: UserCareer::Noob,
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

        lv.trim().parse::<u8>().map_or(Some(0u8), Some)
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
        let selector = Selector::parse(".userrace img").unwrap();
        let race = document
            .select(&selector)
            .next()?
            .value()
            .attr("src")?
            .split('/')
            .last()?;

        let race = match race {
            "human.png" => UserRace::Human,
            "hobbit.png" => UserRace::Dwarf,
            "fairy.png" => UserRace::Elf,
            "orc.png" => UserRace::Ork,
            _ => UserRace::Unknown
        };

        Some(race)
    }

    fn try_crarrer_from_html(document: &ElementRef) -> Option<UserCareer> {
        let selector = Selector::parse(".usercareer img").unwrap();
        let crarrer = document
            .select(&selector)
            .next()?
            .value()
            .attr("src")?
            .split('/')
            .last()?;

        let crarrer = match crarrer {
            "noob.png" => UserCareer::Noob,
            "sword.png" => UserCareer::Sword,
            "fighter.png" => UserCareer::Fighter,
            "business.png" => UserCareer::Business,
            "wizard.png" => UserCareer::Wizard,
            "preist.png" => UserCareer::Preist,
            "archer.png" => UserCareer::Archer,
            "assassin.png" => UserCareer::Assassin,
            _ => UserCareer::Unknown
        };

        Some(crarrer)
    }
}

impl TryFrom<&ElementRef<'_>> for User {
    type Error = &'static str;

    fn try_from(document: &ElementRef) -> Result<Self, Self::Error> {
        let user = User {
            lv: User::try_level_from_html(document).ok_or("user lv invalid")?,
            id: User::try_id_from_html(document).ok_or("user id invalid")?,
            name: User::try_name_from_html(document).ok_or("user name invalid")?,
            race: User::try_race_from_html(document).map_or(UserRace::Unknown, |x|x),
            carrer: User::try_crarrer_from_html(document).map_or(UserCareer::Unknown, |x|x),
        };

        Ok(user)
    }
}