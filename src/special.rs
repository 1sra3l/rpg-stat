/*!
# Special

*/
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign};
extern crate num;
use num::NumCast;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
// TODO Advanced
#[derive(Debug, Clone, PartialEq, Eq, EnumIter, Deserialize, Serialize)]
/*
# Basic Special
This is quite versitle as it is similar to `Option<String>` being either `None` or `Some(String)`

*/
pub enum Basic {
    None,
    Some(String),
}
impl Default for Basic {
    fn default() -> Self {
        Self::None
    }
}
impl fmt::Display for Basic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut v:String;
        match &*self {
            Basic::None => v = String::from("None"),
            Basic::Some(thing) =>{
                v = String::from("Some(");
                v.push_str(thing.to_owned().as_str());
                v.push(')');
            },
        }
        write!(f, "{}", v.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, Deserialize, Serialize)]
/// Specials are just types of attack, coupled with the `Element`
/// These enums are used in determining the effects of the attack and which animations to use
pub enum Normal {
    /// The generic wrestling attack
    Tackle,
    /// Picking up and throwing an enemy a short distance
    Toss,
    /// Picking up and throwing an enemy very hard
    Throw,
    /// using claws, beaks or other sharp objects
    Slash,
    /// Reducing an enemy's core temperature and effecting their skin
    Freeze,
    /// Increasing an enemy's core temperature and effecting their skin
    Burn,
    /// Much more intensity than burn
    Melt,
    /// more intensity than tackle
    Crush,
    /// WAY more intensity than crush
    Grind,
    /// generic physical hit
    Hit,
    // not as powerful as Hit
    Slap,
    /// almost as powerful as Hit
    Smack,
    /// More powerful than Hit 
    Whip,
    /// More powerful than Slash
    Slice,
    /// disorienting attack
    Spin,
    /// more powerful than Spin
    Blur,
    /// Similar in power to Whip
    Strike,
    /// Much less powerful than Freeze
    Splash,
    /// None
    None,
}
impl Default for Normal {
    /// Default to empty
    fn default() -> Self {
        Self::None
    }
}
impl fmt::Display for Normal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let v:String;
        match *self {
            Normal::Toss => v = "Toss".to_string(),
            Normal::Throw => v = "Throw".to_string(),
            Normal::Slash => v = "Slash".to_string(),
            Normal::Freeze => v = "Freeze".to_string(),
            Normal::Burn => v = "Burn".to_string(),
            Normal::Melt => v = "Melt".to_string(),
            Normal::Crush => v = "Crush".to_string(),
            Normal::Grind => v = "Grind".to_string(),
            Normal::Hit => v = "Hit".to_string(),
            Normal::Slap => v = "Slap".to_string(),
            Normal::Smack => v = "Smack".to_string(),
            Normal::Whip => v = "whip".to_string(),
            Normal::Slice => v = "Slice".to_string(),
            Normal::Tackle => v = "Tackle".to_string(),
            Normal::Spin => v = "Spin".to_string(),
            Normal::Blur => v = "Blur".to_string(),
            Normal::Strike => v = "Strike".to_string(),
            Normal::Splash => v = "Splash".to_string(),
            Normal::None => v = "None".to_string(),
            // Normal:: => v = "".to_string(),
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
    + num::NumCast> ManaCost<T> for Normal{
    fn mp_cost(&self) -> T {
        let one:T = num::cast(1).unwrap();
        let five:T = num::cast(5).unwrap();
        let seven:T = num::cast(7).unwrap();
        let empty:T = num::cast(0).unwrap();
        match self {
            Normal::Toss => one,
            Normal::Throw => one,
            Normal::Strike => one,
            Normal::Tackle => one,
            Normal::Spin => one,

            Normal::Slash =>  five,
            Normal::Burn =>  five,
            Normal::Blur => five,
            Normal::Splash =>  five,
            Normal::Crush =>  five,
            Normal::Hit =>  five,
            Normal::Slap =>  five,
            Normal::Whip =>  five,

            Normal::Grind =>  seven,
            Normal::Smack =>  seven,
            Normal::Melt =>  seven,
            Normal::Slice =>  seven,
            Normal::Freeze => seven,
            Normal::None => empty,
            // Normal:: => one,
        }
    }
}
impl Normal {
    /// Get a special from a string (for saving/loading)
    /// 
    /// **Note:** *It does not matter what case comes into the function, everything is converted to lowercase via `to_lowercase()`*
    pub fn from_string(special:String) -> Normal {
        special.to_lowercase();
        if special == "toss" {
            return Normal::Toss
        } else if special == "throw" {
            return Normal::Throw
        } else if special == "slash" {
            return Normal::Slash
        } else if special == "freeze" {
            return Normal::Freeze
        } else if special == "burn" {
            return Normal::Burn
        } else if special == "melt" {
            return Normal::Melt
        } else if special == "crush" {
            return Normal::Crush
        } else if special == "grind" {
            return Normal::Grind
        } else if special == "hit" {
            return Normal::Hit
        } else if special == "slap" {
            return Normal::Slap
        } else if special == "smack" {
            return Normal::Smack
        } else if special == "whip" {
            return Normal::Whip
        } else if special == "slice" {
            return Normal::Slice
        } else if special == "spin" {
            return Normal::Spin
        } else if special == "blur" {
            return Normal::Blur
        } else if special == "strike" {
            return Normal::Strike
        } else if special == "splash" {
            return Normal::Splash
        } else if special == "tackle" {
            return Normal::Tackle
        }
        Normal::None
    }
}
/*
# Advanced 

*/
//TODO more specials
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, Deserialize, Serialize)]
pub enum Advanced {
        /// The generic wrestling attack
    Tackle,
    /// Picking up and throwing an enemy a short distance
    Toss,
    /// Picking up and throwing an enemy very hard
    Throw,
    /// using claws, beaks or other sharp objects
    Slash,
    /// Reducing an enemy's core temperature and effecting their skin
    Freeze,
    /// Increasing an enemy's core temperature and effecting their skin
    Burn,
    /// Much more intensity than burn
    Melt,
    /// more intensity than tackle
    Crush,
    /// WAY more intensity than crush
    Grind,
    /// generic physical hit
    Hit,
    // not as powerful as Hit
    Slap,
    /// almost as powerful as Hit
    Smack,
    /// More powerful than Hit 
    Whip,
    /// More powerful than Slash
    Slice,
    /// disorienting attack
    Spin,
    /// more powerful than Spin
    Blur,
    /// Similar in power to Whip
    Strike,
    /// Much less powerful than Freeze
    Splash,
    /// None
    None,
}
impl Default for Advanced {
    fn default() -> Self {
        Self::None
    }
}
impl fmt::Display for Advanced {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let v:String;
        match *self {
            Advanced::Toss => v = "Toss".to_string(),
            Advanced::Throw => v = "Throw".to_string(),
            Advanced::Slash => v = "Slash".to_string(),
            Advanced::Freeze => v = "Freeze".to_string(),
            Advanced::Burn => v = "Burn".to_string(),
            Advanced::Melt => v = "Melt".to_string(),
            Advanced::Crush => v = "Crush".to_string(),
            Advanced::Grind => v = "Grind".to_string(),
            Advanced::Hit => v = "Hit".to_string(),
            Advanced::Slap => v = "Slap".to_string(),
            Advanced::Smack => v = "Smack".to_string(),
            Advanced::Whip => v = "whip".to_string(),
            Advanced::Slice => v = "Slice".to_string(),
            Advanced::Tackle => v = "Tackle".to_string(),
            Advanced::Spin => v = "Spin".to_string(),
            Advanced::Blur => v = "Blur".to_string(),
            Advanced::Strike => v = "Strike".to_string(),
            Advanced::Splash => v = "Splash".to_string(),
            Advanced::None => v = "None".to_string(),
            //Advanced:: => v = String::from(""),
        }
        write!(f, "{}", v.as_str())
    }
}

pub trait ManaCost<T:Copy 
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
                      + num::NumCast> {
    fn mp_cost(&self) -> T;
}
