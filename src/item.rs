/*!
# Item


*/
use serde::{Deserialize, Serialize};
#[cfg(feature = "fltkform")]
use fltk::{prelude::*, *};
#[cfg(feature = "fltkform")]
use fltk_form_derive::*;
#[cfg(feature = "fltkform")]
use fltk_form::{FltkForm, HasProps};

use std::fmt;

// #Condition
use crate::effect::Normal as Effect;
// #Element
use crate::types::Normal as Element;
use crate::random::*;
/*
# Item trait

This defines the functions for an item
 * `value()`
 * `cost()`
 * `resell()`

These Are applied to all *Item* types

TODO actual economic functions to vary prices
*/
pub trait Item {
    fn value(&self) -> f64;
    fn cost(&self) -> f64;
    fn resell(&self) -> f64;
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "fltkform", derive(FltkForm))]
pub enum Basic {
    Hp,
    Mp,
    None,
}
impl Default for Basic {
    fn default() -> Self {
        Self::None
    }
}impl Random for Basic {
    type Type = Basic;
    fn random_type(&self) -> Self::Type {
        let max = 2;
        let val = self.random_rate(max);
        match val {
            0 => return Basic::Hp,
            1 => return Basic::Mp,
            _=> return Basic::None,
        }
    }
}
impl fmt::Display for Basic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let v:String;
        match *self {
            Basic::Hp => v = String::from("Hp"),
            Basic::Mp => v = String::from("Mp"),
            Basic::None => v = String::from(""),
        }
        write!(f, "{}", v.as_str())
    }
}
impl Item for Basic {
    fn value(&self) -> f64 {
        match *self {
            Basic::Hp => 10.0,
            Basic::Mp => 5.0,
            Basic::None=> 0.0,
        }
    }
    fn cost(&self) -> f64 {
        match *self {
            Basic::Hp => 20.0,
            Basic::Mp => 50.0,
            Basic::None=> 0.0,
        }
    }
    fn resell(&self) -> f64 {
        match *self {
            Basic::Hp => 10.0,
            Basic::Mp => 25.0,
            Basic::None=> 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "fltkform", derive(FltkForm))]
pub enum Normal {
    Hp,
    Mp,
    Heal,
    Exp,
    Def,
    Atk,
    Speed,
    Special,
    None,
}
impl Default for Normal {
    fn default() -> Self {
        Self::None
    }
}
impl Random for Normal {
    type Type = Normal;
    fn random_type(&self) -> Self::Type {
        let max = 10;
        let val = self.random_rate(max);
        match val {
            0 => return Normal::Hp,
            1 => return Normal::Mp,
            4 => return Normal::Heal,
            5 => return Normal::Exp,
            7 => return Normal::Def,
            8 => return Normal::Atk,
            9 => return Normal::Speed,
            10 => return Normal::Special,
            _=> return Normal::None,
        }
    }
}
impl fmt::Display for Normal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let v:String;
        match *self {
            Normal::Hp => v = String::from("Hp"),
            Normal::Mp => v = String::from("Mp"),
            Normal::Heal => v = String::from("Heal"),
            Normal::Exp => v = String::from("Exp"),
            Normal::Def => v = String::from("Def"),
            Normal::Atk => v = String::from("Atk"),
            Normal::Speed => v = String::from("Speed"),
            Normal::Special => v = String::from("Special"),
            Normal::None => v = String::from(""),
        }
        write!(f, "{}", v.as_str())
    }
}
impl Item for Normal {
    fn value(&self) -> f64 {
        match *self {
            Normal::Hp => 10.0,
            Normal::Mp => 5.0,
            Normal::Heal => 10.0,
            Normal::Exp => 1.0,
            Normal::Def => 1.0,
            Normal::Atk => 1.0,
            Normal::Speed => 1.0,
            Normal::Special => 1.0,
            _=> 0.0,
        }
    }
    fn cost(&self) -> f64 {
        match *self {
            Normal::Hp => 20.0,
            Normal::Mp => 50.0,
            Normal::Heal => 10.0,
            Normal::Exp => 150.0,
            Normal::Def => 80.0,
            Normal::Atk => 90.0,
            Normal::Speed => 70.0,
            Normal::Special => 100.0,
            _=> 0.0,
        }
    }
    fn resell(&self) -> f64 {
        match *self {
            Normal::Hp => 10.0,
            Normal::Mp => 25.0,
            Normal::Heal => 5.0,
            Normal::Exp => 75.0,
            Normal::Def => 40.0,
            Normal::Atk => 45.0,
            Normal::Speed => 35.0,
            Normal::Special => 50.0,
            _=> 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "fltkform", derive(FltkForm))]
pub enum Advanced {
    Hp,
    Mp,
    Heal,
    Exp,
    Def,
    Atk,
    Speed,
    Special,
    Crystal,
    Powder,
    Gem,
    None,
}
impl Random for Advanced {
    type Type = Advanced;
    fn random_type(&self) -> Self::Type {
        let max = 10;
        let val = self.random_rate(max);
        match val {
            0 => return Advanced::Hp,
            1 => return Advanced::Mp,
            4 => return Advanced::Heal,
            5 => return Advanced::Exp,
            7 => return Advanced::Def,
            8 => return Advanced::Atk,
            9 => return Advanced::Speed,
            10 => return Advanced::Special,
            11 => return Advanced::Crystal,
            12 => return Advanced::Powder,
            13 => return Advanced::Gem,
            _=> return Advanced::None,
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
            Advanced::Hp => v = String::from("Hp"),
            Advanced::Mp => v = String::from("Mp"),
            Advanced::Heal => v = String::from("Heal"),
            Advanced::Exp => v = String::from("Exp"),
            Advanced::Def => v = String::from("Def"),
            Advanced::Atk => v = String::from("Atk"),
            Advanced::Speed => v = String::from("Speed"),
            Advanced::Special => v = String::from("Special"),
            Advanced::Crystal => v = String::from("Crystal"),
            Advanced::Powder => v = String::from("Powder"),
            Advanced::Gem  => v = String::from("Gem"),
            Advanced::None => v = String::from(""),
        }
        write!(f, "{}", v.as_str())
    }
}
impl Item for Advanced {
    fn value(&self) -> f64 {
        match *self {
            Advanced::Hp => 10.0,
            Advanced::Mp => 5.0,
            Advanced::Heal => 10.0,
            Advanced::Exp => 1.0,
            Advanced::Def => 1.0,
            Advanced::Atk => 1.0,
            Advanced::Speed => 1.0,
            Advanced::Special => 1.0,
            Advanced::Crystal => 30.0,
            Advanced::Powder => 40.0,
            Advanced::Gem => 50.0,
            _=> 0.0,
        }
    }
    fn cost(&self) -> f64 {
        match *self {
            Advanced::Hp => 20.0,
            Advanced::Mp => 50.0,
            Advanced::Heal => 10.0,
            Advanced::Exp => 150.0,
            Advanced::Def => 80.0,
            Advanced::Atk => 90.0,
            Advanced::Speed => 70.0,
            Advanced::Special => 100.0,
            Advanced::Crystal => 40.0,
            Advanced::Powder => 30.0,
            Advanced::Gem => 150.0,
            _=> 0.0,
        }
    }
    fn resell(&self) -> f64 {
        match *self {
            Advanced::Hp => 10.0,
            Advanced::Mp => 25.0,
            Advanced::Heal => 5.0,
            Advanced::Exp => 75.0,
            Advanced::Def => 40.0,
            Advanced::Atk => 45.0,
            Advanced::Speed => 35.0,
            Advanced::Special => 50.0,
            Advanced::Crystal => 20.0,
            Advanced::Powder => 15.0,
            Advanced::Gem => 90.0,
            _=> 0.0,
        }
    }
}
