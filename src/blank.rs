/*!
# 


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
/*
# Basic 

*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, Deserialize, Serialize)]
pub enum Basic {
    /// Nothing
    None,
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
            //Basic:: => v = String::from(""),
            _=> v = String::from("None"),
        }
        write!(f, "{}", v.as_str())
    }
}
/*
# Normal 

*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, Deserialize, Serialize)]
pub enum Normal {
    /// Nothing
    None,
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
            //Normal:: => v = String::from(""),
            _=> v = String::from("None"),
        }
        write!(f, "{}", v.as_str())
    }
}
/*
# Advanced 

*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, Deserialize, Serialize)]
pub enum Advanced {
    /// Nothing
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
            //Advanced:: => v = String::from(""),
            _=> v = String::from("None"),
        }
        write!(f, "{}", v.as_str())
    }
}
