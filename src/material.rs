/*!# Materials 


*/
use std::fmt;
use std::fmt::Debug;
use std::path::Path;
//use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign};
extern crate num;
use serde::{Deserialize, Serialize};

#[cfg(feature = "fltkform")]
use fltk::{prelude::*, *};
#[cfg(feature = "fltkform")]
use fltk_form_derive::*;
#[cfg(feature = "fltkform")]
use fltk_form::FltkForm;

use crate::random::Random;
use crate::stats::Stats;
use crate::stats::Premade;

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
/*# Drop File
Read in a file with this format:
```toml
name = "Claw"
image  = "assets/drops/Claw.svg"
makes = ["assets/refinery/Necklace.toml", "assets/refinery/EarRing.toml"]
[stats]
id = 8
hp = 1000
mp = 90
xp = 10
hp_max = 1000
mp_max = 90
xp_next = 10
gp = 8
speed = 10
level = 1
atk = 90
def = 100
m_atk = 90
m_def = 90
agility = 10
strength = 90
intelligence = 10
dexterity = 10
constitution = 90
charisma = 10
wisdom = 10
age = 10
```
using:
```rs:no_run
let drop = DropFile::read(filename);
```

*/
pub struct DropFile {
    pub name:String,
    pub makes:Vec<String>,
    pub image:String,
    pub stats:Stats,
}
impl DropFile {
    pub fn read<P: AsRef<Path>>(filename:P) -> Self {
        if let Ok(file_string) = std::fs::read_to_string(filename) {
            let decoded:DropFile = match toml::from_str(file_string.as_str()) {
                Ok(decoded) => decoded,
                Err(e) => {
                    println!("DropFile::read()->toml::from_str() Error:{}",e);
                    return Self::default()
                },
            };
            return decoded;
        }
        Self::default()
    }
    pub fn sell(&self) -> f64 {
        self.gp()
    }
}
impl Premade for DropFile {
    fn stat(&self) -> Stats {
        self.stats
    }
    fn set_hp(&mut self, amount:f64) {
        self.stats.hp = amount;
    }
    fn set_mp(&mut self, amount:f64) {
        self.stats.mp = amount;
    }
    fn set_xp(&mut self, amount:f64) {
        self.stats.xp = amount;
    }
    fn set_hp_max(&mut self, amount:f64) {
        self.stats.hp_max = amount;
    }
    fn set_mp_max(&mut self, amount:f64) {
        self.stats.mp_max = amount;
    }
    fn set_xp_next(&mut self, amount:f64) {
        self.stats.xp_next = amount;
    }
    fn set_gp(&mut self, amount:f64) {
        self.stats.gp = amount;
    }
    fn set_atk(&mut self, amount:f64) {
        self.stats.atk = amount;
    }
    fn set_def(&mut self, amount:f64) {
        self.stats.def = amount;
    }
    fn set_m_atk(&mut self, amount:f64) {
        self.stats.m_atk = amount;
    }
    fn set_m_def(&mut self, amount:f64) {
        self.stats.m_def = amount;
    }
    fn set_level(&mut self, amount:f64) {
        self.stats.level = amount;
    }
    fn set_speed(&mut self, amount:f64) {
        self.stats.speed = amount;
    }
}

/*# Drop Inventory

*/
#[derive(Debug, Default, Clone, PartialEq)]
pub struct DropInventory {
    pub items:Vec<DropFile>,
    pub count:Vec<f64>,
}
//
#[macro_export]
macro_rules! inventory {
    { $($t:ty), * } => { pub struct DropInventory {
$(    pub $t:DropFile,
    pub $t_count:f64)*
    } };
}

/*# Drop List

*/
#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct DropList {
    pub items:Vec<String>,
}
impl DropList {
    pub fn get_item(&self, item:usize) -> Option<DropFile> {
        if item >= self.items.len() {
            return None;
        }
        Some(DropFile::read(self.items[item].clone()))
    }
    pub fn get_all(&self) -> Vec<DropFile> {
        let mut return_vec:Vec<DropFile> = vec![];
        for i in self.items.clone() {
            let c = DropFile::read(i.clone());
            if c != DropFile::default() {
                return_vec.push(c);
            }
        }
        return_vec
    }
    pub fn read<P: AsRef<Path>>(filename:P) -> Self {
        if let Ok(file_string) = std::fs::read_to_string(filename) {
            let decoded:DropList = match toml::from_str(file_string.as_str()) {
                Ok(decoded) => decoded,
                Err(e) => {
                    println!("DropList::read()->toml::from_str() Error:{}",e);
                    return Self::default()
                },
            };
            return decoded;
        }
        Self::default()
    }
}
/*
# Armor 

Basic Armor materials

*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum Armor {
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
impl Default for Armor {
    fn default() -> Self {
        Self::None
    }
}
impl fmt::Display for Armor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let v:String;
        match *self {
            //Armor:: => v = String::from(""),
            _=> v = String::from("None"),
        }
        write!(f, "{}", v.as_str())
    }
}

/*
# Advanced 

*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[cfg_attr(feature = "fltkform", derive(FltkForm))]
/*  
# Drops

*/
pub enum Drops {
    /// 
    Feed,
    /// 
    Hide,
    /// 
    Scale,
    /// 
    Tooth,
    /// 
    Horn,
    /// 
    Talon,
    /// 
    Feather,
    /// 
    Claw,
    /// 
    Fang,
    /// 
    Bone,
    /// 
    Hair,
    /// 
    Jerky,
    /// 
    Oil,
    /// 
    Wool,
    /// 
    Pelt,
    /// 
    Leather,
    /// 
    Fur,
    /// 
    Tusk,
    /// Nothing
    None,
}
impl Default for Drops {
    fn default() -> Self {
        Self::None
    }
}
impl fmt::Display for Drops {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let v:String;
        match *self {
            Drops::Feed => v = String::from("Feed"),
            Drops::Hide => v = String::from("Hide"),
            Drops::Scale => v = String::from("Scale"),
            Drops::Tooth => v = String::from("Tooth"),
            Drops::Horn => v = String::from("Horn"),
            Drops::Talon => v = String::from("Talon"),
            Drops::Feather => v = String::from("Feather"),
            Drops::Claw => v = String::from("Claw"),
            Drops::Fang => v = String::from("Fang"),
            Drops::Bone => v = String::from("Bone"),
            Drops::Hair => v = String::from("Hair"),
            Drops::Jerky => v = String::from("Jerky"),
            Drops::Oil => v = String::from("Oil"),
            Drops::Wool => v = String::from("Wool"),
            Drops::Pelt => v = String::from("Pelt"),
            Drops::Leather => v = String::from("Leather"),
            Drops::Fur => v = String::from("Fur"),
            Drops::Tusk => v = String::from("Tusk"),
            _=> v = String::from("None"),
        }
        write!(f, "{}", v.as_str())
    }
}
impl Random for Drops {
    type Type = Drops;
    fn random_type(&self) -> Self::Type {
        let max = 18;
        let val = self.random_rate(max);
        match val {
            0 => Drops::Feed,
            1 => Drops::Hide,
            2 => Drops::Scale,
            3 => Drops::Tooth,
            4 => Drops::Horn,
            5 => Drops::Talon,
            6 => Drops::Feather,
            7 => Drops::Claw,
            8 => Drops::Fang,
            9 => Drops::Bone,
            10 => Drops::Hair,
            11 => Drops::Jerky,
            12 => Drops::Oil,
            13 => Drops::Wool,
            14 => Drops::Pelt,
            15 => Drops::Leather,
            16 => Drops::Fur,
            17 => Drops::Tusk,
            _=> Drops::None,
        }
    }
    
}
impl Drops {
    ///
    pub fn list() -> Vec<Drops> {
        vec![Drops::Feed, Drops::Hide, Drops::Scale, Drops::Tooth, Drops::Horn, Drops::Talon, Drops::Feather, Drops::Claw, Drops::Fang, Drops::Bone, Drops::Hair, Drops::Jerky, Drops::Oil, Drops::Wool, Drops::Pelt, Drops::Leather, Drops::Fur, Drops::Tusk]
    }
    /// 
    pub fn get_price(drop:Drops) -> f64 {
        drop.price()
    }
    pub fn price(&self) -> f64 {
        match *self {
           Drops::Feed =>  1.0,
           Drops::Hide =>  2.0,
           Drops::Scale =>  3.0,
           Drops::Tooth =>  4.0,
           Drops::Horn =>  5.0,
           Drops::Talon =>  6.0,
           Drops::Feather =>  7.0,
           Drops::Claw =>  8.0,
           Drops::Fang =>  9.0,
           Drops::Bone =>  10.0,
           Drops::Hair =>  11.0,
           Drops::Jerky =>  12.0,
           Drops::Oil =>  13.0,
           Drops::Wool =>  14.0,
           Drops::Pelt =>  15.0,
           Drops::Leather =>  16.0,
           Drops::Fur =>  17.0,
           Drops::Tusk =>  18.0,
           _=> 0.0,
        }
    }
}
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[cfg_attr(feature = "fltkform", derive(FltkForm))]
/*

*/
pub struct DropBag {
    pub feed:f64,
    pub hide:f64,
    pub scale:f64,
    pub tooth:f64,
    pub horn:f64,
    pub talon:f64,
    pub feather:f64,
    pub claw:f64,
    pub fang:f64,
    pub bone:f64,
    pub hair:f64,
    pub jerky:f64,
    pub oil:f64,
    pub wool:f64,
    pub pelt:f64,
    pub leather:f64,
    pub fur:f64,
    pub tusk:f64,
}
impl DropBag {
    pub fn new() -> Self {
            DropBag {
            feed:0.0,
            hide:0.0,
            scale:0.0,
            tooth:0.0,
            horn:0.0,
            talon:0.0,
            feather:0.0,
            claw:0.0,
            fang:0.0,
            bone:0.0,
            hair:0.0,
            jerky:0.0,
            oil:0.0,
            wool:0.0,
            pelt:0.0,
            leather:0.0,
            fur:0.0,
            tusk:0.0,
        }
    }
/*
# Sell

Sell an item from your bag
*/
    pub fn sell(&mut self, drop:Drops) -> f64 {
        let mut price:f64 = 0.0;
        match drop {
            Drops::Feed =>  {
                if self.feed >= 1.0 {
                    self.feed -= 1.0;
                    price = drop.price();
                }
            },
            Drops::Hide =>  {
                if self.hide >= 1.0 {
                    self.hide -= 1.0;
                    price = drop.price();
                }
            },
            Drops::Scale =>  {
                if self.scale >= 1.0 {
                    self.scale -= 1.0;
                    price = drop.price();
                }
            },
            Drops::Tooth =>  {
                if self.tooth >= 1.0 {
                    self.tooth -= 1.0;
                    price = drop.price();
                }
            },
            Drops::Horn =>  {
                if self.horn >= 1.0 {
                    self.horn -= 1.0;
                    price = drop.price();
                }
            },
            Drops::Talon =>  {
                if self.talon >= 1.0 {
                    self.talon -= 1.0;
                    price = drop.price();
                }
            },
            Drops::Feather =>  {
                if self.feather >= 1.0 {
                    self.feather -= 1.0;
                    price = drop.price();
                }
            },
            Drops::Claw =>  {
                if self.claw >= 1.0 {
                    self.claw -= 1.0;
                    price = drop.price();
                }
            },
            Drops::Fang =>  {
                if self.fang >= 1.0 {
                    self.fang -= 1.0;
                    price = drop.price();
                }
            },
            Drops::Bone =>  {
                if self.bone >= 1.0 {
                    self.bone -= 1.0;
                    price = drop.price();
                }
            },
            Drops::Hair =>  {
                if self.hair >= 1.0 {
                    self.hair -= 1.0;
                    price = drop.price();
                }
            },
            Drops::Jerky =>  {
                if self.jerky >= 1.0 {
                    self.jerky -= 1.0;
                    price = drop.price();
                }
            },
            Drops::Oil =>  {
                if self.oil >= 1.0 {
                    self.oil -= 1.0;
                    price = drop.price();
                }
            },
            Drops::Wool =>  {
                if self.wool >= 1.0 {
                    self.wool -= 1.0;
                    price = drop.price();
                }
            },
            Drops::Pelt =>  {
                if self.pelt >= 1.0 {
                    self.pelt -= 1.0;
                    price = drop.price();
                }
            },
            Drops::Leather =>  {
                if self.leather >= 1.0 {
                    self.leather -= 1.0;
                    price = drop.price();
                }
            },
            Drops::Fur =>  {
                if self.fur >= 1.0 {
                    self.fur -= 1.0;
                    price = drop.price();
                }
            },
            Drops::Tusk =>  {
                if self.tusk >= 1.0 {
                    self.tusk -= 1.0;
                    price = drop.price();
                }
            },
            _=> (),
        }
        price
    }
/*
# Get

Get an item into your bag
*/
    pub fn get(&mut self, drop:Drops) {
        match drop {
            Drops::Feed =>  self.feed += 1.0,
            Drops::Hide =>  self.hide += 1.0,
            Drops::Scale =>  self.scale += 1.0,
            Drops::Tooth =>  self.tooth += 1.0,
            Drops::Horn =>  self.horn += 1.0,
            Drops::Talon =>  self.talon += 1.0,
            Drops::Feather =>  self.feather += 1.0,
            Drops::Claw =>  self.claw += 1.0,
            Drops::Fang =>  self.fang += 1.0,
            Drops::Bone =>  self.bone += 1.0,
            Drops::Hair =>  self.hair += 1.0,
            Drops::Jerky =>  self.jerky += 1.0,
            Drops::Oil =>  self.oil += 1.0,
            Drops::Wool =>  self.wool += 1.0,
            Drops::Pelt =>  self.pelt += 1.0,
            Drops::Leather =>  self.leather += 1.0,
            Drops::Fur =>  self.fur += 1.0,
            Drops::Tusk =>  self.tusk += 1.0,
            _=> (),
        }
    }
}
impl Default for DropBag {
    fn default() -> Self {
        Self::new()
    }
}
