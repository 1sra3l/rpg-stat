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
    None,
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
            _=> 0.0,
        }
    }
}
