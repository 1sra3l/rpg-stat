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
    + num::NumCast> Compare<T> for MyType {
    type Type = Basic;
    // Plant Effectiveness against a target
    fn plant(other:Basic, value:T) -> Effectiveness<T> {
        Effectiveness::Normal(value)
    }
    /// Rock Effectiveness against a target
    fn rock(other:Basic, value:T) -> Effectiveness<T> {
        Effectiveness::Normal(value)
    }
    /// Water Effectiveness against a target
    fn water(other:Basic, value:T) -> Effectiveness<T> {
        Effectiveness::Normal(value)
    }
    /// Fire Effectiveness against a target
    fn fire(other:Basic, value:T) -> Effectiveness<T> {
        Effectiveness::Normal(value)
    }
    /// Electric Effectiveness against a target
    fn electric(other:Basic, value:T) -> Effectiveness<T> {
        Effectiveness::Normal(value)
    }
    /// Spirit Effectiveness against a target
    fn spirit(other:Basic, value:T) -> Effectiveness<T> {
        Effectiveness::Normal(value)
    }
    /// Light Effectiveness against a target
    fn light(other:Basic, value:T) -> Effectiveness<T> {
        Effectiveness::Normal(value)
    }
    /// Wind Effectiveness against a target
    fn wind(other:Basic, value:T) -> Effectiveness<T> {
        Effectiveness::Normal(value)
    }
    /// None Effectiveness against a target
    fn none(other:Basic, value:T) -> Effectiveness<T> {
        Effectiveness::Normal(value)
    }
    ///  Effectiveness against a target
    fn effectiveness(&self, other:Basic, value:T) -> Effectiveness<T> {
        Effectiveness::Normal(value)
    }
}
```

`Normal` implements this according to the chart:

<div>
<img src="https://raw.githubusercontent.com/1sra3l/rpg-stat/main/assets/type-effectiveness-chart.png" />
</div>

This can be compared easily using the Compare trait

`Advanced` is currently the same as `Normal`
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

//our stuff
use crate::attributes::Effectiveness;

/*
# Compare
This trait is used by `Normal` and `Advanced`
*/
pub trait Compare<T:Copy 
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
    type Type;
    // Plant Effectiveness against a target
    fn plant(other:Self::Type, value:T) -> Effectiveness<T>;
    /// Rock Effectiveness against a target
    fn rock(other:Self::Type, value:T) -> Effectiveness<T>;
    /// Water Effectiveness against a target
    fn water(other:Self::Type, value:T) -> Effectiveness<T>;
    /// Fire Effectiveness against a target
    fn fire(other:Self::Type, value:T) -> Effectiveness<T>;
    /// Electric Effectiveness against a target
    fn electric(other:Self::Type, value:T) -> Effectiveness<T>;
    /// Spirit Effectiveness against a target
    fn spirit(other:Self::Type, value:T) -> Effectiveness<T>;
    /// Light Effectiveness against a target
    fn light(other:Self::Type, value:T) -> Effectiveness<T>;
    /// Wind Effectiveness against a target
    fn wind(other:Self::Type, value:T) -> Effectiveness<T>;
    /// None Effectiveness against a target
    fn none(other:Self::Type, value:T) -> Effectiveness<T>;
    ///  Effectiveness against a target
    fn effectiveness(&self, other:Self::Type, value:T) -> Effectiveness<T>;
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

///  `Normal`
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, Deserialize, Serialize)]
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
    + num::NumCast> Compare<T> for Normal {
    type Type = Normal;
    ///  Plant Effectiveness against a target
    fn plant(other:Normal, value:T) -> Effectiveness<T> {
        match other {
            Normal::Rock => return Effectiveness::HalfExtra(value),
            Normal::Water => return Effectiveness::Half(value),
            Normal::Fire => return Effectiveness::None(value),
            Normal::Light => return Effectiveness::Double(value),
            Normal::Wind => return Effectiveness::Half(value),
            _=> return Effectiveness::Normal(value),
        }
    }
    /// Rock Effectiveness against a target
    fn rock(other:Normal, value:T) -> Effectiveness<T> {
        match other {
            Normal::Plant => return Effectiveness::Half(value),
            Normal::Water => return Effectiveness::HalfExtra(value),
            Normal::Fire => return Effectiveness::Double(value),
            Normal::Electric => return Effectiveness::Double(value),
            Normal::Light => return Effectiveness::Half(value),
            Normal::Wind => return Effectiveness::None(value),
            _=> return Effectiveness::Normal(value),
        }
    }
    /// Water Effectiveness against a target
    fn water(other:Normal, value:T) -> Effectiveness<T> {
        match other {
            Normal::Rock => return Effectiveness::Double(value),
            Normal::Plant => return Effectiveness::HalfExtra(value),
            Normal::Fire => return Effectiveness::Double(value),
            Normal::Electric => return Effectiveness::Half(value),
            Normal::Light => return Effectiveness::None(value),
            Normal::Wind => return Effectiveness::Half(value),
            _=> return Effectiveness::Normal(value),
        }
    }
    /// Fire Effectiveness against a target
    fn fire(other:Normal, value:T) -> Effectiveness<T> {
        match other {
            Normal::Rock => return Effectiveness::Half(value),
            Normal::Plant => return Effectiveness::Double(value),
            Normal::Water => return Effectiveness::HalfExtra(value),
            Normal::Spirit => return Effectiveness::None(value),
            Normal::Wind => return Effectiveness::Half(value),
            _=> return Effectiveness::Normal(value),
        }
    }
    /// Electric Effectiveness against a target
    fn electric(other:Normal, value:T) -> Effectiveness<T> {
        match other {
            Normal::Rock => return Effectiveness::Half(value),
            Normal::Plant => return Effectiveness::HalfExtra(value),
            Normal::Water => return Effectiveness::Double(value),
            Normal::Light => return Effectiveness::None(value),
            Normal::Wind => return Effectiveness::Half(value),
            _=> return Effectiveness::Normal(value),
        }
    }
    /// Spirit Effectiveness against a target
    fn spirit(other:Normal, value:T) -> Effectiveness<T> {
        match other {
            Normal::Water => return Effectiveness::None(value),
            Normal::Fire => return Effectiveness::Double(value),
            Normal::Electric => return Effectiveness::Half(value),
            Normal::Spirit => return Effectiveness::HalfExtra(value),
            Normal::Light => return Effectiveness::Half(value),
            Normal::Wind => return Effectiveness::Double(value),
            Normal::None => return Effectiveness::None(value),
            _=> return Effectiveness::Normal(value),
        }
    }
    /// Light Effectiveness against a target
    fn light(other:Normal, value:T) -> Effectiveness<T> {
        match other {
            Normal::Rock => return Effectiveness::Double(value),
            Normal::Plant => return Effectiveness::None(value),
            Normal::Water => return Effectiveness::Double(value),
            Normal::Fire => return Effectiveness::None(value),
            Normal::Electric => return Effectiveness::Half(value),
            Normal::Wind => return Effectiveness::HalfExtra(value),
            Normal::None => return Effectiveness::Half(value),
            _=> return Effectiveness::Normal(value),
        }
    }
    ///  Effectiveness against a target
    fn wind(other:Normal, value:T) -> Effectiveness<T> {
        match other {
            Normal::Rock => return Effectiveness::Double(value),
            Normal::Plant => return Effectiveness::Half(value),
            Normal::Water => return Effectiveness::Double(value),
            Normal::Fire => return Effectiveness::None(value),
            Normal::Spirit => return Effectiveness::Half(value),
            Normal::Wind => return Effectiveness::HalfExtra(value),
            _=> return Effectiveness::Normal(value),
        }
    }
    ///  Effectiveness against a target
    fn none(other:Normal, value:T) -> Effectiveness<T> {
        match other {
            Normal::Water => return Effectiveness::Half(value),
            Normal::Fire => return Effectiveness::Half(value),
            Normal::Electric => return Effectiveness::Half(value),
            Normal::Spirit => return Effectiveness::None(value),
            Normal::Light => return Effectiveness::None(value),
            Normal::Wind => return Effectiveness::Half(value),
            _=> return Effectiveness::Normal(value),
        }
    }
    
    /// Match current Type to find effectiveness of the value
    fn effectiveness(&self, other:Normal, value:T) -> Effectiveness<T> {
        match *self {
            Normal::Rock => return Normal::rock(other, value),
            Normal::Plant => return Normal::plant(other, value),
            Normal::Water => return Normal::water(other, value),
            Normal::Fire => return Normal::fire(other, value),
            Normal::Electric => return Normal::electric(other, value),
            Normal::Spirit => return Normal::spirit(other, value),
            Normal::Light => return Normal::light(other, value),
            Normal::Wind => return Normal::wind(other, value),
            Normal::None => return Normal::none(other, value),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, Deserialize, Serialize)]
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
    + num::NumCast> Compare<T> for Advanced {
    type Type = Advanced;
    ///  Plant Effectiveness against a target
    fn plant(other:Advanced, value:T) -> Effectiveness<T> {
        match other {
            Advanced::Rock => return Effectiveness::HalfExtra(value),
            Advanced::Water => return Effectiveness::Half(value),
            Advanced::Fire => return Effectiveness::None(value),
            Advanced::Light => return Effectiveness::Double(value),
            Advanced::Wind => return Effectiveness::Half(value),
            _=> return Effectiveness::Normal(value),
        }
    }
    /// Rock Effectiveness against a target
    fn rock(other:Advanced, value:T) -> Effectiveness<T> {
        match other {
            Advanced::Plant => return Effectiveness::Half(value),
            Advanced::Water => return Effectiveness::HalfExtra(value),
            Advanced::Fire => return Effectiveness::Double(value),
            Advanced::Electric => return Effectiveness::Double(value),
            Advanced::Light => return Effectiveness::Half(value),
            Advanced::Wind => return Effectiveness::None(value),
            _=> return Effectiveness::Normal(value),
        }
    }
    /// Water Effectiveness against a target
    fn water(other:Advanced, value:T) -> Effectiveness<T> {
        match other {
            Advanced::Rock => return Effectiveness::Double(value),
            Advanced::Plant => return Effectiveness::HalfExtra(value),
            Advanced::Fire => return Effectiveness::Double(value),
            Advanced::Electric => return Effectiveness::Half(value),
            Advanced::Light => return Effectiveness::None(value),
            Advanced::Wind => return Effectiveness::Half(value),
            _=> return Effectiveness::Normal(value),
        }
    }
    /// Fire Effectiveness against a target
    fn fire(other:Advanced, value:T) -> Effectiveness<T> {
        match other {
            Advanced::Rock => return Effectiveness::Half(value),
            Advanced::Plant => return Effectiveness::Double(value),
            Advanced::Water => return Effectiveness::HalfExtra(value),
            Advanced::Spirit => return Effectiveness::None(value),
            Advanced::Wind => return Effectiveness::Half(value),
            _=> return Effectiveness::Normal(value),
        }
    }
    /// Electric Effectiveness against a target
    fn electric(other:Advanced, value:T) -> Effectiveness<T> {
        match other {
            Advanced::Rock => return Effectiveness::Half(value),
            Advanced::Plant => return Effectiveness::HalfExtra(value),
            Advanced::Water => return Effectiveness::Double(value),
            Advanced::Light => return Effectiveness::None(value),
            Advanced::Wind => return Effectiveness::Half(value),
            _=> return Effectiveness::Normal(value),
        }
    }
    /// Spirit Effectiveness against a target
    fn spirit(other:Advanced, value:T) -> Effectiveness<T> {
        match other {
            Advanced::Water => return Effectiveness::None(value),
            Advanced::Fire => return Effectiveness::Double(value),
            Advanced::Electric => return Effectiveness::Half(value),
            Advanced::Spirit => return Effectiveness::HalfExtra(value),
            Advanced::Light => return Effectiveness::Half(value),
            Advanced::Wind => return Effectiveness::Double(value),
            Advanced::None => return Effectiveness::None(value),
            _=> return Effectiveness::Normal(value),
        }
    }
    /// Light Effectiveness against a target
    fn light(other:Advanced, value:T) -> Effectiveness<T> {
        match other {
            Advanced::Rock => return Effectiveness::Double(value),
            Advanced::Plant => return Effectiveness::None(value),
            Advanced::Water => return Effectiveness::Double(value),
            Advanced::Fire => return Effectiveness::None(value),
            Advanced::Electric => return Effectiveness::Half(value),
            Advanced::Wind => return Effectiveness::HalfExtra(value),
            Advanced::None => return Effectiveness::Half(value),
            _=> return Effectiveness::Normal(value),
        }
    }
    ///  Effectiveness against a target
    fn wind(other:Advanced, value:T) -> Effectiveness<T> {
        match other {
            Advanced::Rock => return Effectiveness::Double(value),
            Advanced::Plant => return Effectiveness::Half(value),
            Advanced::Water => return Effectiveness::Double(value),
            Advanced::Fire => return Effectiveness::None(value),
            Advanced::Spirit => return Effectiveness::Half(value),
            Advanced::Wind => return Effectiveness::HalfExtra(value),
            _=> return Effectiveness::Normal(value),
        }
    }
    ///  Effectiveness against a target
    fn none(other:Advanced, value:T) -> Effectiveness<T> {
        match other {
            Advanced::Water => return Effectiveness::Half(value),
            Advanced::Fire => return Effectiveness::Half(value),
            Advanced::Electric => return Effectiveness::Half(value),
            Advanced::Spirit => return Effectiveness::None(value),
            Advanced::Light => return Effectiveness::None(value),
            Advanced::Wind => return Effectiveness::Half(value),
            _=> return Effectiveness::Normal(value),
        }
    }
    
    /// Match current Type to find effectiveness of the value
    fn effectiveness(&self, other:Advanced, value:T) -> Effectiveness<T> {
        match *self {
            Advanced::Rock => return Advanced::rock(other, value),
            Advanced::Plant => return Advanced::plant(other, value),
            Advanced::Water => return Advanced::water(other, value),
            Advanced::Fire => return Advanced::fire(other, value),
            Advanced::Electric => return Advanced::electric(other, value),
            Advanced::Spirit => return Advanced::spirit(other, value),
            Advanced::Light => return Advanced::light(other, value),
            Advanced::Wind => return Advanced::wind(other, value),
            Advanced::None => return Advanced::none(other, value),
        }
    }
}
