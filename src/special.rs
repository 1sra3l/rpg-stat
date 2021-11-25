/*!
# Special

Special moves learned by `rpgstat::creatures::*;`


*/
use std::fmt;
use std::fmt::Debug;
//use std::fmt::Display;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign};
extern crate num;
//use num::NumCast;
use serde::{Deserialize, Serialize};
//use strum::IntoEnumIterator;
use strum_macros::EnumIter;
#[cfg(feature = "fltkform")]
use fltk::{prelude::*, *};
#[cfg(feature = "fltkform")]
use fltk_form_derive::*;
#[cfg(feature = "fltkform")]
use fltk_form::{FltkForm, HasProps};


use crate::random::*;

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
impl Random for Basic {
    type Type = Basic;
    fn random_type(&self) -> Self::Type {
        if self.half() {
            return Basic::None
        }
        if self.half() {
            return Basic::Some(random_character_name())
        }
        Basic::Some(random_creature_name())
    }
}impl Default for Basic {
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
#[cfg_attr(feature = "fltkform", derive(FltkForm))]
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
impl Random for Normal {
    type Type = Normal;
    fn random_type(&self) -> Self::Type {
        let max = 18;
        let val = self.random_rate(max);
        match val {
            0 => Normal::Tackle,
            1 => Normal::Toss,
            2 => Normal::Throw,
            3 => Normal::Slash,
            4 => Normal::Freeze,
            5 => Normal::Burn,
            6 => Normal::Melt,
            7 => Normal::Crush,
            8 => Normal::Grind,
            9 => Normal::Hit,
            10 => Normal::Slap,
            11 => Normal::Smack,
            12 => Normal::Whip,
            13 => Normal::Slice,
            14 => Normal::Spin,
            15 => Normal::Blur,
            16 => Normal::Strike,
            17 => Normal::Splash,
            _=> Normal::None,
        }
    }
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
    fn damage(&self, level:T) -> T {
        let one:T = num::cast(1).unwrap();
        let five:T = num::cast(5).unwrap();
        let seven:T = num::cast(7).unwrap();
        let empty:T = num::cast(1).unwrap();
        match self {
            Normal::Toss => one * level,
            Normal::Throw => one * level,
            Normal::Strike => one * level,
            Normal::Tackle => one * level,
            Normal::Spin => one * level,

            Normal::Slash =>  five * level,
            Normal::Burn =>  five * level,
            Normal::Blur => five * level,
            Normal::Splash =>  five * level,
            Normal::Crush =>  five * level,
            Normal::Hit =>  five * level,
            Normal::Slap =>  five * level,
            Normal::Whip =>  five * level,

            Normal::Grind =>  seven * level,
            Normal::Smack =>  seven * level,
            Normal::Melt =>  seven * level,
            Normal::Slice =>  seven * level,
            Normal::Freeze => seven * level,
            Normal::None => empty * level,
            // Normal:: => one,
        }
    }
    fn mp_total(&self, _input:T) -> T {
        let thirty:T = num::cast(30).unwrap();
        let twenty:T = num::cast(20).unwrap();
        let ten:T = num::cast(10).unwrap();
        let empty:T = num::cast(1).unwrap();
        match self {
            Normal::Toss => thirty,
            Normal::Throw => thirty,
            Normal::Strike => thirty,
            Normal::Tackle => thirty,
            Normal::Spin => thirty,

            Normal::Slash =>  twenty,
            Normal::Burn =>  twenty,
            Normal::Blur => twenty,
            Normal::Splash =>  twenty,
            Normal::Crush =>  twenty,
            Normal::Hit =>  twenty,
            Normal::Slap =>  twenty,
            Normal::Whip =>  twenty,

            Normal::Grind =>  ten,
            Normal::Smack =>  ten,
            Normal::Melt =>  ten,
            Normal::Slice =>  ten,
            Normal::Freeze => ten,
            Normal::None => empty,
            // Normal:: => one,
        }
    }
}

/*
# Advanced 

*/
//TODO more specials
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, Deserialize, Serialize)]
#[cfg_attr(feature = "fltkform", derive(FltkForm))]
pub enum Advanced {
    /// 
    /// None
    None,
}
impl Random for Advanced {
    type Type = Advanced;
    fn random_type(&self) -> Self::Type {
        let max = 18;
        let val = self.random_rate(max);
        match val {
            _=> Advanced::None,
        }
    }
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
    fn damage(&self, level:T) -> T;
    fn mp_total(&self, _input:T) -> T;
}
