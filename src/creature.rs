/*!
# Creature Types

This encompasses all the different humanoids, as well as enemy creatures, and even pets
*/
use std::fmt;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
#[derive(Clone, PartialEq, Copy, Debug, EnumIter)]//, Serialize, Deserialize)]

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
