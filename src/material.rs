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
pub enum Item {
   /// Hood armor
   Hood,
   /// Jaw armor
   Jaw,
   /// Armor to protect gaps
   Joint,
   /// Collar armor
   Collar,
   /// Upper arm armor below the shoulder armor
   UpperArm
   /// Elbow Armor
   Elbow,
   /// Pants armor
   Pants,
   /// Belly armor
   Belly,
   /// Front torso armor
   Chestplate,
   /// Torso armor
   Torso,
   /// Waist and hip armor
   Hip,
   /// Knee armor
   Knee,
   /// Shin armor
   Shin,
   /// Shoe armor
   Shoe,
   /// Shoulder / upper arm guard
   Shoulder,
   /// Forearm guard
   Forearm,
   /// Hand guard
   Hand,
   /// Shirt armor
   Shirt,
   /// Head Armor
   Head,
   /// Neck Armor
   Neck,
   /// Face armor
   Face,
   /// The coat worn over armor
   Coat,
   /// Hanging upper thigh plate armor
   Thigh,
   /// Nothing
   None,
}
/*
# Part 

*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, Deserialize, Serialize)]
pub enum Part {
   /// Hood armor
   Hood,
   /// Jaw / throat armor
   Bevor,
   /// Circular plate armor protecting various areas
   Rondel,
   /// Collar armor
   Gorget,
   /// Upper arm armor below the shoulder armor
   Rerebrace
   /// Elbow Armor
   Couter,
   /// Pant armor
   Chausses,
   /// Belly armor
   Plackart,
   /// Thigh armor
   Cuisses,
   /// Front torso armor
   Chestplate,
   /// Torso armor
   Curiass,
   /// Waist and hip armor
   Fauld,
   /// Knee armor
   Poleyn,
   /// Shin armor
   Greaves,
   /// Shoe armor
   Sabaton,
   /// Shoulder / upper arm guard
   Spaulders,
   /// Shoulder / armpit (back/chest optional) armor
   Pauldron,
   /// Forearm guard
   Vambrace,
   /// Hand guard
   Gauntlets,
   /// Shirt armor
   Hauberk,
   /// Head Armor
   Helmet,
   /// Neck Armor
   Neckguard,
   /// Face armor
   Faceplate,
   /// The coat worn over armor
   Coat,
   /// Hanging upper thigh plate armor
   Tasset
   /// Nothing
   None,
}

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
    /// 
    Cloth,
    /// 
    Leather,
    /// 
    Metal,
    /// 
    Obsidian,
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
    /// Interlocking rings of metal forming a cloth-like armor
    Mail,
    /// Often metal, small rounded overlapping plates.
    Scale,
    /// Large metal pieces forming entire parts
    Plate,
    /// 
    Splint,
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
    /// 
    Diamond,
    /// 
    Gold,
    /// 
    Gem,
    /// 
    Meteorite,
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
