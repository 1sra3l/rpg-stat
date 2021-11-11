/*!
# Attributes

These are definitions of abstract terms into code

## Rate
Rate of occurance
```
use rpgstat::attributes::Rate;
let yes:Rate = Rate::Always;
assert_eq!(yes.worked(), true);
let no:Rate = Rate::None;
assert_eq!(no.worked(), false);
let hmmm:Rate = Rate::Some;
// who knows....
```

## Effectiveness


```
use rpgstat::attributes::{Effectiveness, Value};
let hp:i32 = 50;
// later on we use an item and check the effectiveness of it
assert_eq!(Effectiveness::Half.value(hp), 25);

```

## Stage
```
use rpgstat::attributes::Stage;
let stage:Stage<i32> = Stage::stage(15);
//
assert_eq!(stage, Stage::Teen(15));

```
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
            Rate::Always => true, // 100%
            Rate::Usually => {
                self.usually()
            },
            Rate::Often => { // 75%
                self.often()
            },
            Rate::Some => { // 50%
                self.half()
            },
            Rate::Hardly => {
                self.hardly()
            },
            Rate::Barely => {
                self.barely()
            },
            Rate::None => false, // 0%
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
pub enum Effectiveness {
    
    Double,
    HalfExtra,
    Half,
    Normal,
    None,
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
    + num::NumCast> Value<T> for Effectiveness {
    fn value(&self, input:T) -> T {
        let mut result:T = Default::default();
        match *self {
            Effectiveness::Double => {
                result = input + input;
                result
            },
            Effectiveness::HalfExtra => {
                let half:T = input / num::cast(2).unwrap();
                result = input + half;
                result
            },
            Effectiveness::Normal => {
                input
            },
            Effectiveness::Half => {
                let half:T = input / num::cast(2).unwrap();
                half
            },
            Effectiveness::None => {
                num::cast(0).unwrap()
            },
        }
    }
}
impl Default for Effectiveness {
    /// Default to empty
    fn default() -> Self {
        Self::None
    }
}
impl fmt::Display for Effectiveness {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let v:String;
        match *self {
            Effectiveness::Double => v = String::from("Double"),
            Effectiveness::HalfExtra => v = String::from("HalfExtra"),
            Effectiveness::Half => v = String::from("Half"),
            Effectiveness::Normal => v = String::from("Normal"),
            Effectiveness::None => v = String::from("None"),
        }
        write!(f, "{}", v.as_str())
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
            Stage::Baby(age)
        } else if age < num::cast(4).unwrap() {
            Stage::Toddler(age)
        } else if age < num::cast(13).unwrap() {
            Stage::Kid(age)
        } else if age < num::cast(20).unwrap() {
            Stage::Teen(age)
        } else if age < num::cast(40).unwrap() {
            Stage::Young(age)
        } else if age < num::cast(65).unwrap() {
            Stage::Grown(age)
        } else if age < num::cast(85).unwrap() {
            Stage::Older(age)
        } else {
            Stage::Old(age)
        }
        
    }
}

pub trait Value<T:Copy
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
    + num::NumCast> {
    fn value(&self, input:T) -> T;
}
