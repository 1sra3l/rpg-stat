/*!
# Armor

Use stats::Builder to get some basic premade armor stats

```
use rpg_stat::armor::Basic as Armor;
use rpg_stat::stats::Basic as Stats;
use rpg_stat::stats::Builder;
let armor = Armor::Good;
let stats:Stats<f64> = armor.build_basic(0.0, 1.0);
assert_eq!(stats.hp, 5.0);
```
These enums can be combined with other modules, such as Element to create an elemental armor

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
use crate::stats::Basic as BasicStats;
use crate::stats::Normal as NormalStats;
use crate::stats::Advanced as AdvancedStats;
use crate::stats::Builder;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[cfg_attr(feature = "fltkform", derive(FltkForm))]
/* 
# Basic Armor


*/
pub enum Basic {
    /// Good Armor
    Good,
    /// Better Armor
    Better,
    /// Best Armor
    Best,
    /// Epic Armor
    Epic,
    /// Legendary Armor
    Legendary,
    /// No armor
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
            Basic::Good => v = String::from("Good"),
            Basic::Better => v = String::from("Better"),
            Basic::Best => v = String::from("Best"),
            Basic::Epic => v = String::from("Epic"),
            Basic::Legendary => v = String::from("Legendary"),
            _=> v = String::from("None"),
        }
        write!(f, "{}", v.as_str())
    }
}
impl Random for Basic {
    type Type = Basic;
    fn random_type(&self) -> Self::Type {
        let max = 5;
        let val = self.random_rate(max);
        match val {
            0 => Basic::Good,
            1 => Basic::Better,
            2 => Basic::Best,
            3 => Basic::Epic,
            4 => Basic::Legendary,
            _=> Basic::None,
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
    + num::NumCast> Builder<T> for Basic {
    fn build_basic(&self, id:T, level:T) -> BasicStats<T>{
        let mut hp:T = num::cast(0).unwrap();
        let mut mp:T = num::cast(0).unwrap();
        let mut xp:T = num::cast(0).unwrap();
        let xp_next:T = num::cast(0).unwrap();
        let mut gp:T = num::cast(0).unwrap();
        let mut speed:T = num::cast(0).unwrap();
        match *self {
            Basic::Good => {
                xp = num::cast(1).unwrap();
                hp = num::cast(5).unwrap();
                mp = num::cast(5).unwrap();
                speed = num::cast(1).unwrap();
            },
            Basic::Better => {
                xp = num::cast(7).unwrap();
                hp = num::cast(35).unwrap();
                mp = num::cast(35).unwrap();
                speed = num::cast(2).unwrap();
            },
            Basic::Best => {
                xp = num::cast(14).unwrap();
                hp = num::cast(70).unwrap();
                mp = num::cast(70).unwrap();
                speed = num::cast(4).unwrap();
            },
            Basic::Epic => {
                xp = num::cast(28).unwrap();
                hp = num::cast(105).unwrap();
                mp = num::cast(105).unwrap();
                speed = num::cast(7).unwrap();
            },
            Basic::Legendary => {
                xp = num::cast(50).unwrap();
                hp = num::cast(200).unwrap();
                mp = num::cast(200).unwrap();
                speed = num::cast(10).unwrap();
            },
            _=> (),
        }
        let mut stats = BasicStats {
            id,
            xp,
            xp_next,
            level,
            gp,
            hp,
            mp,
            hp_max:hp,
            mp_max:mp,
            speed,
        };
        stats.level_up();
        stats
        
    }
    fn build_normal(&self, id:T, level:T) -> NormalStats<T>{
        let mut hp:T = num::cast(0).unwrap();
        let mp:T = num::cast(0).unwrap();
        let mut xp:T = num::cast(0).unwrap();
        let xp_next:T = num::cast(0).unwrap();
        let mut gp:T = num::cast(0).unwrap();
        let mut speed:T = num::cast(0).unwrap();
        let mut atk:T = num::cast(0).unwrap();
        let mut def:T = num::cast(0).unwrap();
        let mut m_atk:T = num::cast(0).unwrap();
        let mut m_def:T = num::cast(0).unwrap();
        match *self {
            Basic::Good => {
                xp = num::cast(4).unwrap();
                hp = num::cast(4).unwrap();
                def = num::cast(10).unwrap();
                m_def = num::cast(10).unwrap();
            },
            Basic::Better => {
                xp = num::cast(9).unwrap();
                hp = num::cast(7).unwrap();
                speed = num::cast(1).unwrap();
                atk = num::cast(5).unwrap();
                def = num::cast(50).unwrap();
                m_def = num::cast(50).unwrap();
            },
            Basic::Best => {
                xp = num::cast(16).unwrap();
                hp = num::cast(12).unwrap();
                speed = num::cast(2).unwrap();
                atk = num::cast(15).unwrap();
                def = num::cast(100).unwrap();
                m_def = num::cast(100).unwrap();
            },
            Basic::Epic => {
                xp = num::cast(27).unwrap();
                hp = num::cast(50).unwrap();
                speed = num::cast(4).unwrap();
                atk = num::cast(70).unwrap();
                def = num::cast(200).unwrap();
                m_atk = num::cast(25).unwrap();
                m_def = num::cast(300).unwrap();
            },
            Basic::Legendary => {
                xp = num::cast(51).unwrap();
                hp = num::cast(115).unwrap();
                speed = num::cast(10).unwrap();
                atk = num::cast(150).unwrap();
                def = num::cast(350).unwrap();
                m_atk = num::cast(100).unwrap();
                m_def = num::cast(450).unwrap();
            },
            _=> (),
        }
        let mut stats = NormalStats {
            id,
            xp,
            xp_next,
            level,
            gp,
            hp,
            mp,
            hp_max:hp,
            mp_max:mp,
            speed,
            atk,
            def,
            m_atk,
            m_def,
        };
        stats.level_up();
        stats
    }
    fn build_advanced(&self, id:T, level:T) -> AdvancedStats<T>{
        let mut hp:T = num::cast(0).unwrap();
        let mut mp:T = num::cast(0).unwrap();
        let mut xp:T = num::cast(0).unwrap();
        let xp_next:T = num::cast(0).unwrap();
        let gp:T = num::cast(0).unwrap();
        let mut speed:T = num::cast(0).unwrap();
        let mut atk:T = num::cast(0).unwrap();
        let mut def:T = num::cast(0).unwrap();
        let mut m_atk:T = num::cast(0).unwrap();
        let mut m_def:T = num::cast(0).unwrap();
        let mut agility:T = num::cast(0).unwrap();
        let mut strength:T = num::cast(0).unwrap();
        let mut dexterity:T = num::cast(0).unwrap();
        let mut constitution:T = num::cast(0).unwrap();
        let intelligence:T = num::cast(0).unwrap();
        let mut charisma:T = num::cast(0).unwrap();
        let wisdom:T = num::cast(0).unwrap();
        let age:T = num::cast(0).unwrap();
        match *self {
            Basic::Good => {
                xp = num::cast(4).unwrap();
                hp = num::cast(4).unwrap();
                def = num::cast(10).unwrap();
                m_def = num::cast(10).unwrap();
                strength = num::cast(5).unwrap();
                constitution = num::cast(5).unwrap();
                charisma = num::cast(2).unwrap();
            },
            Basic::Better => {
                xp = num::cast(9).unwrap();
                hp = num::cast(7).unwrap();
                speed = num::cast(1).unwrap();
                atk = num::cast(5).unwrap();
                def = num::cast(50).unwrap();
                m_def = num::cast(50).unwrap();
                strength = num::cast(10).unwrap();
                constitution = num::cast(12).unwrap();
                charisma = num::cast(7).unwrap();
            },
            Basic::Best => {
                xp = num::cast(16).unwrap();
                hp = num::cast(12).unwrap();
                speed = num::cast(2).unwrap();
                atk = num::cast(15).unwrap();
                def = num::cast(100).unwrap();
                m_def = num::cast(100).unwrap();
                strength = num::cast(25).unwrap();
                constitution = num::cast(20).unwrap();
                charisma = num::cast(12).unwrap();
            },
            Basic::Epic => {
                xp = num::cast(27).unwrap();
                hp = num::cast(50).unwrap();
                speed = num::cast(4).unwrap();
                atk = num::cast(70).unwrap();
                def = num::cast(200).unwrap();
                m_atk = num::cast(25).unwrap();
                m_def = num::cast(300).unwrap();
                strength = num::cast(40).unwrap();
                constitution = num::cast(30).unwrap();
                charisma = num::cast(25).unwrap();
            },
            Basic::Legendary => {
                xp = num::cast(51).unwrap();
                hp = num::cast(115).unwrap();
                speed = num::cast(10).unwrap();
                atk = num::cast(150).unwrap();
                def = num::cast(350).unwrap();
                m_atk = num::cast(100).unwrap();
                m_def = num::cast(450).unwrap();
                agility = num::cast(10).unwrap();
                strength = num::cast(100).unwrap();
                constitution = num::cast(70).unwrap();
                charisma = num::cast(52).unwrap();
                dexterity = num::cast(10).unwrap();
            },
            _=> (),
        }
        let mut stats = AdvancedStats {
            id,
            xp,
            xp_next,
            level,
            gp,
            hp,
            mp,
            hp_max:hp,
            mp_max:mp,
            speed,
            atk,
            def,
            m_atk,
            m_def,
            agility,
            strength,
            dexterity,
            constitution,
            intelligence,
            charisma,
            wisdom,
            age,
        };
        stats.level_up();
        stats
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[cfg_attr(feature = "fltkform", derive(FltkForm))]
/* 
# Normal

*/
pub enum Normal {
    /// Hood
    Hood,
    /// Jaw
    Jaw,
    /// Joint
    Joint,
    /// Collar
    Collar,
    /// UpperArm
    UpperArm,
    /// Elbow
    Elbow,
    /// Pants
    Pants,
    /// Belly
    Belly,
    /// Chestplate
    Chestplate,
    /// Torso
    Torso,
    /// Hip
    Hip,
    /// Knee
    Knee,
    /// Shin
    Shin,
    /// Shoe
    Shoe,
    /// Shoulder
    Shoulder,
    /// Forearm
    Forearm,
    /// Hand
    Hand,
    /// Shirt
    Shirt,
    /// Head
    Head,
    /// Neck
    Neck,
    /// Face
    Face,
    /// Coat
    Coat,
    /// Thigh
    Thigh,
    /// No armor
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
            Normal::Hood => v = String::from("Hood"),
            Normal::Jaw => v = String::from("Jaw"),
            Normal::Joint => v = String::from("Joint"),
            Normal::Collar => v = String::from("Collar"),
            Normal::UpperArm => v = String::from("UpperArm"),
            Normal::Elbow => v = String::from("Elbow"),
            Normal::Pants => v = String::from("Pants"),
            Normal::Belly => v = String::from("Belly"),
            Normal::Chestplate => v = String::from("Chestplate"),
            Normal::Torso => v = String::from("Torso"),
            Normal::Hip => v = String::from("Hip"),
            Normal::Knee => v = String::from("Knee"),
            Normal::Shin => v = String::from("Shin"),
            Normal::Shoe => v = String::from("Shoe"),
            Normal::Shoulder => v = String::from("Shoulder"),
            Normal::Forearm => v = String::from("Forearm"),
            Normal::Hand => v = String::from("Hand"),
            Normal::Shirt => v = String::from("Shirt"),
            Normal::Head => v = String::from("Head"),
            Normal::Neck => v = String::from("Neck"),
            Normal::Face => v = String::from("Face"),
            Normal::Coat => v = String::from("Coat"),
            Normal::Thigh => v = String::from("Thigh"),
            _=> v = String::from("None"),
        }
        write!(f, "{}", v.as_str())
    }
}
impl Random for Normal {
    type Type = Normal;
    fn random_type(&self) -> Self::Type {
        let max = 23;
        let val = self.random_rate(max);
        match val {
            0 => Normal::Hood,
            1 => Normal::Jaw,
            2 => Normal::Joint,
            3 => Normal::Collar,
            4 => Normal::UpperArm,
            5 => Normal::Elbow,
            6 => Normal::Pants,
            7 => Normal::Belly,
            8 => Normal::Chestplate,
            9 => Normal::Torso,
            10 => Normal::Hip,
            11 => Normal::Knee,
            12 => Normal::Shin,
            13 => Normal::Shoe,
            14 => Normal::Shoulder,
            15 => Normal::Forearm,
            16 => Normal::Hand,
            17 => Normal::Shirt,
            18 => Normal::Head,
            19 => Normal::Neck,
            20 => Normal::Face,
            21 => Normal::Coat,
            22 => Normal::Thigh,
            _=> Normal::None,
        }
    }
    
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[cfg_attr(feature = "fltkform", derive(FltkForm))]
/* 
# Advanced

*/
pub enum Advanced {
   /// Hood armor
   Hood,
   /// Jaw / throat armor
   Bevor,
   /// Circular plate armor protecting various areas
   Rondel,
   /// Collar armor
   Gorget,
   /// Upper arm armor below the shoulder armor
   Rerebrace,
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
   Tasset,
   /// No armor
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
            Advanced::Hood => v = String::from("Hood"),
            Advanced::Bevor => v = String::from("Bevor"),
            Advanced::Rondel => v = String::from("Rondel"),
            Advanced::Gorget => v = String::from("Gorget"),
            Advanced::Rerebrace => v = String::from("Rerebrace"),
            Advanced::Couter => v = String::from("Couter"),
            Advanced::Chausses => v = String::from("Chausses"),
            Advanced::Plackart => v = String::from("Plackart"),
            Advanced::Cuisses => v = String::from("Cuisses"),
            Advanced::Chestplate => v = String::from("Chestplate"),
            Advanced::Curiass => v = String::from("Curiass"),
            Advanced::Fauld => v = String::from("Fauld"),
            Advanced::Poleyn => v = String::from("Poleyn"),
            Advanced::Greaves => v = String::from("Greaves"),
            Advanced::Sabaton => v = String::from("Sabaton"),
            Advanced::Spaulders => v = String::from("Spaulders"),
            Advanced::Pauldron => v = String::from("Pauldron"),
            Advanced::Vambrace => v = String::from("Vambrace"),
            Advanced::Gauntlets => v = String::from("Gauntlets"),
            Advanced::Hauberk => v = String::from("Hauberk"),
            Advanced::Helmet => v = String::from("Helmet"),
            Advanced::Neckguard => v = String::from("Neckguard"),
            Advanced::Faceplate => v = String::from("Faceplate"),
            Advanced::Coat => v = String::from("Coat"),
            Advanced::Tasset => v = String::from("Tasset"),
            _=> v = String::from("None"),
        }
        write!(f, "{}", v.as_str())
    }
}
impl Random for Advanced {
    type Type = Advanced;
    fn random_type(&self) -> Self::Type {
        let max = 25;
        let val = self.random_rate(max);
        match val {
            0 => Advanced::Hood,
            1 => Advanced::Bevor,
            2 => Advanced::Rondel,
            3 => Advanced::Gorget,
            4 => Advanced::Rerebrace,
            5 => Advanced::Couter,
            6 => Advanced::Chausses,
            7 => Advanced::Plackart,
            8 => Advanced::Cuisses,
            9 => Advanced::Chestplate,
            10 => Advanced::Curiass,
            11 => Advanced::Fauld,
            12 => Advanced::Poleyn,
            13 => Advanced::Greaves,
            14 => Advanced::Sabaton,
            15 => Advanced::Spaulders,
            16 => Advanced::Pauldron,
            17 => Advanced::Vambrace,
            18 => Advanced::Gauntlets,
            19 => Advanced::Hauberk,
            20 => Advanced::Helmet,
            21 => Advanced::Neckguard,
            22 => Advanced::Faceplate,
            23 => Advanced::Coat,
            24 => Advanced::Tasset,
            _=> Advanced::None,
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
    + num::NumCast> Builder<T> for Advanced {
    fn build_basic(&self, id:T, level:T) -> BasicStats<T>{
        let mut hp:T = num::cast(0).unwrap();
        let mut mp:T = num::cast(0).unwrap();
        let mut xp:T = num::cast(0).unwrap();
        let xp_next:T = num::cast(0).unwrap();
        let mut gp:T = num::cast(0).unwrap();
        let mut speed:T = num::cast(0).unwrap();
        match *self {
            //TODO
            _=> {
                xp = num::cast(50).unwrap();
                hp = num::cast(200).unwrap();
                mp = num::cast(200).unwrap();
                speed = num::cast(10).unwrap();
            },
        }
        let mut stats = BasicStats {
            id,
            xp,
            xp_next,
            level,
            gp,
            hp,
            mp,
            hp_max:hp,
            mp_max:mp,
            speed,
        };
        stats.level_up();
        stats
        
    }
    fn build_normal(&self, id:T, level:T) -> NormalStats<T>{
        let mut hp:T = num::cast(0).unwrap();
        let mp:T = num::cast(0).unwrap();
        let mut xp:T = num::cast(0).unwrap();
        let xp_next:T = num::cast(0).unwrap();
        let mut gp:T = num::cast(0).unwrap();
        let mut speed:T = num::cast(0).unwrap();
        let mut atk:T = num::cast(0).unwrap();
        let mut def:T = num::cast(0).unwrap();
        let mut m_atk:T = num::cast(0).unwrap();
        let mut m_def:T = num::cast(0).unwrap();
        match *self {
            //TODO
            _=> {
                xp = num::cast(51).unwrap();
                hp = num::cast(115).unwrap();
                speed = num::cast(10).unwrap();
                atk = num::cast(150).unwrap();
                def = num::cast(350).unwrap();
                m_atk = num::cast(100).unwrap();
                m_def = num::cast(450).unwrap();
            },
        }
        let mut stats = NormalStats {
            id,
            xp,
            xp_next,
            level,
            gp,
            hp,
            mp,
            hp_max:hp,
            mp_max:mp,
            speed,
            atk,
            def,
            m_atk,
            m_def,
        };
        stats.level_up();
        stats
    }
    fn build_advanced(&self, id:T, level:T) -> AdvancedStats<T>{
        let mut hp:T = num::cast(0).unwrap();
        let mut mp:T = num::cast(0).unwrap();
        let mut xp:T = num::cast(0).unwrap();
        let xp_next:T = num::cast(0).unwrap();
        let gp:T = num::cast(0).unwrap();
        let mut speed:T = num::cast(0).unwrap();
        let mut atk:T = num::cast(0).unwrap();
        let mut def:T = num::cast(0).unwrap();
        let mut m_atk:T = num::cast(0).unwrap();
        let mut m_def:T = num::cast(0).unwrap();
        let mut agility:T = num::cast(0).unwrap();
        let mut strength:T = num::cast(0).unwrap();
        let mut dexterity:T = num::cast(0).unwrap();
        let mut constitution:T = num::cast(0).unwrap();
        let intelligence:T = num::cast(0).unwrap();
        let mut charisma:T = num::cast(0).unwrap();
        let wisdom:T = num::cast(0).unwrap();
        let age:T = num::cast(0).unwrap();
        match *self {
            //TODO
            _=> {
                xp = num::cast(51).unwrap();
                hp = num::cast(115).unwrap();
                speed = num::cast(10).unwrap();
                atk = num::cast(150).unwrap();
                def = num::cast(350).unwrap();
                m_atk = num::cast(100).unwrap();
                m_def = num::cast(450).unwrap();
                agility = num::cast(10).unwrap();
                strength = num::cast(100).unwrap();
                constitution = num::cast(70).unwrap();
                charisma = num::cast(52).unwrap();
                dexterity = num::cast(10).unwrap();
            },
        }
        let mut stats = AdvancedStats {
            id,
            xp,
            xp_next,
            level,
            gp,
            hp,
            mp,
            hp_max:hp,
            mp_max:mp,
            speed,
            atk,
            def,
            m_atk,
            m_def,
            agility,
            strength,
            dexterity,
            constitution,
            intelligence,
            charisma,
            wisdom,
            age,
        };
        stats.level_up();
        stats
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[cfg_attr(feature = "fltkform", derive(FltkForm))]
/* 
# Armor

*/
pub enum Armor {
   /// Hood armor
   Hood,
   /// Jaw / throat armor
   Bevor,
   /// Circular plate armor protecting various areas
   Rondel,
   /// Collar armor
   Gorget,
   /// Upper arm armor below the shoulder armor
   Rerebrace,
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
   Tasset,
   /// No armor
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
            Armor::Hood => v = String::from("Hood"),
            Armor::Bevor => v = String::from("Bevor"),
            Armor::Rondel => v = String::from("Rondel"),
            Armor::Gorget => v = String::from("Gorget"),
            Armor::Rerebrace => v = String::from("Rerebrace"),
            Armor::Couter => v = String::from("Couter"),
            Armor::Chausses => v = String::from("Chausses"),
            Armor::Plackart => v = String::from("Plackart"),
            Armor::Cuisses => v = String::from("Cuisses"),
            Armor::Chestplate => v = String::from("Chestplate"),
            Armor::Curiass => v = String::from("Curiass"),
            Armor::Fauld => v = String::from("Fauld"),
            Armor::Poleyn => v = String::from("Poleyn"),
            Armor::Greaves => v = String::from("Greaves"),
            Armor::Sabaton => v = String::from("Sabaton"),
            Armor::Spaulders => v = String::from("Spaulders"),
            Armor::Pauldron => v = String::from("Pauldron"),
            Armor::Vambrace => v = String::from("Vambrace"),
            Armor::Gauntlets => v = String::from("Gauntlets"),
            Armor::Hauberk => v = String::from("Hauberk"),
            Armor::Helmet => v = String::from("Helmet"),
            Armor::Neckguard => v = String::from("Neckguard"),
            Armor::Faceplate => v = String::from("Faceplate"),
            Armor::Coat => v = String::from("Coat"),
            Armor::Tasset => v = String::from("Tasset"),
            _=> v = String::from("None"),
        }
        write!(f, "{}", v.as_str())
    }
}
impl Random for Armor {
    type Type = Armor;
    fn random_type(&self) -> Self::Type {
        let max = 25;
        let val = self.random_rate(max);
        match val {
            0 => Armor::Hood,
            1 => Armor::Bevor,
            2 => Armor::Rondel,
            3 => Armor::Gorget,
            4 => Armor::Rerebrace,
            5 => Armor::Couter,
            6 => Armor::Chausses,
            7 => Armor::Plackart,
            8 => Armor::Cuisses,
            9 => Armor::Chestplate,
            10 => Armor::Curiass,
            11 => Armor::Fauld,
            12 => Armor::Poleyn,
            13 => Armor::Greaves,
            14 => Armor::Sabaton,
            15 => Armor::Spaulders,
            16 => Armor::Pauldron,
            17 => Armor::Vambrace,
            18 => Armor::Gauntlets,
            19 => Armor::Hauberk,
            20 => Armor::Helmet,
            21 => Armor::Neckguard,
            22 => Armor::Faceplate,
            23 => Armor::Coat,
            24 => Armor::Tasset,
            _=> Armor::None,
        }
    }
    
}
