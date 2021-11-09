/*!
# Types

This includes various enums related to the type of life-stage/element/effect/special

All have a default()
All have implemented fmt::Display


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
use rand::{thread_rng, Rng};
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
    Never,
}
impl Rate {
    pub fn random_rate(value:u32) -> u32 {
        let mut rng = thread_rng();
        let n: u32 = rng.gen_range(0..value);
        n
    }
    /*
    
    */
    pub fn worked(&self) -> bool {
        match *self {
            Rate::Always => return true, // 100%
            Rate::Usually => {
                if Self::random_rate(9) > 1 {
                    return true
                }
                return false
            },
            Rate::Often => { // 75%
                if Self::random_rate(3) > 0 {
                    return true
                }
                return false
            },
            Rate::Some => { // 50%
                if Self::random_rate(1) == 1 {
                    return true
                }
                return false
            },
            Rate::Hardly => {
                if Self::random_rate(3) == 0 {
                    return true
                }
                return false
            },
            Rate::Barely => {
                if Self::random_rate(9) == 0 {
                    return true
                }
                return false
            },
            Rate::Never => return false, // 0%
        }
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, Deserialize, Serialize)]
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
/// 
pub enum Element {
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
impl Default for Element {
    /// Default to empty
    fn default() -> Self {
        Self::Rock
    }
}
impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let v:String;
        match *self {
            Element::Rock => v = String::from("Rock"),
            Element::Plant => v = String::from("Plant"),
            Element::Water => v = String::from("Water"),
            Element::Fire => v = String::from("Fire"),
            Element::Electric => v = String::from("Electric"),
            Element::Spirit => v = String::from("Spirit"),
            Element::Light => v = String::from("Light"),
            Element::Wind => v = String::from("Wind"),
            Element::None => v = String::from("None"),
        }
        write!(f, "{}", v.as_str())
    }
}
impl Element {
    /// Return the "opposite" enum from a "matching" `String`
    pub fn opposite_from_string(creature_type:String) -> Element {
        creature_type.to_lowercase();
        let element_type = Element::from_string(creature_type);
        return Element::opposite_from_type(element_type)
    }

    /// Return the opposite enum from a "matching" `Element`
    pub fn opposite_from_type(creature_type:Element) -> Element {
        match creature_type {
            Element::Plant => return Element::Electric,
            Element::Water => return Element::Fire,
            Element::Fire => return Element::Water,
            Element::Electric => return Element::Plant,
            Element::Spirit => return Element::Wind,
            Element::Light => return Element::Rock,
            Element::Wind => return Element::Spirit,
            Element::Rock => return Element::Light,
            _=> return Element::None
        }
    }

    /// Return an enum from a "matching" `String`
    pub fn from_string(creature_type:String) -> Element {
        creature_type.to_lowercase();
        if creature_type == "plant" {
            return Element::Plant
        } else if creature_type == "water" {
            return Element::Water
        } else if creature_type == "fire" {
            return Element::Fire
        } else if creature_type == "electric" {
            return Element::Electric
        } else if creature_type == "spirit" {
            return Element::Spirit
        } else if creature_type == "light" {
            return Element::Light
        } else if creature_type == "wind" {
            return Element::Wind
        } else if creature_type == "rock" {
            return Element::Rock
        } else {
            return Element::None
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, Deserialize, Serialize)]
/// Specials are just types of attack, coupled with the `Element`
/// These enums are used in determining the effects of the attack and which animations to use
pub enum Special {
    /// The generic wrestling attack
    Tackle,
    /// Picking up and throwing an enemy a short distance
    Toss,
    /// Picking up and throwing an enemy very hard
    Throw,
    /// using claws, beaks or other sharp objects
    Slash,
    /// Reducing an enemy's core temperature and effecting their skin
    Freeze,
    /// Increasing an enemy's core temperature and effecting their skin
    Burn,
    /// Much more intensity than burn
    Melt,
    /// more intensity than tackle
    Crush,
    /// WAY more intensity than crush
    Grind,
    /// generic physical hit
    Hit,
    // not as powerful as Hit
    Slap,
    /// almost as powerful as Hit
    Smack,
    /// More powerful than Hit 
    Whip,
    /// More powerful than Slash
    Slice,
    /// disorienting attack
    Spin,
    /// more powerful than Spin
    Blur,
    /// Similar in power to Whip
    Strike,
    /// Much less powerful than Freeze
    Splash,
}
impl Default for Special {
    /// Default to empty
    fn default() -> Self {
        Self::Tackle
    }
}
impl fmt::Display for Special {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let v:String;
        match *self {
            Special::Toss => v = "Toss".to_string(),
            Special::Throw => v = "Throw".to_string(),
            Special::Slash => v = "Slash".to_string(),
            Special::Freeze => v = "Freeze".to_string(),
            Special::Burn => v = "Burn".to_string(),
            Special::Melt => v = "Melt".to_string(),
            Special::Crush => v = "Crush".to_string(),
            Special::Grind => v = "Grind".to_string(),
            Special::Hit => v = "Hit".to_string(),
            Special::Slap => v = "Slap".to_string(),
            Special::Smack => v = "Smack".to_string(),
            Special::Whip => v = "whip".to_string(),
            Special::Slice => v = "Slice".to_string(),
            Special::Tackle => v = "Tackle".to_string(),
            Special::Spin => v = "Spin".to_string(),
            Special::Blur => v = "Blur".to_string(),
            Special::Strike => v = "Strike".to_string(),
            Special::Splash => v = "Splash".to_string(),
            // Special:: => v = "".to_string(),
        }
        write!(f, "{}", v.as_str())
    }
}
impl Special {
    /// Get a special from a string (for saving/loading)
    /// 
    /// **Note:** *It does not matter what case comes into the function, everything is converted to lowercase via `to_lowercase()`*
    pub fn from_string(special:String) -> Special {
        special.to_lowercase();
        if special == "toss" {
            return Special::Toss
        } else if special == "throw" {
            return Special::Throw
        } else if special == "slash" {
            return Special::Slash
        } else if special == "freeze" {
            return Special::Freeze
        } else if special == "burn" {
            return Special::Burn
        } else if special == "melt" {
            return Special::Melt
        } else if special == "crush" {
            return Special::Crush
        } else if special == "grind" {
            return Special::Grind
        } else if special == "hit" {
            return Special::Hit
        } else if special == "slap" {
            return Special::Slap
        } else if special == "smack" {
            return Special::Smack
        } else if special == "whip" {
            return Special::Whip
        } else if special == "slice" {
            return Special::Slice
        } else if special == "spin" {
            return Special::Spin
        } else if special == "blur" {
            return Special::Blur
        } else if special == "strike" {
            return Special::Strike
        } else if special == "splash" {
            return Special::Splash
        }
        Special::Tackle
    }

    /// get special attack mana point cost
    pub fn mp_cost(&self) -> f32 {
        match self {
            Special::Toss => 1.0,
            Special::Throw => 1.0,
            Special::Strike => 1.0,
            Special::Tackle => 1.0,
            Special::Spin => 1.0,

            Special::Slash =>  5.0,
            Special::Burn =>  5.0,
            Special::Blur => 5.0,
            Special::Splash =>  5.0,
            Special::Crush =>  5.0,
            Special::Hit =>  5.0,
            Special::Slap =>  5.0,
            Special::Whip =>  5.0,

            Special::Grind =>  7.0,
            Special::Smack =>  7.0,
            Special::Melt =>  7.0,
            Special::Slice =>  7.0,
            Special::Freeze => 7.0,
            // Special:: => 1.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, Deserialize, Serialize)]
/// This enum defines our different types of attack/item/special effects
pub enum Effect
{
    /// no effect aka a player or something else that should not have an effect, like a key
    None,
    /// add or remove HP based on the attack/item/special's HP
    HP,
    /// add or remove MP based on the attack/item/special's MP
    MP,
    /// add or remove XP. enemies all have this effect
    XP,
/// # These next five continously drain HP/MP until remedy, or duration runs out
    /// hp drain
    Burn,
    /// hp drain, very minor mp drain
    Poison,
    /// minor hp drain, very minor mp drain, no movement
    Freeze,
    /// hp drain, minor mp drain
    Sick,
    /// mp drain
    Sap,
/// # These next two continuously add HP/MP
    /// mp add
    Bless,
    /// hp add
    Heal,
/// # Blocker/locker effects
    /// no movement
    Stuck,
    /// no attack
    Bound,
    /// no mana attack
    Blocked,
    /// a lock to prevent access
    Locked,
}
impl Default for Effect {
    /// Default to empty
    fn default() -> Self {
        Self::None
    }
}
impl fmt::Display for Effect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let v:String;
        match *self {
            Effect::None => v = "None".to_string(),
            Effect::HP => v = "HP".to_string(),
            Effect::MP => v = "MP".to_string(),
            Effect::XP => v = "XP".to_string(),
            Effect::Burn => v = "Burn".to_string(),
            Effect::Poison => v = "Poison".to_string(),
            Effect::Freeze => v = "Freeze".to_string(),
            Effect::Sick => v = "Sick".to_string(),
            Effect::Sap => v = "Sap".to_string(),
            Effect::Bless => v = "Bless".to_string(),
            Effect::Heal => v = "Heal".to_string(),
            Effect::Stuck => v = "Stuck".to_string(),
            Effect::Bound => v = "Bound".to_string(),
            Effect::Blocked => v = "Blocked".to_string(),
            Effect::Locked => v = "Locked".to_string(),
        }
        write!(f, "{}", v.as_str())
    }
}
impl Effect {
    /// Turn the `String` into an `Effect`
    pub fn from_string(effect:String) -> Effect {
        effect.to_lowercase();
        if effect == "hp" {
            return Effect::HP
        } else if effect == "mp" {
            return Effect::MP
        } else if effect == "xp" {
            return Effect::XP
        } else if effect == "burn" {
            return Effect::Burn
        } else if effect == "poison" {
            return Effect::Poison
        } else if effect == "freeze" {
            return Effect::Freeze
        } else if effect == "sick" {
            return Effect::Sick
        } else if effect == "sap" {
            return Effect::Sap
        } else if effect == "bless" {
            return Effect::Bless
        } else if effect == "heal" {
            return Effect::Heal
        } else if effect == "stuck" {
            return Effect::Stuck
        } else if effect == "bound" {
            return Effect::Bound
        } else if effect == "blocked" {
            return Effect::Blocked
        } else if effect == "locked" {
            return Effect::Locked
        }
        Effect::None
    }
}
