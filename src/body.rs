//use std::fmt;
use serde::{Deserialize, Serialize};

//use crate::random::*;
//use crate::class::Advanced as Class;

#[derive(Clone, Debug, Copy, PartialEq, Deserialize, Serialize)]
pub enum Shape {
    Normal,
    Anime,
    Rodent,
    Reptile,
    Bear,
    Canine,
    Feline,
    None,
}
impl Default for Shape {
    fn default() -> Self {
        Self::None
    }
}
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Body {
    pub head:Shape,
    pub hair:Shape,
    pub face:Shape,
    pub torso:Shape,
    pub arms:Shape,
    pub legs:Shape,
}
impl Default for Body {
    fn default() -> Self {
        Self::new()
    }
}
impl Body {
    pub fn new()-> Self {
        Body {
            head:Shape::None,
            hair:Shape::None,
            face:Shape::None,
            torso:Shape::None,
            arms:Shape::None,
            legs:Shape::None,
        }
    }
}
