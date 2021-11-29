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

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Body {
    pub head:Shape,
    pub hair:Shape,
    pub face:Shape,
    pub torso:Shape,
    pub arms:Shape,
    pub legs:Shape,
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
