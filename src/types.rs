/*!
# Types

This includes various enums related to the type of character you have

`Basic` is the basic type `Good` or `Bad`

`Normal` has elemental types

`Advanced` has elemental types

# Effectiveness

`Basic` has no need for effectiveness against types.

`Normal` implements this according to the chart:

![Alt text](https://github.com/1sra3l/rpg-stat/assets/type-effectiveness-chart.png?raw=true "Type Effectiveness")


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

//our stuff
use crate::attributes::Effectiveness;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, Deserialize, Serialize)]
/*
# Basic

This is the basic type `Good` or `Bad`
*/
pub enum Basic {
    /// Good
    Good,
    Bad,
}
impl Default for Basic {
    /// Default to enemy type for games
    fn default() -> Self {
        Self::Bad
    }
}
impl fmt::Display for Basic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let v:String;
        match *self {
            Basic::Good => v = String::from("Good"),
            Basic::Bad => v = String::from("Bad"),
        }
        write!(f, "{}", v.as_str())
    }
}
//
//  ```                        
//    ------   _  __ __     
//      |  \ / |) |_ |_     
//      |   |  |  |_ _/     
// ```
/// 
/// * rock     - earth type  
/// * plant    - green type  
/// * water    - liquid type 
/// * fire     - lava type   
/// * electric - lightning type
/// * spirit   - holy type    
/// * light    - laser type   
/// * wind     - tornado type 
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, Deserialize, Serialize)]
pub enum Normal {
    Rock,
    Plant,
    Water,
    Fire,
    Electric,
    Spirit,
    Light,
    Wind,
    None,
}
impl Normal {
    ///  Plant Effectiveness against a target
    pub fn plant(other:Normal, value:i32) -> Effectiveness<i32> {
        match other {
            Normal::Rock => return Effectiveness::HalfExtra(value),
            Normal::Water => return Effectiveness::Half(value),
            Normal::Fire => return Effectiveness::None(value),
            Normal::Light => return Effectiveness::Double(value),
            Normal::Wind => return Effectiveness::Half(value),
            _=> {},
        }
        Effectiveness::Normal(value)
    }
    /// Rock Effectiveness against a target
    pub fn rock(other:Normal, value:i32) -> Effectiveness<i32> {
        match other {
            Normal::Plant => return Effectiveness::Half(value),
            Normal::Water => return Effectiveness::HalfExtra(value),
            Normal::Fire => return Effectiveness::Double(value),
            Normal::Electric => return Effectiveness::Double(value),
            Normal::Light => return Effectiveness::Half(value),
            Normal::Wind => return Effectiveness::None(value),
            _=> {},
        }
        Effectiveness::Normal(value)
    }
    /// Water Effectiveness against a target
    pub fn water(other:Normal, value:i32) -> Effectiveness<i32> {
        match other {
            Normal::Rock => return Effectiveness::Double(value),
            Normal::Plant => return Effectiveness::HalfExtra(value),
            Normal::Fire => return Effectiveness::Double(value),
            Normal::Electric => return Effectiveness::Half(value),
            Normal::Light => return Effectiveness::None(value),
            Normal::Wind => return Effectiveness::Half(value),
            _=> {},
        }
        Effectiveness::Normal(value)
    }
    /// Fire Effectiveness against a target
    pub fn fire(other:Normal, value:i32) -> Effectiveness<i32> {
        match other {
            Normal::Rock => return Effectiveness::Half(value),
            Normal::Plant => return Effectiveness::Double(value),
            Normal::Water => return Effectiveness::HalfExtra(value),
            Normal::Spirit => return Effectiveness::None(value),
            Normal::Wind => return Effectiveness::Half(value),
            _=> {},
        }
        Effectiveness::Normal(value)
    }
    /// Electric Effectiveness against a target
    pub fn electric(other:Normal, value:i32) -> Effectiveness<i32> {
        match other {
            Normal::Rock => return Effectiveness::Half(value),
            Normal::Plant => return Effectiveness::HalfExtra(value),
            Normal::Water => return Effectiveness::Double(value),
            Normal::Light => return Effectiveness::None(value),
            Normal::Wind => return Effectiveness::Half(value),
            _=> {},
        }
        Effectiveness::Normal(value)
    }
    /// Spirit Effectiveness against a target
    pub fn spirit(other:Normal, value:i32) -> Effectiveness<i32> {
        match other {
            Normal::Water => return Effectiveness::None(value),
            Normal::Fire => return Effectiveness::Double(value),
            Normal::Electric => return Effectiveness::Half(value),
            Normal::Spirit => return Effectiveness::HalfExtra(value),
            Normal::Light => return Effectiveness::Half(value),
            Normal::Wind => return Effectiveness::Double(value),
            Normal::None => return Effectiveness::None(value),
            _=> {},
        }
        Effectiveness::Normal(value)
    }
    /// Light Effectiveness against a target
    pub fn light(other:Normal, value:i32) -> Effectiveness<i32> {
        match other {
            Normal::Rock => return Effectiveness::Double(value),
                    Normal::Plant => return Effectiveness::None(value),
                    Normal::Water => return Effectiveness::Double(value),
                    Normal::Fire => return Effectiveness::None(value),
                    Normal::Electric => return Effectiveness::Half(value),
                    Normal::Wind => return Effectiveness::HalfExtra(value),
                    Normal::None => return Effectiveness::Half(value),
            _=> {},
        }
        Effectiveness::Normal(value)
    }
    ///  Effectiveness against a target
    pub fn wind(other:Normal, value:i32) -> Effectiveness<i32> {
        match other {
            Normal::Rock => return Effectiveness::Double(value),
            Normal::Plant => return Effectiveness::Half(value),
            Normal::Water => return Effectiveness::Double(value),
            Normal::Fire => return Effectiveness::None(value),
            Normal::Spirit => return Effectiveness::Half(value),
            Normal::Wind => return Effectiveness::HalfExtra(value),
            _=> {},
        }
        Effectiveness::Normal(value)
    }
    ///  Effectiveness against a target
    pub fn none(other:Normal, value:i32) -> Effectiveness<i32> {
        match other {
            Normal::Water => return Effectiveness::Half(value),
            Normal::Fire => return Effectiveness::Half(value),
            Normal::Electric => return Effectiveness::Half(value),
            Normal::Spirit => return Effectiveness::None(value),
            Normal::Light => return Effectiveness::None(value),
            Normal::Wind => return Effectiveness::Half(value),
            _=> {},
        }
        Effectiveness::Normal(value)
    }
    
    /// Match current Type to find effectiveness of the value
    pub fn effectiveness(&self, other:Normal, value:i32) -> Effectiveness<i32> {
        match *self {
            Normal::Rock => return Normal::rock(other, value),
            Normal::Plant => return Normal::plant(other, value),
            Normal::Water => return Normal::water(other, value),
            Normal::Fire => return Normal::fire(other, value),
            Normal::Electric => return Normal::electric(other, value),
            Normal::Spirit => return Normal::spirit(other, value),
            Normal::Light => return Normal::light(other, value),
            Normal::Wind => return Normal::wind(other, value),
            Normal::None => return Normal::none(other, value),
        }
        Effectiveness::Normal(value)
    }
}
impl Default for Normal {
    /// Default to empty
    fn default() -> Self {
        Self::Rock
    }
}
impl fmt::Display for Normal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let v:String;
        match *self {
            Normal::Rock => v = String::from("Rock"),
            Normal::Plant => v = String::from("Plant"),
            Normal::Water => v = String::from("Water"),
            Normal::Fire => v = String::from("Fire"),
            Normal::Electric => v = String::from("Electric"),
            Normal::Spirit => v = String::from("Spirit"),
            Normal::Light => v = String::from("Light"),
            Normal::Wind => v = String::from("Wind"),
            Normal::None => v = String::from("None"),
        }
        write!(f, "{}", v.as_str())
    }
}
/*
# Advanced 
TODO
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, Deserialize, Serialize)]
pub enum Advanced {
    Rock,
    Plant,
    Water,
    Fire,
    Electric,
    Spirit,
    Light,
    Wind,
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
            Advanced::Rock => v = String::from("Rock"),
            Advanced::Plant => v = String::from("Plant"),
            Advanced::Water => v = String::from("Water"),
            Advanced::Fire => v = String::from("Fire"),
            Advanced::Electric => v = String::from("Electric"),
            Advanced::Spirit => v = String::from("Spirit"),
            Advanced::Light => v = String::from("Light"),
            Advanced::Wind => v = String::from("Wind"),
            Advanced::None => v = String::from("None"),
            //Advanced:: => v = String::from(""),
        }
        write!(f, "{}", v.as_str())
    }
}
