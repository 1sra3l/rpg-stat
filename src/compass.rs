/*!
# Compass
The direction module

All of these default to None

## Basic

intended for the most basic clicker-style RPG

```
// basic compass has Forward/Backward
use rpg_stat::compass::Basic as Compass;
// this is what I want
let direction = Compass::Forward;
```

## Normal

Intended for standard top-down world view RPG

```
// normal compass has Up, Down, Left, Right
use rpg_stat::compass::Normal as Compass;
// this is what I want
let direction = Compass::Up;
```

## Advanced

```
// advanced compass has the four cardinal and four intercardinal directions
use rpg_stat::compass::Advanced as Compass;
// this is where to go in winter
let direction = Compass::South;
```

*/
use std::fmt;
use crate::random::Random;
use serde::{Deserialize, Serialize};
#[cfg(feature = "fltkform")]
use fltk::{prelude::*, *};
#[cfg(feature = "fltkform")]
use fltk_form_derive::*;
#[cfg(feature = "fltkform")]
use fltk_form::FltkForm;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[cfg_attr(feature = "fltkform", derive(FltkForm))]
/*
# Basic
1-D movement, like time
*/
pub enum Basic {
    /// 
    Forward,
    /// 
    Backward,
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
            Basic::Forward => v = String::from("Forward"),
            Basic::Backward => v = String::from("Backward"),
            _=> v = String::from("None"),
        }
        write!(f, "{}", v.as_str())
    }
}
impl Random for Basic {
    type Type = Basic;
    fn random_type(&self) -> Self::Type {
        let max = 2;
        let val = self.random_rate(max);
        match val {
            0 => Basic::Forward,
            1 => Basic::Backward,
            _=> Basic::None,
        }
    }
    
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[cfg_attr(feature = "fltkform", derive(FltkForm))]
/*
# Normal
2-D movement
*/
pub enum Normal {
    /// Going up the y axis
    Up,
    /// Going down the y axis
    Down,
    /// Going up the x axis
    Left,
    /// Going down the x axis
    Right,
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
            Normal::Up => v = String::from("Up"),
            Normal::Down => v = String::from("Down"),
            Normal::Left => v = String::from("Left"),
            Normal::Right => v = String::from("Right"),
            _=> v = String::from("None"),
        }
        write!(f, "{}", v.as_str())
    }
}
impl Random for Normal {
    type Type = Normal;
    fn random_type(&self) -> Self::Type {
        let max = 4;
        let val = self.random_rate(max);
        match val {
            0 => Normal::Up,
            1 => Normal::Down,
            2 => Normal::Left,
            3 => Normal::Right,
            _=> Normal::None,
        }
    }
    
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[cfg_attr(feature = "fltkform", derive(FltkForm))]
/*
# Advanced
The four cardinal directions and the four intercardinal directions
*/
pub enum Advanced {
    /// Opposite of South, up
    North,
    /// Opposite of SouthWest, up-right
    NorthEast,
    /// Opposite of West, right
    East,
    /// Opposite of NorthWest, down-right
    SouthEast,
    /// Opposite of North, down
    South,
    /// Opposite of NorthEast, down-left
    SouthWest,
    /// Opposite of East, left
    West,
    /// Opposite of SouthEast, up-left
    NorthWest,
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
            Advanced::North => v = String::from("North"),
            Advanced::NorthEast => v = String::from("NorthEast"),
            Advanced::East => v = String::from("East"),
            Advanced::SouthEast => v = String::from("SouthEast"),
            Advanced::South => v = String::from("South"),
            Advanced::SouthWest => v = String::from("SouthWest"),
            Advanced::West => v = String::from("West"),
            Advanced::NorthWest => v = String::from("NorthWest"),
            _=> v = String::from("None"),
        }
        write!(f, "{}", v.as_str())
    }
}
impl Random for Advanced {
    type Type = Advanced;
    fn random_type(&self) -> Self::Type {
        let max = 8;
        let val = self.random_rate(max);
        match val {
            0 => Advanced::North,
            1 => Advanced::NorthEast,
            2 => Advanced::East,
            3 => Advanced::SouthEast,
            4 => Advanced::South,
            5 => Advanced::SouthWest,
            6 => Advanced::West,
            7 => Advanced::NorthWest,
            _=> Advanced::None,
        }
    }
    
}
