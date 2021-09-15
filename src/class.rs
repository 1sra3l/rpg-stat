/*!
# Character Class

This abstraction can be as `Basic` as `Hero` or `Enemy`
You can even use the fully `Advanced` version to use the entire class realm.
*/
use std::fmt;
use serde::{Deserialize, Serialize};

/*
Default to making an `Enemy`, because there are honestly few `Hero`s in games
*/
#[derive(Clone, PartialEq, Copy, Debug, Serialize, Deserialize)]
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

Obviously you can forgo using this for enemies, by using the enemy enumerations and the same trait


*/
#[derive(Clone, PartialEq, Copy, Debug, Serialize, Deserialize)]
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

//impl ClassTrait for Normal {fn _class(&self){}}
//TODO lots...
pub enum Advanced {
    TODO,
}
impl fmt::Display for  Advanced{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let v:String;
        match *self {
            Advanced::TODO => v = String::from("TODO"),
        }
        write!(f, "{}", v.as_str())
    }
}

pub trait ClassTrait {
    fn _class(&self);
    
}
