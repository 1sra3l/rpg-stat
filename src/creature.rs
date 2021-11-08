/*!
# Creature Types

This encompasses all the different humanoids, as well as enemy creatures, and even pets

```
use rpgstat::stats::Basic as Stats;
use rpgstat::class::Basic as Class;
use rpgstat::creature::Animal;
use rpgstat::stats::Builder;

let bear:Animal = Animal::Bear;
// this number only matters if you want
let id:f64 = 0.0;
// this effects the stats returned
let level:f64 = 1.0;
// use the basic `Builder`
let bear_stats:Stats<f64> = bear.build_basic(id, level);
```


*/
use std::fmt;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use std::ops::{Add, AddAssign,  Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign};
use serde::{Deserialize, Serialize};

use crate::stats::{ Builder, Basic, Normal, Advanced };

#[derive(Clone, PartialEq, Copy, Debug, EnumIter, Serialize, Deserialize)]
/// The Person class of creature types
pub enum Person {
    /// These little guys rock!
    Dwarf,
    /// What would a heroic journey be without an elf or two
    Elf,
    /// Winged small humanoids
    Fairy,
    /// Large brutes often subjugating humans
    Giant,
    /// Little lovers of nature and engineering
    Gnome,
    /// Obviously we'd like to be heroic
    Human,
    /// Mermaid and Merman
    Mer,
    /// Shape shifter
    Selkie,
    /// If you end up with a nagging one, sorry, but they are super helpful in a boss fight!
    Sprite,
}
impl Default for Person {
    fn default() -> Self {
        Self::Human
    }
}
impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let v:String;
        match *self {
            Person::Dwarf => v = String::from("Dwarf"),
            Person::Elf => v = String::from("Elf"),
            Person::Fairy => v = String::from("Fairy"),
            Person::Giant => v = String::from("Giant"),
            Person::Gnome => v = String::from("Gnome"),
            Person::Human => v = String::from("Human"),
            Person::Mer => v = String::from("Mer"),
            Person::Selkie => v = String::from("Selkie"),
            Person::Sprite => v = String::from("Sprite"),
        }
        write!(f, "{}", v.as_str())
    }
}
///  The various monsters
#[derive(Clone, PartialEq, Copy, Debug, EnumIter)]
pub enum Monster {
    Dragon,
    Golem,
    Ogre,
    Orc,
    Undead,
    Werewolf,
    Yeti,
}


#[derive(Clone, PartialEq, Copy, Debug, EnumIter)]
/// The various animals you encounter
pub enum Animal {
    /// a crocodile
    Crocodile,
    /// a bear
    Bear,
    /// a bird
    Bird,

    //Boar,Dog,Fox,

    /// an insect
    Insect,
    /// a lion
    Lion,
    /// a rabbit
    Rabbit,
    /// a rat
    Rat,
    /// a snake
    Snake,
    /// a tiger
    Tiger,
    /// a wolf
    Wolf,
}
impl Default for Animal {
    fn default() -> Self {
        Self::Rat
    }
}
impl fmt::Display for Animal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let v:String;
        // Animal:: => v = String::from(""),
        match *self {
            Animal::Crocodile => v = String::from("Crocodile"),
            Animal::Bear => v = String::from("Bear"),
            Animal::Bird => v = String::from("Bird"),
            Animal::Insect => v = String::from("Insect"),
            Animal::Lion => v = String::from("Lion"),
            Animal::Rabbit => v = String::from("Rabbit"),
            Animal::Rat => v = String::from("Rat"),
            Animal::Snake => v = String::from("Snake"),
            Animal::Tiger => v = String::from("Tiger"),
            Animal::Wolf => v = String::from("Wolf"),
            
        }
        write!(f, "{}", v.as_str())
    }
}
impl<T:Copy 
    + Default
    + AddAssign
    + Add<Output = T>
    + Div<Output = T>
    + DivAssign
    + Mul<Output = T>
    + MulAssign
    + Neg<Output = T>
    + Rem<Output = T>
    + RemAssign
    + Sub<Output = T>
    + SubAssign
    + std::cmp::PartialOrd
    + num::NumCast> Builder<T> for Animal {
    /// Build a `Basic` stat
    fn build_basic(&self, id:T, level:T) -> Basic<T>{
        let mut hp:T = num::cast(10).unwrap();
        let mut mp:T = num::cast(5).unwrap();
        let mut xp:T = num::cast(1).unwrap();
        let mut xp_next:T = num::cast(10).unwrap();
        let mut gp:T = num::cast(5).unwrap();
        let mut speed:T = num::cast(5).unwrap();
        //TODO OR ue legendary.ini + serde
        match *self {
            Animal::Rat => {
                gp += num::cast(2).unwrap();
            }
            Animal::Snake => {
                speed += num::cast(2).unwrap();
                mp += num::cast(1).unwrap();
                hp += num::cast(1).unwrap();
                xp += num::cast(1).unwrap();
                gp += num::cast(1).unwrap();
            }
            Animal::Rabbit => {
                speed += num::cast(2).unwrap();
                hp += num::cast(2).unwrap();
                mp += num::cast(2).unwrap();
                xp += num::cast(2).unwrap();
            }
            Animal::Wolf => {
                speed += num::cast(3).unwrap();
                hp += num::cast(3).unwrap();
                mp += num::cast(3).unwrap();
                xp += num::cast(3).unwrap();
                gp += num::cast(2).unwrap();
            }
            Animal::Crocodile => {
                speed -= num::cast(1).unwrap();
                hp += num::cast(5).unwrap();
                mp += num::cast(5).unwrap();
                xp += num::cast(5).unwrap();
                gp += num::cast(4).unwrap();
            }
            Animal::Insect => {
                hp -= num::cast(2).unwrap();
                mp += num::cast(4).unwrap();
            }
            Animal::Lion => {
                hp += num::cast(10).unwrap();
                speed += num::cast(7).unwrap();
                mp += num::cast(4).unwrap();
                xp += num::cast(9).unwrap();
            }
            Animal::Tiger => {
                hp += num::cast(9).unwrap();
                speed += num::cast(9).unwrap();
                mp += num::cast(7).unwrap();
                xp += num::cast(7).unwrap();
            }
            Animal::Bear => {
                hp += num::cast(15).unwrap();
                mp += num::cast(10).unwrap();
                xp += num::cast(10).unwrap();
            }
            Animal::Bird => {
                hp += num::cast(2).unwrap();
                mp += num::cast(12).unwrap();
                xp += num::cast(7).unwrap();
            }
            
        }
        hp *= level;
        mp *= level;
        // TODO fixme:
        xp *= level;
        // TODO fixme:
        xp_next *= level;
        gp *= level;
        speed += level;
        Basic {
            id:id,
            xp:xp,
            xp_next:xp_next,
            level:level,
            gp:gp,
            hp: hp,
            mp: mp,
            hp_max: hp,
            mp_max: mp,
            speed: speed,
        }
        
    }
    // Build a `Normal` stat
    fn build_normal(&self, id:T, level:T) -> Normal<T>{
        let mut hp:T = num::cast(10).unwrap();
        let mut mp:T = num::cast(5).unwrap();
        let mut xp:T = num::cast(1).unwrap();
        let mut xp_next:T = num::cast(10).unwrap();
        let mut gp:T = num::cast(5).unwrap();
        let mut speed:T = num::cast(5).unwrap();
        let mut atk:T = num::cast(10).unwrap();
        let mut def:T = num::cast(10).unwrap();
        let mut m_atk:T = num::cast(10).unwrap();
        let mut m_def:T = num::cast(10).unwrap();
        //TODO OR use legendary.ini + serde
        match *self {
            Animal::Rat => {
                gp += num::cast(2).unwrap();
            }
            Animal::Snake => {
                speed += num::cast(2).unwrap();
                mp += num::cast(1).unwrap();
                hp += num::cast(1).unwrap();
                xp += num::cast(1).unwrap();
                gp += num::cast(1).unwrap();
                atk += num::cast(1).unwrap();
                def += num::cast(1).unwrap();
                m_atk += num::cast(1).unwrap();
                m_def += num::cast(1).unwrap();
            }
            Animal::Rabbit => {
                speed += num::cast(2).unwrap();
                hp += num::cast(2).unwrap();
                mp += num::cast(2).unwrap();
                xp += num::cast(2).unwrap();
                atk += num::cast(2).unwrap();
                def += num::cast(2).unwrap();
                m_atk += num::cast(2).unwrap();
                m_def += num::cast(7).unwrap();
            }
            Animal::Wolf => {
                speed += num::cast(3).unwrap();
                hp += num::cast(3).unwrap();
                mp += num::cast(3).unwrap();
                xp += num::cast(3).unwrap();
                gp += num::cast(2).unwrap();
                atk += num::cast(10).unwrap();
                def += num::cast(5).unwrap();
                m_atk += num::cast(3).unwrap();
                m_def += num::cast(5).unwrap();
            }
            Animal::Crocodile => {
                speed -= num::cast(1).unwrap();
                hp += num::cast(5).unwrap();
                mp += num::cast(5).unwrap();
                xp += num::cast(5).unwrap();
                gp += num::cast(4).unwrap();
                atk += num::cast(7).unwrap();
                def += num::cast(7).unwrap();
                m_atk += num::cast(3).unwrap();
                m_def += num::cast(1).unwrap();
            }
            Animal::Insect => {
                hp -= num::cast(2).unwrap();
                mp += num::cast(4).unwrap();
                atk += num::cast(2).unwrap();
                def += num::cast(3).unwrap();
                m_atk += num::cast(1).unwrap();
                m_def += num::cast(1).unwrap();
            }
            Animal::Lion => {
                hp += num::cast(10).unwrap();
                speed += num::cast(7).unwrap();
                mp += num::cast(4).unwrap();
                xp += num::cast(9).unwrap();
                atk += num::cast(11).unwrap();
                def += num::cast(9).unwrap();
                m_atk += num::cast(5).unwrap();
                m_def += num::cast(8).unwrap();
            }
            Animal::Tiger => {
                hp += num::cast(9).unwrap();
                speed += num::cast(9).unwrap();
                mp += num::cast(7).unwrap();
                xp += num::cast(7).unwrap();
                atk += num::cast(12).unwrap();
                def += num::cast(5).unwrap();
                m_atk += num::cast(5).unwrap();
                m_def += num::cast(12).unwrap();
            }
            Animal::Bear => {
                hp += num::cast(15).unwrap();
                mp += num::cast(10).unwrap();
                xp += num::cast(10).unwrap();
                atk += num::cast(18).unwrap();
                def += num::cast(12).unwrap();
                m_atk += num::cast(2).unwrap();
                m_def += num::cast(5).unwrap();
            }
            Animal::Bird => {
                hp += num::cast(2).unwrap();
                mp += num::cast(12).unwrap();
                xp += num::cast(7).unwrap();
                atk += num::cast(11).unwrap();
                def += num::cast(1).unwrap();
                m_atk += num::cast(8).unwrap();
                m_def += num::cast(10).unwrap();
                speed += num::cast(9).unwrap();
            }
            
        }
        hp *= level;
        mp *= level;
        // TODO fixme:
        xp *= level;
        // TODO fixme:
        xp_next *= level;
        gp *= level;
        speed += level;
        Normal {
            id:id,
            xp:xp,
            xp_next:xp_next,
            level:level,
            gp:gp,
            hp: hp,
            mp: mp,
            hp_max: hp,
            mp_max: mp,
            speed: speed,
            atk:atk,
            def:def,
            m_atk:m_atk,
            m_def:m_def,
        }
    }

    // Build an `Advanced` stat
    fn build_advanced(&self, id:T, level:T) -> Advanced<T>{
        let mut hp:T = num::cast(10).unwrap();
        let mut mp:T = num::cast(5).unwrap();
        let mut xp:T = num::cast(1).unwrap();
        let mut xp_next:T = num::cast(10).unwrap();
        let mut gp:T = num::cast(5).unwrap();
        let mut speed:T = num::cast(5).unwrap();
        let mut atk:T = num::cast(10).unwrap();
        let mut def:T = num::cast(10).unwrap();
        let mut m_atk:T = num::cast(10).unwrap();
        let mut m_def:T = num::cast(10).unwrap();
        let mut agility:T = num::cast(10).unwrap();
        let mut strength:T = num::cast(10).unwrap();
        let mut dexterity:T = num::cast(10).unwrap();
        let mut constitution:T = num::cast(10).unwrap();
        let mut intelligence:T = num::cast(10).unwrap();
        let mut charisma:T = num::cast(10).unwrap();
        let mut wisdom:T = num::cast(10).unwrap();
        let mut age:T = num::cast(10).unwrap();
        //TODO OR use legendary.ini + serde
        match *self {
            Animal::Rat => {
                gp += num::cast(2).unwrap();
            }
            Animal::Snake => {
                speed += num::cast(2).unwrap();
                mp += num::cast(1).unwrap();
                hp += num::cast(1).unwrap();
                xp += num::cast(1).unwrap();
                gp += num::cast(1).unwrap();
                atk += num::cast(1).unwrap();
                def += num::cast(1).unwrap();
                m_atk += num::cast(1).unwrap();
                m_def += num::cast(1).unwrap();
            }
            Animal::Rabbit => {
                speed += num::cast(2).unwrap();
                hp += num::cast(2).unwrap();
                mp += num::cast(2).unwrap();
                xp += num::cast(2).unwrap();
                atk += num::cast(2).unwrap();
                def += num::cast(2).unwrap();
                m_atk += num::cast(2).unwrap();
                m_def += num::cast(7).unwrap();
            }
            Animal::Wolf => {
                speed += num::cast(3).unwrap();
                hp += num::cast(3).unwrap();
                mp += num::cast(3).unwrap();
                xp += num::cast(3).unwrap();
                gp += num::cast(2).unwrap();
                atk += num::cast(10).unwrap();
                def += num::cast(5).unwrap();
                m_atk += num::cast(3).unwrap();
                m_def += num::cast(5).unwrap();
            }
            Animal::Crocodile => {
                speed -= num::cast(1).unwrap();
                hp += num::cast(5).unwrap();
                mp += num::cast(5).unwrap();
                xp += num::cast(5).unwrap();
                gp += num::cast(4).unwrap();
                atk += num::cast(7).unwrap();
                def += num::cast(7).unwrap();
                m_atk += num::cast(3).unwrap();
                m_def += num::cast(1).unwrap();
            }
            Animal::Insect => {
                hp -= num::cast(2).unwrap();
                mp += num::cast(4).unwrap();
                atk += num::cast(2).unwrap();
                def += num::cast(3).unwrap();
                m_atk += num::cast(1).unwrap();
                m_def += num::cast(1).unwrap();
            }
            Animal::Lion => {
                hp += num::cast(10).unwrap();
                speed += num::cast(7).unwrap();
                mp += num::cast(4).unwrap();
                xp += num::cast(9).unwrap();
                atk += num::cast(11).unwrap();
                def += num::cast(9).unwrap();
                m_atk += num::cast(5).unwrap();
                m_def += num::cast(8).unwrap();
            }
            Animal::Tiger => {
                hp += num::cast(9).unwrap();
                speed += num::cast(9).unwrap();
                mp += num::cast(7).unwrap();
                xp += num::cast(7).unwrap();
                atk += num::cast(12).unwrap();
                def += num::cast(5).unwrap();
                m_atk += num::cast(5).unwrap();
                m_def += num::cast(12).unwrap();
            }
            Animal::Bear => {
                hp += num::cast(15).unwrap();
                mp += num::cast(10).unwrap();
                xp += num::cast(10).unwrap();
                atk += num::cast(18).unwrap();
                def += num::cast(12).unwrap();
                m_atk += num::cast(2).unwrap();
                m_def += num::cast(5).unwrap();
            }
            Animal::Bird => {
                hp += num::cast(2).unwrap();
                mp += num::cast(12).unwrap();
                xp += num::cast(7).unwrap();
                atk += num::cast(11).unwrap();
                def += num::cast(1).unwrap();
                m_atk += num::cast(8).unwrap();
                m_def += num::cast(10).unwrap();
            }
            
        }
        hp *= level;
        mp *= level;
        // TODO fixme:
        xp *= level;
        // TODO fixme:
        xp_next *= level;
        gp *= level;
        speed += level;
        Advanced {
            id:id,
            xp:xp,
            xp_next:xp_next,
            level:level,
            gp:gp,
            hp: hp,
            mp: mp,
            hp_max: hp,
            mp_max: mp,
            speed: speed,
            atk:atk,
            def:def,
            m_atk:m_atk,
            m_def:m_def,
            agility:agility,
            strength:strength,
            dexterity:dexterity,
            constitution:constitution,
            intelligence:intelligence,
            charisma:charisma,
            wisdom:wisdom,
            age:age,
        }
    }
}
