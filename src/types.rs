/*!
# Types

This includes various enums related to the type of character you have

`Basic` is the basic type `Good` or `Bad`

`Normal` has elemental types

`Advanced` has elemental types

# Effectiveness

`Basic` has no need for effectiveness against types, so you can implement your own `Compare` if you like

```
use rpg_stat::types::Basic;
use rpg_stat::types::Compare;
use rpg_stat::attributes::Effectiveness;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign};
extern crate num;
use num::NumCast;

// Work around it
struct MyType(Basic);
// here it is.
impl Compare for MyType {
    type Type = Basic;
    // Plant Effectiveness against a target
    fn plant(other:Basic) -> Effectiveness {
        Effectiveness::Normal
    }
    /// Rock Effectiveness against a target
    fn rock(other:Basic) -> Effectiveness {
        Effectiveness::Normal
    }
    /// Water Effectiveness against a target
    fn water(other:Basic) -> Effectiveness {
        Effectiveness::Normal
    }
    /// Fire Effectiveness against a target
    fn fire(other:Basic) -> Effectiveness {
        Effectiveness::Normal
    }
    /// Electric Effectiveness against a target
    fn electric(other:Basic) -> Effectiveness {
        Effectiveness::Normal
    }
    /// Spirit Effectiveness against a target
    fn spirit(other:Basic) -> Effectiveness {
        Effectiveness::Normal
    }
    /// Light Effectiveness against a target
    fn light(other:Basic) -> Effectiveness {
        Effectiveness::Normal
    }
    /// Wind Effectiveness against a target
    fn wind(other:Basic) -> Effectiveness {
        Effectiveness::Normal
    }
    /// None Effectiveness against a target
    fn none(other:Basic) -> Effectiveness {
        Effectiveness::Normal
    }
    ///  Effectiveness against a target
    fn effectiveness(&self, other:Basic) -> Effectiveness {
        Effectiveness::Normal
    }
}
```

`Normal` implements this, see the `Compare` trait

This can be compared easily using the Compare trait

`Advanced` is currently the same as `Normal`
*/
use std::fmt;
use std::fmt::Debug;
extern crate num;
use serde::{Deserialize, Serialize};

#[cfg(feature = "fltkform")]
use fltk::{prelude::*, *};
#[cfg(feature = "fltkform")]
use fltk_form_derive::*;
#[cfg(feature = "fltkform")]
use fltk_form::FltkForm;

//module stuff
use crate::attributes::Effectiveness;
use crate::random::Random;

/*
# Compare
This trait is used by `Normal` and `Advanced`
*/
pub trait Compare {
    type Type;
    // Plant Effectiveness against a target
    fn plant(other:Self::Type) -> Effectiveness;
    /// Rock Effectiveness against a target
    fn rock(other:Self::Type) -> Effectiveness;
    /// Water Effectiveness against a target
    fn water(other:Self::Type) -> Effectiveness;
    /// Fire Effectiveness against a target
    fn fire(other:Self::Type) -> Effectiveness;
    /// Electric Effectiveness against a target
    fn electric(other:Self::Type) -> Effectiveness;
    /// Spirit Effectiveness against a target
    fn spirit(other:Self::Type) -> Effectiveness;
    /// Light Effectiveness against a target
    fn light(other:Self::Type) -> Effectiveness;
    /// Wind Effectiveness against a target
    fn wind(other:Self::Type) -> Effectiveness;
    /// None Effectiveness against a target
    fn none(other:Self::Type) -> Effectiveness;
    ///  Effectiveness against a target
    fn effectiveness(&self, other:Self::Type) -> Effectiveness;
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
///  `Basic`
///```rs:no_run                       
///    ------   _  __ __     
///      |  \ / |) |_ |_     
///      |   |  |  |_ _/     
///```
/// 
/// * Good     - good
/// * Bad    - bad
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
impl std::fmt::Display for Basic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let v = format!("{:?}", *self);
        let _u = v.split("::");
        write!(f, "{}", v.as_str())
    }
}

/* # `Normal`
```rs:no_run
    ------   _  __ __     
      |  \ / |) |_ |_     
      |   |  |  |_ _/     
```
 
 * rock     - earth type  
 * plant    - green type  
 * water    - liquid type 
 * fire     - lava type   
 * electric - lightning type
 * spirit   - holy type    
 * light    - laser type   
 * wind     - tornado type

## Compare

Implemented according to this chart:
<div>
<img src="https://raw.githubusercontent.com/1sra3l/rpg-stat/main/assets/type-effectiveness-chart.png" />
</div>

```
use rpg_stat::types::Normal as Type;
// to check effectiveness
use rpg_stat::types::Compare;
// need effectiveness too!
use rpg_stat::attributes::Effectiveness;

let rock = Type::Rock;
let wind = Type::Wind;
assert_eq!(rock.effectiveness(wind), Effectiveness::None);
assert_eq!(wind.effectiveness(rock), Effectiveness::Double);

```
*/

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[cfg_attr(feature = "fltkform", derive(FltkForm))]
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
impl Normal {
    #[allow(unused)]
    pub fn level_threshold(&self, current_level:f64) -> f64 {
        let mut threshold:f64 = current_level;
        let mut counter:u32 = 0;
        while threshold > 10.0 {
            threshold /= 10.0;
            counter += 1;
        }
        match *self {
            //TODO type based level 
            Normal::Rock => threshold += 0.0,
            Normal::Plant => threshold += 0.0,
            Normal::Water => threshold += 0.0,
            Normal::Fire => threshold += 0.0,
            Normal::Electric => threshold += 0.0,
            Normal::Spirit => threshold += 0.0,
            Normal::Light => threshold += 0.0,
            Normal::Wind => threshold += 0.0,
            _=> threshold += 0.0,
        }
        for _div in 0..counter {
            threshold *= 10.0;
        }
        threshold
    }
}
impl Random for Normal {
    type Type = Normal;
    fn random_type(&self) -> Self::Type {
        let max = 8;
        let val = self.random_rate(max);
        match val {
            0 => Normal::Rock,
            1 => Normal::Plant,
            2 => Normal::Water,
            3 => Normal::Fire,
            4 => Normal::Electric,
            5 => Normal::Spirit,
            7 => Normal::Light,
            8 => Normal::Wind,
            _=> Normal::None,
        }
    }
}
impl Default for Normal {
    /// Default to empty
    fn default() -> Self {
        Self::Rock
    }
}
impl std::fmt::Display for Normal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let v = format!("{:?}", *self);
        let _u = v.split("::");
        write!(f, "{}", v.as_str())
    }
}
impl Compare for Normal {
    type Type = Normal;
    ///  Plant Effectiveness against a target
    fn plant(other:Normal) -> Effectiveness {
        match other {
            Normal::Rock => Effectiveness::HalfExtra,
            Normal::Water => Effectiveness::Half,
            Normal::Fire => Effectiveness::None,
            Normal::Light => Effectiveness::Double,
            Normal::Wind => Effectiveness::Half,
            _=> Effectiveness::Normal,
        }
    }
    /// Rock Effectiveness against a target
    fn rock(other:Normal) -> Effectiveness {
        match other {
            Normal::Plant => Effectiveness::Half,
            Normal::Water => Effectiveness::HalfExtra,
            Normal::Fire => Effectiveness::Double,
            Normal::Electric => Effectiveness::Double,
            Normal::Light => Effectiveness::Half,
            Normal::Wind => Effectiveness::None,
            _=> Effectiveness::Normal,
        }
    }
    /// Water Effectiveness against a target
    fn water(other:Normal) -> Effectiveness {
        match other {
            Normal::Rock => Effectiveness::Double,
            Normal::Plant => Effectiveness::HalfExtra,
            Normal::Fire => Effectiveness::Double,
            Normal::Electric => Effectiveness::Half,
            Normal::Light => Effectiveness::None,
            Normal::Wind => Effectiveness::Half,
            _=> Effectiveness::Normal,
        }
    }
    /// Fire Effectiveness against a target
    fn fire(other:Normal) -> Effectiveness {
        match other {
            Normal::Rock => Effectiveness::Half,
            Normal::Plant => Effectiveness::Double,
            Normal::Water => Effectiveness::HalfExtra,
            Normal::Spirit => Effectiveness::None,
            Normal::Wind => Effectiveness::Half,
            _=> Effectiveness::Normal,
        }
    }
    /// Electric Effectiveness against a target
    fn electric(other:Normal) -> Effectiveness {
        match other {
            Normal::Rock => Effectiveness::Half,
            Normal::Plant => Effectiveness::HalfExtra,
            Normal::Water => Effectiveness::Double,
            Normal::Light => Effectiveness::None,
            Normal::Wind => Effectiveness::Half,
            _=> Effectiveness::Normal,
        }
    }
    /// Spirit Effectiveness against a target
    fn spirit(other:Normal) -> Effectiveness {
        match other {
            Normal::Water => Effectiveness::None,
            Normal::Fire => Effectiveness::Double,
            Normal::Electric => Effectiveness::Half,
            Normal::Spirit => Effectiveness::HalfExtra,
            Normal::Light => Effectiveness::Half,
            Normal::Wind => Effectiveness::Double,
            Normal::None => Effectiveness::None,
            _=> Effectiveness::Normal,
        }
    }
    /// Light Effectiveness against a target
    fn light(other:Normal) -> Effectiveness {
        match other {
            Normal::Rock => Effectiveness::Double,
            Normal::Plant => Effectiveness::None,
            Normal::Water => Effectiveness::Double,
            Normal::Fire => Effectiveness::None,
            Normal::Electric => Effectiveness::Half,
            Normal::Wind => Effectiveness::HalfExtra,
            Normal::None => Effectiveness::Half,
            _=> Effectiveness::Normal,
        }
    }
    ///  Effectiveness against a target
    fn wind(other:Normal) -> Effectiveness {
        match other {
            Normal::Rock => Effectiveness::Double,
            Normal::Plant => Effectiveness::Half,
            Normal::Water => Effectiveness::Double,
            Normal::Fire => Effectiveness::None,
            Normal::Spirit => Effectiveness::Half,
            Normal::Wind => Effectiveness::HalfExtra,
            _=> Effectiveness::Normal,
        }
    }
    ///  Effectiveness against a target
    fn none(other:Normal) -> Effectiveness {
        match other {
            Normal::Water => Effectiveness::Half,
            Normal::Fire => Effectiveness::Half,
            Normal::Electric => Effectiveness::Half,
            Normal::Spirit => Effectiveness::None,
            Normal::Light => Effectiveness::None,
            Normal::Wind => Effectiveness::Half,
            _=> Effectiveness::Normal,
        }
    }
    
    /// Match current Type to find effectiveness of the value
    fn effectiveness(&self, other:Normal) -> Effectiveness {
        match *self {
            Normal::Rock => Normal::rock(other),
            Normal::Plant => Normal::plant(other),
            Normal::Water => Normal::water(other),
            Normal::Fire => Normal::fire(other),
            Normal::Electric => Normal::electric(other),
            Normal::Spirit => Normal::spirit(other),
            Normal::Light => Normal::light(other),
            Normal::Wind => Normal::wind(other),
            Normal::None => Normal::none(other),
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[cfg_attr(feature = "fltkform", derive(FltkForm))]
///  `Advanced`
pub enum Advanced {
    /// Feline - Cat type
    Feline,
    /// Canine - Dog type
    Canine,
    /// Rodent - mouse type
    Rodent,
    /// Primate - monkey type
    Primate,
    /// Bug - creepy crawly type
    Bug,
    /// Amphibian - frog/salamander type
    Amphibian,
    /// Reptile -  type
    Reptile,
    /// Fish - 
    Fish,
    /// Dragon - 
    Dragon,
    /// Legendary - 
    Legendary,
    /// Plasma - 
    Plasma,
    /// Magma - 
    Magma,
    /// Crystal - 
    Crystal,
    /// Laser - 
    Laser,
    /// Tech - 
    Tech,
    /// Leaf - 
    Leaf,
    /// Patch - 
    Patch,
    /// Undead - 
    Undead,
    /// Star - 
    Star,
    /// Galactic - 
    Galactic,
    /// Kaiju - 
    Kaiju,
    /// Xeno - 
    Xeno,
    /// Paper - 
    Paper,
    /// Shifter - 
    Shifter,
    /// Gravity - 
    Gravity,
    /// Life - 
    Life,
    /// Food - 
    Food,
    /// Death - 
    Death,
    /// Mana - 
    Mana,
    /// Bubble - 
    Bubble,
    /// Seed - 
    Seed,
    /// Bean - 
    Bean,
    /// Clay - 
    Clay,
    /// Steel - 
    Steel,
    /// Iron - 
    Iron,
    /// Vine - 
    Vine,
    /// Tree - 
    Tree,
    /// River - 
    River,
    /// Ocean - 
    Ocean,
    /// Ember - 
    Ember,
    /// Lava - 
    Lava,
    /// Spark - 
    Spark,
    /// Lightning - 
    Lightning,
    /// Holy - 
    Holy,
    /// Unholy - 
    Unholy,
    /// Sunrise - 
    Sunrise,
    /// Sunset - 
    Sunset,
    /// Moonrise - 
    Moonrise,
    /// Moonset - 
    Moonset,
    /// Tornado - 
    Tornado,
    /// Breeze - 
    Breeze,
    /// Blustry - 
    Blustry,
    None,
}
impl Default for Advanced {
    fn default() -> Self {
        Self::None
    }
}
impl std::fmt::Display for Advanced {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let v = format!("{:?}", *self);
        let _u = v.split("::");
        write!(f, "{}", v.as_str())
    }
}
impl Random for Advanced {
    type Type = Advanced;
    fn random_type(&self) -> Self::Type {
        let max = 52;
        let val = self.random_rate(max);
        match val {
            0 => Advanced::Feline,
            1 => Advanced::Canine,
            2 => Advanced::Rodent,
            3 => Advanced::Primate,
            4 => Advanced::Bug,
            5 => Advanced::Amphibian,
            6 => Advanced::Reptile,
            7 => Advanced::Fish,
            8 => Advanced::Dragon,
            9 => Advanced::Legendary,
            10 => Advanced::Plasma,
            11 => Advanced::Magma,
            12 => Advanced::Crystal,
            13 => Advanced::Laser,
            14 => Advanced::Tech,
            15 => Advanced::Leaf,
            16 => Advanced::Patch,
            17 => Advanced::Undead,
            18 => Advanced::Star,
            19 => Advanced::Galactic,
            20 => Advanced::Kaiju,
            21 => Advanced::Xeno,
            22 => Advanced::Paper,
            23 => Advanced::Shifter,
            24 => Advanced::Gravity,
            25 => Advanced::Life,
            26 => Advanced::Food,
            27 => Advanced::Death,
            28 => Advanced::Mana,
            29 => Advanced::Bubble,
            30 => Advanced::Seed,
            31 => Advanced::Bean,
            32 => Advanced::Clay,
            33 => Advanced::Steel,
            34 => Advanced::Iron,
            35 => Advanced::Vine,
            36 => Advanced::Tree,
            37 => Advanced::River,
            38 => Advanced::Ocean,
            39 => Advanced::Ember,
            40 => Advanced::Lava,
            41 => Advanced::Spark,
            42 => Advanced::Lightning,
            43 => Advanced::Holy,
            44 => Advanced::Unholy,
            45 => Advanced::Sunrise,
            46 => Advanced::Sunset,
            47 => Advanced::Moonrise,
            48 => Advanced::Moonset,
            49 => Advanced::Tornado,
            50 => Advanced::Breeze,
            51 => Advanced::Blustry,
            _=> Advanced::None,
        }
    }
    
}
