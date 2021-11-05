/*!
# Character Class

This abstraction can be as `Basic` as `Hero` or `Enemy`
You can even use the fully `Advanced` version to use the entire class realm.
*/
use std::fmt;
use serde::{Deserialize, Serialize};
use crate::stats::Basic as BasicStats;
use crate::stats::Normal as NormalStats;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

/*
Alignement allows for the creation of multile outcomes in situations
*/
#[derive(Clone, PartialEq, Copy, Debug, Serialize, Deserialize, EnumIter)]
pub enum Alignment {
    Light,
    Dark,
    Undecided,
}
impl Default for Alignment {
    fn default() -> Self {
        Self::Undecided
    }
}
impl fmt::Display for Alignment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let v:String;
        match *self {
            Alignment::Light => v = String::from("Light"),
            Alignment::Dark => v = String::from("Dark"),
            Alignment::Undecided => v = String::from("Undecided"),
        }
        write!(f, "{}", v.as_str())
    }
}

/*
Default to making an `Enemy`, because there are honestly few `Hero`s in games
*/
#[derive(Clone, PartialEq, Copy, Debug, Serialize, Deserialize, EnumIter)]
pub enum Basic {
    /// Obviously the protagonist
    Hero,
    /// Obviously the antagonist
    Enemy,
}
impl Default for Basic {
    fn default() -> Self {
        Self::Enemy
    }
}
impl fmt::Display for Basic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let v:String;
        match *self {
            Basic::Hero => v = String::from("Hero"),
            Basic::Enemy => v = String::from("Enemy"),
        }
        write!(f, "{}", v.as_str())
    }
}

/*
# The "Normal" type of class

This can be used in combination with `Basic` if both sides are the same, to provide PvP, or soldier battle scenarios

To use this alone something like `Soldier` could be the enemies in that scenario, or however one sees fit.

Not all variants need to be processed, so feel free to ignore the ones you don't like and extend it via something like:

```rs:no_run
   match normal {
    Adept => (), //whatever you want
    Archer => (), //whatever you want
    // etc ...
    _=> {
        // my stuff is cool
    },
   
   }

```

You can couple these things with `Alignment` to produce variations on Monks, Elementals, Alchemists, etc..

*/
#[derive(Clone, PartialEq, Copy, Debug, Serialize, Deserialize, EnumIter)]
pub enum Normal {
    /// Full of concotions to both heal and harm.
    Alchemist,
    /// Aw, shoot you are good!
    Archer,
    /// Devoted to studying the elements and harnessing their powers
    Elemental,
    /// Generally this is the good guy in the story...
    Knight,
    /// Devoted to reading really old books and figuring out really old combinations of substances
    Monk,
    /// Into life magic, which overcomes death magic
    Priest,
    /// This is the default, because a lot of RPG games involve the `Soldier` in a dungeon being an enemy for whatever reason...
    Soldier,
    /// Incredibly stealthy, this rounded character can survive any environment
    Ranger,
    /// The elite female warriors from the deepest coldest winters
    Valkyrie,
}
impl Default for Normal {
    fn default() -> Self {
        Self::Soldier
    }
}
impl fmt::Display for  Normal{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let v:String;
        match *self {
            Normal::Alchemist => v = String::from("Alchemist"),
            Normal::Archer => v = String::from("Archer"),
            Normal::Elemental => v = String::from("Elemental"),
            Normal::Knight => v = String::from("Knight"),
            Normal::Monk => v = String::from("Monk"),
            Normal::Priest => v = String::from("Priest"),
            Normal::Soldier => v = String::from("Soldier"),
            Normal::Ranger => v = String::from("Ranger"),
            Normal::Valkyrie => v = String::from("Valkyrie"),
        }
        write!(f, "{}", v.as_str())
    }
}
/*
The Advanced group includes more support type characters.  Thhese classes can be used primarily for NPC type characters in games as well as the `Normal` types also included in `Advanced`

You can couple these things with `Alignment` to produce variations on Monks, Elementals, Alchemists, etc..
*/
pub enum Advanced {
    /// The goal is to discover new things without needing to get into fights
    Adventurer,
    /// Scientist who creates useful medicines
    Alchemist,
    /// Often found hunting for game
    Archer,
    /// Crafting and creating goods of many uses
    Artistan,
    /// Focused solely on support for other players/characters and their healing
    Clergy,
    /// Devoted to studying the elements and harnessing their powers
    Elemental,
    /// People devoted to keeping roads fixed and laws in place
    Governmental,
    /// A  protector of the `Governmental` class
    Knight,
    /// Devoted to reading really old books and figuring out really old combinations of substances
    Monk,
    /// Into life magic, which overcomes death magic
    Priest,
    /// Known to do well in fights, they generally keep to the sea
    Sailor,
    /// Travelling abroad to wage wars, they return infrequently
    Soldier,
    /// Incredibly stealthy, this rounded character can survive any environment
    Ranger,
    /// The elite female warriors from the deepest coldest winters
    Valkyrie,
    /// One of the various classes of people who work
    Worker,
}
impl Default for Advanced {
    fn default() -> Self {
        Self::Worker
    }
}
impl fmt::Display for  Advanced{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let v:String;
        match *self {
            Advanced::Adventurer => v = String::from("Adventurer"),
            Advanced::Alchemist => v = String::from("Alchemist"),
            Advanced::Archer => v = String::from("Archer"),
            Advanced::Artistan => v = String::from("Artistan"),
            Advanced::Clergy => v = String::from("Clergy"),
            Advanced::Elemental => v = String::from("Elemental"),
            Advanced::Governmental => v = String::from("Governmental"),
            Advanced::Knight => v = String::from("Knight"),
            Advanced::Monk => v = String::from("Monk"),
            Advanced::Priest => v = String::from("Priest"),
            Advanced::Sailor => v = String::from("Sailor"),
            Advanced::Soldier => v = String::from("Soldier"),
            Advanced::Ranger => v = String::from("Ranger"),
            Advanced::Valkyrie => v = String::from("Valkyrie"),
            Advanced::Worker => v = String::from("Worker"),
        }
        write!(f, "{}", v.as_str())
    }
}
