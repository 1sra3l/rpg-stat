/*!
# Types

This includes various enums related to the type of character you have

`Basic` is the basic type `Good` or `Bad`

`Normal` has elemental types

`Advanced` has elemental types

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
