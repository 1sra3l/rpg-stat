/*!
# Types

This includes various enums related to the type of character you have

`Basic` is the basic type `Good` or `Bad`

`Normal` has elemental types

`Advanced` has elemental types

# Effectiveness

`Basic` has no need for effectiveness against types, so you can implement your own `Compare` if you like

```
use rpgstat::types::Basic;
use rpgstat::types::Compare;
use rpgstat::attributes::Effectiveness;
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
//use std::fmt::Display;
//use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign};
extern crate num;
///use num::NumCast;
use serde::{Deserialize, Serialize};
//use strum::IntoEnumIterator;
use strum_macros::EnumIter;
#[cfg(feature = "fltkform")]
use fltk::{prelude::*, *};
#[cfg(feature = "fltkform")]
use fltk_form_derive::*;
#[cfg(feature = "fltkform")]
use fltk_form::{FltkForm, HasProps};

//our stuff
use crate::attributes::Effectiveness;

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
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, Deserialize, Serialize)]
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
impl fmt::Display for Basic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let v:String;
        match *self {
            Basic::Good => v = String::from("Good"),
            Basic::Bad => v = String::from("Bad"),
        }
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
use rpgstat::types::Normal as Type;
// to check effectiveness
use rpgstat::types::Compare;
// need effectiveness too!
use rpgstat::attributes::Effectiveness;

let rock = Type::Rock;
let wind = Type::Wind;
assert_eq!(rock.effectiveness(wind), Effectiveness::None);
assert_eq!(wind.effectiveness(rock), Effectiveness::Double);

```
*/

use crate::random::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, Deserialize, Serialize)]
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
        for div in 0..counter {
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
impl fmt::Display for Normal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let v:String;
        match *self {
            Normal::Rock => v = String::from("Rock"),
            Normal::Plant => v = String::from("Plant"),
            Normal::Water => v = String::from("Water"),
            Normal::Fire => v = String::from("Fire"),
            Normal::Electric => v = String::from("Electric"),
            Normal::Spirit => v = String::from("Spirit"),
            Normal::Light => v = String::from("Light"),
            Normal::Wind => v = String::from("Wind"),
            Normal::None => v = String::from("None"),
        }
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, Deserialize, Serialize)]
#[cfg_attr(feature = "fltkform", derive(FltkForm))]
///  `Advanced`
///```rs:no_run                     
///    ------   _  __ __     
///      |  \ / |) |_ |_     
///      |   |  |  |_ _/     
///```
/// 
/// * rock     - earth type  
/// * plant    - green type  
/// * water    - liquid type 
/// * fire     - lava type   
/// * electric - lightning type
/// * spirit   - holy type    
/// * light    - laser type   
/// * wind     - tornado type 
pub enum Advanced {
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
impl Default for Advanced {
    fn default() -> Self {
        Self::None
    }
}
impl fmt::Display for Advanced {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let v:String;
        match *self {
            Advanced::Rock => v = String::from("Rock"),
            Advanced::Plant => v = String::from("Plant"),
            Advanced::Water => v = String::from("Water"),
            Advanced::Fire => v = String::from("Fire"),
            Advanced::Electric => v = String::from("Electric"),
            Advanced::Spirit => v = String::from("Spirit"),
            Advanced::Light => v = String::from("Light"),
            Advanced::Wind => v = String::from("Wind"),
            Advanced::None => v = String::from("None"),
            //Advanced:: => v = String::from(""),
        }
        write!(f, "{}", v.as_str())
    }
}
impl Compare for Advanced {
    type Type = Advanced;
    ///  Plant Effectiveness against a target
    fn plant(other:Advanced) -> Effectiveness {
        match other {
            Advanced::Rock => Effectiveness::HalfExtra,
            Advanced::Water => Effectiveness::Half,
            Advanced::Fire => Effectiveness::None,
            Advanced::Light => Effectiveness::Double,
            Advanced::Wind => Effectiveness::Half,
            _=> Effectiveness::Normal,
        }
    }
    /// Rock Effectiveness against a target
    fn rock(other:Advanced) -> Effectiveness {
        match other {
            Advanced::Plant => Effectiveness::Half,
            Advanced::Water => Effectiveness::HalfExtra,
            Advanced::Fire => Effectiveness::Double,
            Advanced::Electric => Effectiveness::Double,
            Advanced::Light => Effectiveness::Half,
            Advanced::Wind => Effectiveness::None,
            _=> Effectiveness::Normal,
        }
    }
    /// Water Effectiveness against a target
    fn water(other:Advanced) -> Effectiveness {
        match other {
            Advanced::Rock => Effectiveness::Double,
            Advanced::Plant => Effectiveness::HalfExtra,
            Advanced::Fire => Effectiveness::Double,
            Advanced::Electric => Effectiveness::Half,
            Advanced::Light => Effectiveness::None,
            Advanced::Wind => Effectiveness::Half,
            _=> Effectiveness::Normal,
        }
    }
    /// Fire Effectiveness against a target
    fn fire(other:Advanced) -> Effectiveness {
        match other {
            Advanced::Rock => Effectiveness::Half,
            Advanced::Plant => Effectiveness::Double,
            Advanced::Water => Effectiveness::HalfExtra,
            Advanced::Spirit => Effectiveness::None,
            Advanced::Wind => Effectiveness::Half,
            _=> Effectiveness::Normal,
        }
    }
    /// Electric Effectiveness against a target
    fn electric(other:Advanced) -> Effectiveness {
        match other {
            Advanced::Rock => Effectiveness::Half,
            Advanced::Plant => Effectiveness::HalfExtra,
            Advanced::Water => Effectiveness::Double,
            Advanced::Light => Effectiveness::None,
            Advanced::Wind => Effectiveness::Half,
            _=> Effectiveness::Normal,
        }
    }
    /// Spirit Effectiveness against a target
    fn spirit(other:Advanced) -> Effectiveness {
        match other {
            Advanced::Water => Effectiveness::None,
            Advanced::Fire => Effectiveness::Double,
            Advanced::Electric => Effectiveness::Half,
            Advanced::Spirit => Effectiveness::HalfExtra,
            Advanced::Light => Effectiveness::Half,
            Advanced::Wind => Effectiveness::Double,
            Advanced::None => Effectiveness::None,
            _=> Effectiveness::Normal,
        }
    }
    /// Light Effectiveness against a target
    fn light(other:Advanced) -> Effectiveness {
        match other {
            Advanced::Rock => Effectiveness::Double,
            Advanced::Plant => Effectiveness::None,
            Advanced::Water => Effectiveness::Double,
            Advanced::Fire => Effectiveness::None,
            Advanced::Electric => Effectiveness::Half,
            Advanced::Wind => Effectiveness::HalfExtra,
            Advanced::None => Effectiveness::Half,
            _=> Effectiveness::Normal,
        }
    }
    ///  Effectiveness against a target
    fn wind(other:Advanced) -> Effectiveness {
        match other {
            Advanced::Rock => Effectiveness::Double,
            Advanced::Plant => Effectiveness::Half,
            Advanced::Water => Effectiveness::Double,
            Advanced::Fire => Effectiveness::None,
            Advanced::Spirit => Effectiveness::Half,
            Advanced::Wind => Effectiveness::HalfExtra,
            _=> Effectiveness::Normal,
        }
    }
    ///  Effectiveness against a target
    fn none(other:Advanced) -> Effectiveness {
        match other {
            Advanced::Water => Effectiveness::Half,
            Advanced::Fire => Effectiveness::Half,
            Advanced::Electric => Effectiveness::Half,
            Advanced::Spirit => Effectiveness::None,
            Advanced::Light => Effectiveness::None,
            Advanced::Wind => Effectiveness::Half,
            _=> Effectiveness::Normal,
        }
    }
    
    /// Match current Type to find effectiveness of the value
    fn effectiveness(&self, other:Advanced) -> Effectiveness {
        match *self {
            Advanced::Rock => Advanced::rock(other),
            Advanced::Plant => Advanced::plant(other),
            Advanced::Water => Advanced::water(other),
            Advanced::Fire => Advanced::fire(other),
            Advanced::Electric => Advanced::electric(other),
            Advanced::Spirit => Advanced::spirit(other),
            Advanced::Light => Advanced::light(other),
            Advanced::Wind => Advanced::wind(other),
            Advanced::None => Advanced::none(other),
        }
    }
}
