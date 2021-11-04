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
    /// Obviously we'd like to be heroic
    Human,
    /// What would a heroic journey be without an elf or two
    Elf,
    /// These little guys rock!
    Dwarf,
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
            Person::Human => v = String::from("Human"),
            Person::Elf => v = String::from("Elf"),
            Person::Dwarf => v = String::from("Dwarf"),
            Person::Sprite => v = String::from("Sprite"),
        }
        write!(f, "{}", v.as_str())
    }
}

#[derive(Clone, PartialEq, Copy, Debug, EnumIter)]
/// The various monsters you encounter
pub enum Animal {
    /// 
    Rat,
    /// 
    Snake,
    /// 
    Rabbit,
    /// 
    Wolf,
    /// 
    Crocodile,
}
impl Default for Animal {
    fn default() -> Self {
        Self::Rat
    }
}
impl fmt::Display for Animal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let v:String;
        match *self {
            Animal::Rat => v = String::from("Rat"),
            Animal::Snake => v = String::from("Snake"),
            Animal::Rabbit => v = String::from("Rabbit"),
            Animal::Wolf => v = String::from("Wolf"),
            Animal::Crocodile => v = String::from("Crocodile"),
        }
        write!(f, "{}", v.as_str())
    }
}
