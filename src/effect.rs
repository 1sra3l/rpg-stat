/*!
# Effect
This composes the various Effects in-game related to a character's Stats


*/
use std::fmt;
use std::fmt::Debug;
//use std::fmt::Display;
//use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign};
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
use fltk_form::FltkForm;

use crate::random::Random;

#[cfg(feature = "makesvg")]
use svg::node::element::Group as SvgGroup;
#[cfg(feature = "makesvg")]
use crate::body::VectorView;

/*
# Basic 

*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, Deserialize, Serialize)]
#[cfg_attr(feature = "fltkform", derive(FltkForm))]
pub enum Basic {
    /// no effect aka a player or something else that should not have an effect, like a key
    None,
    /// add or remove HP based on the attack/item/special's HP
    HP,
    /// add or remove MP based on the attack/item/special's MP
    MP,
    /// add or remove XP. enemies all have this effect
    XP,
    /// add or remove GP.
    GP,
}
impl Default for Basic {
    fn default() -> Self {
        Self::None
    }
}
impl fmt::Display for Basic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let v:String;
        match *self {
            Basic::HP => v = String::from("HP"),
            Basic::MP => v = String::from("MP"),
            Basic::GP => v = String::from("GP"),
            Basic::XP => v = String::from("XP"),
            Basic::None => v = String::from("None"),
        }
        write!(f, "{}", v.as_str())
    }
}
impl Random for Basic {
    type Type = Basic;
    fn random_type(&self) -> Self::Type {
        let max = 4;
        let val = self.random_rate(max);
        match val {
            0 => Basic::HP,
            1 => Basic::MP,
            2 => Basic::XP,
            3 => Basic::GP,
            _=> Basic::None,
        }
    }
}

/*
# Normal 

*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, Deserialize, Serialize)]
#[cfg_attr(feature = "fltkform", derive(FltkForm))]
pub enum Normal {
    /// no effect aka a player or something else that should not have an effect, like a key
    None,
    /// add or remove HP based on the attack/item/special's HP
    HP,
    /// add or remove MP based on the attack/item/special's MP
    MP,
    /// add or remove XP. enemies all have this effect
    XP,
    /// add or remove GP.
    GP,
/// # These next five continously drain HP/MP until remedy, or duration runs out
    /// hp drain
    Burn,
    /// hp drain, very minor mp drain
    Poison,
    /// minor hp drain, very minor mp drain, no movement
    Freeze,
    /// hp drain, minor mp drain
    Sick,
    /// mp drain
    Sap,
/// # These next two continuously add HP/MP
    /// mp add
    Bless,
    /// hp add
    Heal,
/// # Blocker/locker effects
    /// no movement
    Stuck,
    /// no attack
    Bound,
    /// no mana attack
    Blocked,
    /// a lock to prevent access
    Locked,  
}
impl Default for Normal {
    fn default() -> Self {
        Self::None
    }
}
impl fmt::Display for Normal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let v:String;
        match *self {
            Normal::None => v = "None".to_string(),
            Normal::HP => v = "HP".to_string(),
            Normal::MP => v = "MP".to_string(),
            Normal::XP => v = "XP".to_string(),
            Normal::GP => v = String::from("GP"),
            Normal::Burn => v = "Burn".to_string(),
            Normal::Poison => v = "Poison".to_string(),
            Normal::Freeze => v = "Freeze".to_string(),
            Normal::Sick => v = "Sick".to_string(),
            Normal::Sap => v = "Sap".to_string(),
            Normal::Bless => v = "Bless".to_string(),
            Normal::Heal => v = "Heal".to_string(),
            Normal::Stuck => v = "Stuck".to_string(),
            Normal::Bound => v = "Bound".to_string(),
            Normal::Blocked => v = "Blocked".to_string(),
            Normal::Locked => v = "Locked".to_string(),
        }
        write!(f, "{}", v.as_str())
    }
}
impl Random for Normal {
    type Type = Normal;
    fn random_type(&self) -> Self::Type {
        let max = 15;
        let val = self.random_rate(max);
        match val {
            0 => Normal::HP,
            1 => Normal::MP,
            2 => Normal::XP,
            3 => Normal::GP,
            4 => Normal::Burn,
            5 => Normal::Poison,
            7 => Normal::Freeze,
            8 => Normal::Sick,
            9 => Normal::Sap,
            10 => Normal::Bless,
            11 => Normal::Heal,
            12 => Normal::Stuck,
            13 => Normal::Bound,
            14 => Normal::Blocked,
            15 => Normal::Locked,
            _=> Normal::None,
        }
    }
}
/*
# Advanced 

*/
//TODO more effects
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, Deserialize, Serialize)]
#[cfg_attr(feature = "fltkform", derive(FltkForm))]
pub enum Advanced {
    /// no effect aka a player or something else that should not have an effect, like a key
    None,
    /// add or remove HP based on the attack/item/special's HP
    HP,
    /// add or remove MP based on the attack/item/special's MP
    MP,
    /// add or remove XP. enemies all have this effect
    XP,
    /// add or remove GP.
    GP,
/// # These next five continously drain HP/MP until remedy, or duration runs out
    /// hp drain
    Burn,
    /// hp drain, very minor mp drain
    Poison,
    /// minor hp drain, very minor mp drain, no movement
    Freeze,
    /// hp drain, minor mp drain
    Sick,
    /// mp drain
    Sap,
/// # These next two continuously add HP/MP
    /// mp add
    Bless,
    /// hp add
    Heal,
/// # Blocker/locker effects
    /// no movement
    Stuck,
    /// no attack
    Bound,
    /// no mana attack
    Blocked,
    /// a lock to prevent access
    Locked,  
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
            Advanced::HP => v = "HP".to_string(),
            Advanced::MP => v = "MP".to_string(),
            Advanced::XP => v = "XP".to_string(),
            Advanced::GP => v = String::from("GP"),
            Advanced::Burn => v = "Burn".to_string(),
            Advanced::Poison => v = "Poison".to_string(),
            Advanced::Freeze => v = "Freeze".to_string(),
            Advanced::Sick => v = "Sick".to_string(),
            Advanced::Sap => v = "Sap".to_string(),
            Advanced::Bless => v = "Bless".to_string(),
            Advanced::Heal => v = "Heal".to_string(),
            Advanced::Stuck => v = "Stuck".to_string(),
            Advanced::Bound => v = "Bound".to_string(),
            Advanced::Blocked => v = "Blocked".to_string(),
            Advanced::Locked => v = "Locked".to_string(),
        }
        write!(f, "{}", v.as_str())
    }
}
