/*!
# Attributes

These are definitions of abstract terms into code

## Rate
Rate of occurance
```
use rpg_stat::attributes::Rate;
let yes:Rate = Rate::Always;
assert_eq!(yes.worked(), true);
let no:Rate = Rate::None;
assert_eq!(no.worked(), false);
let hmmm:Rate = Rate::Some;
// who knows....
```

## Effectiveness

We can easily find the value of an effectiveness:

```
use rpg_stat::attributes::{Effectiveness, Value};
let hp:i32 = 50;
// later on we use an item and check the effectiveness of it
assert_eq!(Effectiveness::Half.value(hp), 25);

```
This effectiveness can be stored in a struct and you could implement a wrapper for `value(T)`:
```
use rpg_stat::attributes::{Effectiveness, Value};

pub struct Item {
    pub name:String,
    pub amount:i32,
    pub effectiveness:Effectiveness,
}
impl Item {
    // much easier to use now!
    pub fn value(&self) -> i32 {
        self.effectiveness.value(self.amount)
    }
}
```

## Stage
```
use rpg_stat::attributes::{Stage, Value};
let stage:Stage = Stage::Baby.value(15);
//
assert_eq!(stage, Stage::Teen);

```


*/
use std::fmt;
use std::fmt::Debug;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign};
extern crate num;
use serde::{Deserialize, Serialize};

#[cfg(feature = "fltkform")]
use fltk::{prelude::*, *};
#[cfg(feature = "fltkform")]
use fltk_form_derive::*;
#[cfg(feature = "fltkform")]
use fltk_form::FltkForm;

// our modules
use crate::random::Random;


/*
# Rate

This can be used to determine the Rate at which enemies/items appear in areas, or can be used for the Rate effectiveness of an attack/item/etc

To find a random true/false value simple call `worked()` on your enum

```
use rpg_stat::attributes::Rate;
let yes:Rate = Rate::Always;
assert_eq!(yes.worked(), true);
assert_eq!(Rate::None::worked(), false);
```
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[cfg_attr(feature = "fltkform", derive(FltkForm))]
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
impl Random for Rate{
    type Type = Rate;
    fn random_type(&self) -> Self::Type {
        let max = 7;
        let val = self.random_rate(max);
        match val {
            0 => Rate::None,
            1 => Rate::Often,
            3 => Rate::Hardly,
            4 => Rate::Some,
            5 => Rate::Barely,
            7 => Rate::Always,
            _=> Rate::Usually,
        }
    }
}
impl Rate {
    /*
    
    */
    #[allow(unused)]
    pub fn worked(&self) -> bool {
        match *self {
            Rate::Always => true, // 100%
            Rate::Usually => self.usually(),
            Rate::Often => self.often(),
            Rate::Some => self.half(), // 50%
            Rate::Hardly => self.hardly(),
            Rate::Barely => self.barely(),
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

```rs
use rpg_stat::attributes::{Effectiveness, Value};
let hp:i32 = 50;
// later on we use an item and check the effectiveness of it
assert_eq!(Effectiveness::Half.value(hp), 25);

```
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[cfg_attr(feature = "fltkform", derive(FltkForm))]
pub enum Effectiveness {    
    /// Two times the effect
    Double,
    /// One half extra added
    HalfExtra,
    /// Half of the amount
    Half,
    /// The full amount
    Normal,
    /// No amount
    None,
}
impl Random for Effectiveness{
    type Type = Effectiveness;
    fn random_type(&self) -> Self::Type {
        let max = 6;
        let val = self.random_rate(max);
        match val {
            0 => Effectiveness::Double,
            1 => Effectiveness::HalfExtra,
            4 => Effectiveness::Half,
            5 => Effectiveness::Normal,
            _=> Effectiveness::None,
        }
    }
}
impl<T:Copy
    + Default
    + Debug
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
    type Type = T;
    fn value(&self, input:T) -> Self::Type {
        match *self {
            Effectiveness::Double => input + input,
            Effectiveness::HalfExtra => {
                let half:T = input / num::cast(2).unwrap();
                input + half
            },
            Effectiveness::Normal => input,
            Effectiveness::Half => input / num::cast(2).unwrap(),
            Effectiveness::None => num::cast(0).unwrap(),
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[cfg_attr(feature = "fltkform", derive(FltkForm))]
/// This is the 'stage' of life the creature is in
/// Stages of life are similar to Pokemon evolution,
/// however our creatures cannot change species randomly
/// Using a life stage is based in real life, rather than
/// the random changing into some other species thing
pub enum Stage {
    Baby,
    Toddler,
    Kid,
    Teen,
    Young,
    Grown,
    Older,
    Old,
}
impl Random for Stage{
    type Type = Stage;
    fn random_type(&self) -> Self::Type {
        let max = 8;
        let val = self.random_rate(max);
        match val {
            0 => Stage::Toddler,
            1 => Stage::Kid,
            3 => Stage::Teen,
            4 => Stage::Young,
            5 => Stage::Grown,
            7 => Stage::Older,
            8 => Stage::Old,
            _=> Stage::Baby,
        }
    }
}
impl Stage {
    /// Default to empty
    #[allow(dead_code)]
    fn default() -> Self {
        Self::Teen
    }
}
impl<T:Copy
    + Default
    + Debug
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
    + num::NumCast> Value<T> for Stage {
    /// Get the Life stage of the Kreature
    /// The stage the Kreature is at is determined by their 'age'
    type Type = Stage;
    fn value(&self, age:T) -> Self::Type {
        if age < num::cast(2).unwrap() {
            Stage::Baby
        } else if age < num::cast(4).unwrap() {
            Stage::Toddler
        } else if age < num::cast(13).unwrap() {
            Stage::Kid
        } else if age < num::cast(20).unwrap() {
            Stage::Teen
        } else if age < num::cast(40).unwrap() {
            Stage::Young
        } else if age < num::cast(65).unwrap() {
            Stage::Grown
        } else if age < num::cast(85).unwrap() {
            Stage::Older
        } else {
            Stage::Old
        }
        
    }
}

pub trait Value<T:Copy
    + Default
    + Debug
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
    type Type;
    fn value(&self, input:T) -> Self::Type;
}
