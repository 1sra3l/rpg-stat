/*!
# Attributes

These are definitions of abstract terms into code

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
use crate::random::Random;

/*
# Rate

This can be used to determine the Rate at which enemies/items appear in areas, or can be used for the Rate effectiveness of an attack/item/etc

To find a random true/false value simple call `worked()` on your enum
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, Deserialize, Serialize)]
pub enum Rate {
    /// 100%
    Always,
    /// 90%
    Usually,
    /// 75%
    Often,
    /// 50%
    Some,
    /// 25%
    Hardly,
    /// 10%
    Barely,
    /// 0%
    None,
}
impl Random for Rate{}
impl Rate {
    /*
    
    */
    pub fn worked(&self) -> bool {
        match *self {
            Rate::Always => return true, // 100%
            Rate::Usually => {
                return self.usually()
            },
            Rate::Often => { // 75%
                return self.often()
            },
            Rate::Some => { // 50%
                return self.half()
            },
            Rate::Hardly => {
                return self.hardly()
            },
            Rate::Barely => {
                return self.barely()
            },
            Rate::None => return false, // 0%
        }
    }
}
impl fmt::Display for Rate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let v:String;
        match *self {
            Rate::Always => v = String::from("Always"),
            Rate::Usually => v = String::from("Usually"),
            Rate::Often => v = String::from("Often"),
            Rate::Some => v = String::from("Some"),
            Rate::Hardly => v = String::from("Hardly"),
            Rate::Barely => v = String::from("Barely"),
            Rate::None => v = String::from("None"),
        }
        write!(f, "{}", v.as_str())
    }
}

/*
# Effectiveness
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, Deserialize, Serialize)]
pub enum Effectiveness<T:Copy 
                 + Default
                 + Display
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
    
    Double(T),
    HalfExtra(T),
    Half(T),
    Normal(T),
    None(T),
}
impl<T:Copy
    + Display
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
    + num::NumCast> Effectiveness<T> {
    pub fn value(&self) -> T {
        let mut result:T = Default::default();
        match *self {
            Effectiveness::Double(input) => {
                result = input + input;
                return result
            },
            Effectiveness::HalfExtra(input) => {
                let half:T = input / num::cast(2).unwrap();
                result = input + half;
                return result
            },
            Effectiveness::Normal(input) => {
                return input
            },
            Effectiveness::Half(input) => {
                let half:T = input / num::cast(2).unwrap();
                return half
            },
            Effectiveness::None(input) => {
                return result
            },
        }
    }
}
impl<T:Copy
    + Display
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
    + num::NumCast> Default for Effectiveness<T> {
    /// Default to empty
    fn default() -> Self {
        Self::None(Default::default())
    }
}
impl<T:Copy 
    + Default
    + Display
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
    + num::NumCast> fmt::Display for Effectiveness<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let v:String;
        let mut value:T = Default::default();
        match *self {
            Effectiveness::Double(input) => {
                v = String::from("Double");
                value = input;
            },
            Effectiveness::HalfExtra(input) => {
                v = String::from("HalfExtra");
                value = input;
            },
            Effectiveness::Half(input) => {
                v = String::from("Half");
                value = input;
            },
            Effectiveness::Normal(input) => {
                v = String::from("Normal");
                value = input;
            },
            Effectiveness::None(input) => {
                v = String::from("None");
                value = input;
            },
        }
        write!(f, "{}({})", v.as_str(), value)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, Deserialize, Serialize)]
/// This is the 'stage' of life the creature is in
/// Stages of life are similar to Pokemon evolution,
/// however our creatures cannot change species randomly
/// Using a life stage is based in real life, rather than
/// the random changing into some other species thing
pub enum Stage<T:Copy 
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
    Baby(T),
    Toddler(T),
    Kid(T),
    Teen(T),
    Young(T),
    Grown(T),
    Older(T),
    Old(T),
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
    + num::NumCast> Default for Stage<T> {
    /// Default to empty
    fn default() -> Self {
        Self::Teen(num::cast(15).unwrap())
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
    + num::NumCast> Stage<T> {
    /// Get the Life stage of the Kreature
    /// The stage the Kreature is at is determined by their 'age'
    pub fn stage(age:T) -> Stage<T> {
        if age < num::cast(2).unwrap() {
            return Stage::Baby(age)
        } else if age < num::cast(4).unwrap() {
            return Stage::Toddler(age)
        } else if age < num::cast(13).unwrap() {
            return Stage::Kid(age)
        } else if age < num::cast(20).unwrap() {
            return Stage::Teen(age)
        } else if age < num::cast(40).unwrap() {
            return Stage::Young(age)
        } else if age < num::cast(65).unwrap() {
            return Stage::Grown(age)
        } else if age < num::cast(85).unwrap() {
            return Stage::Older(age)
        } else {
            return Stage::Old(age)
        }
        
    }
}
