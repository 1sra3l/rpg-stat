/*!
# Character Class

This abstraction can be as `Basic` as `Hero` or `Enemy`
You can even use the fully `Advanced` version to use the entire class realm.

## Builder

Make anything pretty easily from built in classes `Basic`, `Normal`, and `Advanced`

```
use rpgstat::stats::Builder;
use rpgstat::stats::Basic as Stat;
use rpgstat::class::Basic as Class;

let class:Class = Class::Hero;
let stat:Stat<f64> = class.build_basic(0.0, 1.0);
```

### Build a `Basic` stat

This is what the code does for `Hero`

```
type T = f64;
let hp:T = num::cast(10).unwrap();
let mp:T = num::cast(5).unwrap();
let xp:T = num::cast(1).unwrap();
let xp_next:T = num::cast(10).unwrap();
let gp:T = num::cast(5).unwrap();
let speed:T = num::cast(10).unwrap();
```

### Build a `Normal` stat

This is what the code does for `Hero`

```
type T = f64;
let hp:T = num::cast(10).unwrap();
let mp:T = num::cast(5).unwrap();
let xp:T = num::cast(1).unwrap();
let xp_next:T = num::cast(10).unwrap();
let gp:T = num::cast(5).unwrap();
let speed:T = num::cast(10).unwrap();
let atk:T = num::cast(10).unwrap();
let def:T = num::cast(10).unwrap();
let m_atk:T = num::cast(10).unwrap();
let m_def:T = num::cast(10).unwrap();
```

### Build a `Advanced` stat

This is what the code does for `Hero`

```
type T = f64;
let hp:T = num::cast(10).unwrap();
let mp:T = num::cast(5).unwrap();
let xp:T = num::cast(1).unwrap();
let xp_next:T = num::cast(10).unwrap();
let gp:T = num::cast(5).unwrap();
let speed:T = num::cast(10).unwrap();
let atk:T = num::cast(10).unwrap();
let def:T = num::cast(10).unwrap();
let m_atk:T = num::cast(10).unwrap();
let m_def:T = num::cast(10).unwrap();
let agility:T = num::cast(10).unwrap();
let strength:T = num::cast(10).unwrap();
let dexterity:T = num::cast(10).unwrap();
let constitution:T = num::cast(10).unwrap();
let intelligence:T = num::cast(10).unwrap();
let charisma:T = num::cast(10).unwrap();
let wisdom:T = num::cast(10).unwrap();
let age:T = num::cast(10).unwrap();
```

*/
use std::fmt;
use serde::{Deserialize, Serialize};
use std::ops::{Add, AddAssign,  Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign};
//use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use std::fmt::Debug;

#[cfg(feature = "fltkform")]
use fltk::{prelude::*, *};
#[cfg(feature = "fltkform")]
use fltk_form_derive::*;
#[cfg(feature = "fltkform")]
use fltk_form::{FltkForm, HasProps};
#[cfg(feature = "fltkform")]
use std::collections::HashMap;
#[cfg(feature = "fltkform")]
use std::mem::transmute;

// RPG Stat
use crate::stats::Basic as BasicStats;
use crate::stats::Normal as NormalStats;
use crate::stats::Advanced as AdvancedStats;
use crate::stats::Builder;

/*
Alignement allows for the creation of multile outcomes in situations

This is a WIP currently

*/
#[allow(unused)]
#[derive(Clone, PartialEq, Copy, Debug, Serialize, Deserialize, EnumIter)]
#[cfg_attr(feature = "fltkform", derive(FltkForm))]
pub enum Alignment {
    Light,
    Dark,
    Undecided,
}
impl Default for Alignment {
    fn default() -> Self {
        Self::Undecided
    }
}
impl fmt::Display for Alignment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let v:String;
        match *self {
            Alignment::Light => v = String::from("Light"),
            Alignment::Dark => v = String::from("Dark"),
            Alignment::Undecided => v = String::from("Undecided"),
        }
        write!(f, "{}", v.as_str())
    }
}

/*
# Basic Class

Default to making an `Enemy`, because there are honestly few `Hero`s in games

## Builder

Make anything pretty easily from built in classes

```
use rpgstat::stat::Builder;
use rpgstat::stat::Basic as Stat;
use rpgstat::class::Basic as Class;

let class:Class = Class::Hero;
let stat:Stat<f64> = class.build_basic();
```

*/
#[allow(unused)]
#[derive(Clone, PartialEq, Copy, Debug, Serialize, Deserialize, EnumIter)]
#[cfg_attr(feature = "fltkform", derive(FltkForm))]
pub enum Basic {
    /// Obviously the protagonist
    Hero,
    /// Obviously the antagonist
    Enemy,
}
impl Default for Basic {
    fn default() -> Self {
        Self::Enemy
    }
}
impl fmt::Display for Basic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let v:String;
        match *self {
            Basic::Hero => v = String::from("Hero"),
            Basic::Enemy => v = String::from("Enemy"),
        }
        write!(f, "{}", v.as_str())
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
        let mut hp:T = num::cast(10).unwrap();
        let mut mp:T = num::cast(5).unwrap();
        let mut xp:T = num::cast(1).unwrap();
        let mut xp_next:T = num::cast(10).unwrap();
        let mut gp:T = num::cast(5).unwrap();
        let mut speed:T = num::cast(10).unwrap();
        match *self {
            Basic::Hero => {},
            Basic::Enemy => {
                xp = num::cast(1).unwrap();
                xp_next =num::cast(10).unwrap();
                hp = num::cast(5).unwrap();
                mp = num::cast(5).unwrap();
                speed = num::cast(7).unwrap();
            },
        }
        hp *= level;
        mp *= level;
        // TODO fixme:
        xp *= level;
        // TODO fixme:
        xp_next *= level;
        gp *= level;
        speed += level;
        BasicStats {
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
        }
        
    }
    fn build_normal(&self, id:T, level:T) -> NormalStats<T>{
        let mut hp:T = num::cast(10).unwrap();
        let mut mp:T = num::cast(5).unwrap();
        let mut xp:T = num::cast(1).unwrap();
        let mut xp_next:T = num::cast(10).unwrap();
        let mut gp:T = num::cast(5).unwrap();
        let mut speed:T = num::cast(10).unwrap();
        let atk:T = num::cast(10).unwrap();
        let def:T = num::cast(10).unwrap();
        let m_atk:T = num::cast(10).unwrap();
        let m_def:T = num::cast(10).unwrap();
        match *self {
            Basic::Hero => {},
            Basic::Enemy => {
                xp = num::cast(1).unwrap();
                xp_next =num::cast(10).unwrap();
                hp = num::cast(5).unwrap();
                mp = num::cast(5).unwrap();
                speed = num::cast(7).unwrap();
            },
        }
        hp *= level;
        mp *= level;
        // TODO fixme:
        xp *= level;
        // TODO fixme:
        xp_next *= level;
        gp *= level;
        speed += level;
        NormalStats {
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
        }
    }
    fn build_advanced(&self, id:T, level:T) -> AdvancedStats<T>{
        let mut hp:T = num::cast(10).unwrap();
        let mut mp:T = num::cast(5).unwrap();
        let mut xp:T = num::cast(1).unwrap();
        let mut xp_next:T = num::cast(10).unwrap();
        let mut gp:T = num::cast(5).unwrap();
        let mut speed:T = num::cast(10).unwrap();
        let atk:T = num::cast(10).unwrap();
        let def:T = num::cast(10).unwrap();
        let m_atk:T = num::cast(10).unwrap();
        let m_def:T = num::cast(10).unwrap();
        let agility:T = num::cast(10).unwrap();
        let strength:T = num::cast(10).unwrap();
        let dexterity:T = num::cast(10).unwrap();
        let constitution:T = num::cast(10).unwrap();
        let intelligence:T = num::cast(10).unwrap();
        let charisma:T = num::cast(10).unwrap();
        let wisdom:T = num::cast(10).unwrap();
        let age:T = num::cast(10).unwrap();
        match *self {
            Basic::Hero => {},
            Basic::Enemy => {
                xp = num::cast(1).unwrap();
                xp_next =num::cast(10).unwrap();
                hp = num::cast(5).unwrap();
                mp = num::cast(5).unwrap();
                speed = num::cast(7).unwrap();
            },
        }
        hp *= level;
        mp *= level;
        // TODO fixme:
        xp *= level;
        // TODO fixme:
        xp_next *= level;
        gp *= level;
        speed += level;
        AdvancedStats {
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
        }
    }
}
/*
# The "Normal" type of class

This can be used in combination with `Basic` if both sides are the same, to provide PvP, or soldier battle scenarios

To use this alone something like `Soldier` could be the enemies in that scenario, or however one sees fit.

You can couple these things with `Alignment` to produce variations on Monks, Elementals, Alchemists, etc..

## Builder

Make anything pretty easily from built in classes

```
use rpgstat::stat::Builder;
use rpgstat::stat::Normal as Stat;
use rpgstat::class::Normal as Class;

let class:Class = Class::Archer;
let stat:Stat<f64> = class.build_normal();
```

*/
#[allow(unused)]
#[derive(Clone, PartialEq, Copy, Debug, Serialize, Deserialize, EnumIter)]
#[cfg_attr(feature = "fltkform", derive(FltkForm))]
pub enum Normal {
    /// Full of concotions to both heal and harm.
    Alchemist,
    /// Aw, shoot you are good!
    Archer,
    /// Devoted to studying the elements and harnessing their powers
    Elemental,
    /// Generally this is the good guy in the story...
    Knight,
    /// Devoted to reading really old books and figuring out really old combinations of substances
    Monk,
    /// Into life magic, which overcomes death magic
    Priest,
    /// This is the default, because a lot of RPG games involve the `Soldier` in a dungeon being an enemy for whatever reason...
    Soldier,
    /// Incredibly stealthy, this rounded character can survive any environment
    Ranger,
    /// The elite female warriors from the deepest coldest winters
    Valkyrie,
}
impl Default for Normal {
    fn default() -> Self {
        Self::Soldier
    }
}
impl fmt::Display for  Normal{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let v:String;
        match *self {
            Normal::Alchemist => v = String::from("Alchemist"),
            Normal::Archer => v = String::from("Archer"),
            Normal::Elemental => v = String::from("Elemental"),
            Normal::Knight => v = String::from("Knight"),
            Normal::Monk => v = String::from("Monk"),
            Normal::Priest => v = String::from("Priest"),
            Normal::Soldier => v = String::from("Soldier"),
            Normal::Ranger => v = String::from("Ranger"),
            Normal::Valkyrie => v = String::from("Valkyrie"),
        }
        write!(f, "{}", v.as_str())
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
    + num::NumCast> Builder<T> for Normal {
    /// Build a `Basic` stat
    fn build_basic(&self, id:T, level:T) -> BasicStats<T>{
        let mut hp:T;
        let mut mp:T;
        let mut xp:T = num::cast(1).unwrap();
        let mut xp_next:T = num::cast(10).unwrap();
        let mut gp:T = num::cast(5).unwrap();
        let mut speed:T;
        match *self {
            Normal::Alchemist => {
                hp =  num::cast(40).unwrap();
                mp =  num::cast(16).unwrap();
                speed =  num::cast(5).unwrap();
            },
            Normal::Archer => {
                hp =  num::cast(50).unwrap();
                mp =  num::cast(25).unwrap();
                speed =  num::cast(5).unwrap();
            },
            Normal::Knight => {
                hp =  num::cast(50).unwrap();
                mp =  num::cast(20).unwrap();
                speed =  num::cast(7).unwrap();
            },
            Normal::Monk => {
                hp = num::cast(50).unwrap();
                mp = num::cast(20).unwrap();
                speed = num::cast(5).unwrap();
            },
            Normal::Elemental => {
                hp = num::cast(70).unwrap();
                mp = num::cast(40).unwrap();
                speed = num::cast(4).unwrap();
            },
            Normal::Priest => {
                hp = num::cast(60).unwrap();
                mp = num::cast(10).unwrap();
                speed = num::cast(4).unwrap();
            },
            Normal::Soldier => {
                hp = num::cast(90).unwrap();
                mp = num::cast(0).unwrap();
                speed = num::cast(5).unwrap();
            },
            Normal::Ranger => {
                hp = num::cast(40).unwrap();
                mp = num::cast(70).unwrap();
                speed = num::cast(8).unwrap();
            },
            Normal::Valkyrie => {
                hp = num::cast(50).unwrap();
                mp = num::cast(10).unwrap();
                speed = num::cast(7).unwrap();
            },
        }
        hp *= level;
        mp *= level;
        // TODO fixme:
        xp *= level;
        // TODO fixme:
        xp_next *= level;
        gp *= level;
        speed += level;
        BasicStats {
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
        }
        
    }
    // Build a `Normal` stat
    fn build_normal(&self, id:T, level:T) -> NormalStats<T>{
        let mut hp:T;
        let mut mp:T;
        let mut xp:T = num::cast(1).unwrap();
        let mut xp_next:T = num::cast(10).unwrap();
        let mut gp:T = num::cast(5).unwrap();
        let mut speed:T;
        let mut atk:T;
        let mut def:T;
        let mut m_atk:T;
        let mut m_def:T;
        match *self {
            Normal::Alchemist => {
                hp =  num::cast(40).unwrap();
                mp =  num::cast(16).unwrap();
                atk =  num::cast(14).unwrap();
                def =  num::cast(30).unwrap();
                m_atk =  num::cast(20).unwrap();
                m_def =  num::cast(30).unwrap();
                speed =  num::cast(5).unwrap();
            },
            Normal::Archer => {
                hp =  num::cast(50).unwrap();
                mp =  num::cast(25).unwrap();
                atk =  num::cast(15).unwrap();
                def =  num::cast(10).unwrap();
                m_atk =  num::cast(15).unwrap();
                m_def =  num::cast(35).unwrap();
                speed =  num::cast(5).unwrap();
            },
            Normal::Knight => {
                hp =  num::cast(50).unwrap();
                mp =  num::cast(20).unwrap();
                atk =  num::cast(20).unwrap();
                def =  num::cast(20).unwrap();
                m_atk =  num::cast(20).unwrap();
                m_def =  num::cast(20).unwrap();
                speed =  num::cast(7).unwrap();
            },
            Normal::Monk => {
                hp = num::cast(50).unwrap();
                mp = num::cast(20).unwrap();
                atk = num::cast(10).unwrap();
                def = num::cast(15).unwrap();
                m_atk = num::cast(5).unwrap();
                m_def = num::cast(40).unwrap();
                speed = num::cast(5).unwrap();
            },
            Normal::Elemental => {
                hp = num::cast(70).unwrap();
                mp = num::cast(40).unwrap();
                atk = num::cast(1).unwrap();
                def = num::cast(8).unwrap();
                m_atk = num::cast(30).unwrap();
                m_def = num::cast(1).unwrap();
                speed = num::cast(4).unwrap();
            },
            Normal::Priest => {
                hp = num::cast(60).unwrap();
                mp = num::cast(10).unwrap();
                atk = num::cast(20).unwrap();
                def = num::cast( 10).unwrap();
                m_atk = num::cast(10).unwrap();
                m_def = num::cast(40).unwrap();
                speed = num::cast(4).unwrap();
            },
            Normal::Soldier => {
                hp = num::cast(90).unwrap();
                mp = num::cast(0).unwrap();
                atk = num::cast(30).unwrap();
                def = num::cast(12).unwrap();
                m_atk = num::cast(0).unwrap();
                m_def = num::cast(18).unwrap();
                speed = num::cast(5).unwrap();
            },
            Normal::Ranger => {
                hp = num::cast(40).unwrap();
                mp = num::cast(70).unwrap();
                atk = num::cast(15).unwrap();
                def = num::cast( 9).unwrap();
                m_atk = num::cast(11).unwrap();
                m_def = num::cast(30).unwrap();
                speed = num::cast(8).unwrap();
            },
            Normal::Valkyrie => {
                hp = num::cast(50).unwrap();
                mp = num::cast(10).unwrap();
                atk = num::cast(20).unwrap();
                def = num::cast(20).unwrap();
                m_atk = num::cast(20).unwrap();
                m_def = num::cast(30).unwrap();
                speed = num::cast(7).unwrap();
            },
        }
        hp *= level;
        mp *= level;
        // TODO fixme:
        xp *= level;
        // TODO fixme:
        xp_next *= level;
        gp *= level;
        speed += level;
        atk += level;
        m_atk += level;
        def += level;
        m_def += level;
        NormalStats {
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
        }
    }

    // Build an `Advanced` stat
    fn build_advanced(&self, id:T, level:T) -> AdvancedStats<T>{
        let mut hp:T;
        let mut mp:T;
        let mut xp:T = num::cast(1).unwrap();
        let mut xp_next:T = num::cast(10).unwrap();
        let mut gp:T = num::cast(5).unwrap();
        let mut speed:T;
        let mut atk:T;
        let mut def:T;
        let mut m_atk:T;
        let mut m_def:T;
        let mut agility:T;
        let mut strength:T;
        let mut dexterity:T;
        let mut constitution:T;
        let mut intelligence:T;
        let mut charisma:T;
        let mut wisdom:T;
        let age:T;
        match *self {
           Normal::Alchemist => {
                hp =  num::cast(40).unwrap();
                mp =  num::cast(16).unwrap();
                atk =  num::cast(14).unwrap();
                def =  num::cast(30).unwrap();
                m_atk =  num::cast(20).unwrap();
                m_def =  num::cast(30).unwrap();
                speed =  num::cast(5).unwrap();
                agility = num::cast(10).unwrap();
                strength = num::cast(10).unwrap();
                dexterity = num::cast(10).unwrap();
                constitution = num::cast(10).unwrap();
                intelligence = num::cast(10).unwrap();
                charisma = num::cast(10).unwrap();
                wisdom = num::cast(10).unwrap();
                age = num::cast(40).unwrap();
            },
            Normal::Archer => {
                hp =  num::cast(50).unwrap();
                mp =  num::cast(25).unwrap();
                atk =  num::cast(15).unwrap();
                def =  num::cast(10).unwrap();
                m_atk =  num::cast(15).unwrap();
                m_def =  num::cast(35).unwrap();
                speed =  num::cast(5).unwrap();
                agility = num::cast(10).unwrap();
                strength = num::cast(10).unwrap();
                dexterity = num::cast(10).unwrap();
                constitution = num::cast(10).unwrap();
                intelligence = num::cast(10).unwrap();
                charisma = num::cast(10).unwrap();
                wisdom = num::cast(10).unwrap();
                age = num::cast(40).unwrap();
            },
            Normal::Knight => {
                hp =  num::cast(50).unwrap();
                mp =  num::cast(20).unwrap();
                atk =  num::cast(20).unwrap();
                def =  num::cast(20).unwrap();
                m_atk =  num::cast(20).unwrap();
                m_def =  num::cast(20).unwrap();
                speed =  num::cast(7).unwrap();
                agility = num::cast(10).unwrap();
                strength = num::cast(10).unwrap();
                dexterity = num::cast(10).unwrap();
                constitution = num::cast(10).unwrap();
                intelligence = num::cast(10).unwrap();
                charisma = num::cast(10).unwrap();
                wisdom = num::cast(10).unwrap();
                age = num::cast(40).unwrap();
            },
            Normal::Monk => {
                hp = num::cast(50).unwrap();
                mp = num::cast(20).unwrap();
                atk = num::cast(10).unwrap();
                def = num::cast(15).unwrap();
                m_atk = num::cast(5).unwrap();
                m_def = num::cast(40).unwrap();
                speed = num::cast(5).unwrap();
                agility = num::cast(10).unwrap();
                strength = num::cast(10).unwrap();
                dexterity = num::cast(10).unwrap();
                constitution = num::cast(10).unwrap();
                intelligence = num::cast(10).unwrap();
                charisma = num::cast(10).unwrap();
                wisdom = num::cast(10).unwrap();
                age = num::cast(40).unwrap();
            },
            Normal::Elemental => {
                hp = num::cast(70).unwrap();
                mp = num::cast(40).unwrap();
                atk = num::cast(1).unwrap();
                def = num::cast(8).unwrap();
                m_atk = num::cast(30).unwrap();
                m_def = num::cast(1).unwrap();
                speed = num::cast(4).unwrap();
                agility = num::cast(10).unwrap();
                strength = num::cast(10).unwrap();
                dexterity = num::cast(10).unwrap();
                constitution = num::cast(10).unwrap();
                intelligence = num::cast(10).unwrap();
                charisma = num::cast(10).unwrap();
                wisdom = num::cast(10).unwrap();
                age = num::cast(40).unwrap();
            },
            Normal::Priest => {
                hp = num::cast(60).unwrap();
                mp = num::cast(10).unwrap();
                atk = num::cast(20).unwrap();
                def = num::cast( 10).unwrap();
                m_atk = num::cast(10).unwrap();
                m_def = num::cast(40).unwrap();
                speed = num::cast(4).unwrap();
                agility = num::cast(10).unwrap();
                strength = num::cast(10).unwrap();
                dexterity = num::cast(10).unwrap();
                constitution = num::cast(10).unwrap();
                intelligence = num::cast(10).unwrap();
                charisma = num::cast(10).unwrap();
                wisdom = num::cast(10).unwrap();
                age = num::cast(40).unwrap();
            },
            Normal::Soldier => {
                hp = num::cast(90).unwrap();
                mp = num::cast(0).unwrap();
                atk = num::cast(30).unwrap();
                def = num::cast(12).unwrap();
                m_atk = num::cast(0).unwrap();
                m_def = num::cast(18).unwrap();
                speed = num::cast(5).unwrap();
                agility = num::cast(10).unwrap();
                strength = num::cast(10).unwrap();
                dexterity = num::cast(10).unwrap();
                constitution = num::cast(10).unwrap();
                intelligence = num::cast(10).unwrap();
                charisma = num::cast(10).unwrap();
                wisdom = num::cast(10).unwrap();
                age = num::cast(40).unwrap();
            },
            Normal::Ranger => {
                hp = num::cast(40).unwrap();
                mp = num::cast(70).unwrap();
                atk = num::cast(15).unwrap();
                def = num::cast( 9).unwrap();
                m_atk = num::cast(11).unwrap();
                m_def = num::cast(30).unwrap();
                speed = num::cast(8).unwrap();
                agility = num::cast(10).unwrap();
                strength = num::cast(10).unwrap();
                dexterity = num::cast(10).unwrap();
                constitution = num::cast(10).unwrap();
                intelligence = num::cast(10).unwrap();
                charisma = num::cast(10).unwrap();
                wisdom = num::cast(10).unwrap();
                age = num::cast(40).unwrap();
            },
            Normal::Valkyrie => {
                hp = num::cast(50).unwrap();
                mp = num::cast(10).unwrap();
                atk = num::cast(20).unwrap();
                def = num::cast(20).unwrap();
                m_atk = num::cast(20).unwrap();
                m_def = num::cast(30).unwrap();
                speed = num::cast(7).unwrap();
                agility = num::cast(10).unwrap();
                strength = num::cast(10).unwrap();
                dexterity = num::cast(10).unwrap();
                constitution = num::cast(10).unwrap();
                intelligence = num::cast(10).unwrap();
                charisma = num::cast(10).unwrap();
                wisdom = num::cast(10).unwrap();
                age = num::cast(40).unwrap();
            },
        }
        hp *= level;
        mp *= level;
        // TODO fixme:
        xp *= level;
        // TODO fixme:
        xp_next *= level;
        gp *= level;
        speed += level;
        speed += level;
        atk += level;
        m_atk += level;
        def += level;
        m_def += level;
        agility += level;
        strength += level;
        dexterity += level;
        constitution += level;
        intelligence += level;
        charisma += level;
        wisdom += level;
        AdvancedStats {
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
        }
    }
}
/*
# Advanced


*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, Deserialize, Serialize)]
#[cfg_attr(feature = "fltkform", derive(FltkForm))]
///  `Advanced`
pub enum Advanced {
    /// Adventurer - 
    Adventurer,
    /// Artisan - 
    Artisan,
    /// Clergy - 
    Clergy,
    /// Governmental - 
    Governmental,
    /// Sailor - 
    Sailor,
    /// Worker - 
    Worker,
    /// Hoarder - 
    Hoarder,
    /// Community - 
    Community,
    /// Sport - 
    Sport,
    /// Solo - 
    Solo,
    /// Research - 
    Research,
    /// Scientist - 
    Scientist,
    /// Engineer - 
    Engineer,
    /// Clown - 
    Clown,
    /// Musician - 
    Musician,
    /// Baker - 
    Baker,
    /// Gardener - 
    Gardener,
    /// Boring - 
    Boring,
    /// Random - 
    Random,
    /// Gambler - 
    Gambler,
    /// Bicyclist - 
    Bicyclist,
    /// SkateBoarder - 
    SkateBoarder,
    /// Climber - 
    Climber,
    /// Watcher - 
    Watcher,
    /// Spy - 
    Spy,
    /// Shinobi - 
    Shinobi,
    /// Samurai - 
    Samurai,
    /// Shaolin - 
    Shaolin,
    /// Knitter - 
    Knitter,
    /// Crocheter - 
    Crocheter,
    /// Student - 
    Student,
    /// Teacher - 
    Teacher,
    /// Spiritual - 
    Spiritual,
    /// Farmer - 
    Farmer,
    /// Metallurgist - 
    Metallurgist,
    /// Archivist - 
    Archivist,
    /// Janitor - 
    Janitor,
    /// Cook - 
    Cook,
    /// Florist - 
    Florist,
    /// HomeMaker - 
    HomeMaker,
    /// Actor - 
    Actor,
    /// GameMaker - 
    GameMaker,
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
            Advanced::Adventurer => v = String::from("Adventurer"),
            Advanced::Artistan => v = String::from("Artistan"),
            Advanced::Clergy => v = String::from("Clergy"),
            Advanced::Governmental => v = String::from("Governmental"),
            Advanced::Sailor => v = String::from("Sailor"),
            Advanced::Worker => v = String::from("Worker"),
            Advanced::Horder => v = String::from("Horder"),
            Advanced::Community => v = String::from("Community"),
            Advanced::Sport => v = String::from("Sport"),
            Advanced::Solo => v = String::from("Solo"),
            Advanced::Research => v = String::from("Research"),
            Advanced::Scientist => v = String::from("Scientist"),
            Advanced::Engineer => v = String::from("Engineer"),
            Advanced::Clown => v = String::from("Clown"),
            Advanced::Musician => v = String::from("Musician"),
            Advanced::Baker => v = String::from("Baker"),
            Advanced::Gardener => v = String::from("Gardener"),
            Advanced::Boring => v = String::from("Boring"),
            Advanced::Random => v = String::from("Random"),
            Advanced::Gambler => v = String::from("Gambler"),
            Advanced::Bicyclist => v = String::from("Bicyclist"),
            Advanced::SkateBoarder => v = String::from("SkateBoarder"),
            Advanced::Climber => v = String::from("Climber"),
            Advanced::Watcher => v = String::from("Watcher"),
            Advanced::Spy => v = String::from("Spy"),
            Advanced::Shinobi => v = String::from("Shinobi"),
            Advanced::Samurai => v = String::from("Samurai"),
            Advanced::Shaolin => v = String::from("Shaolin"),
            Advanced::Knitter => v = String::from("Knitter"),
            Advanced::Crocheter => v = String::from("Crocheter"),
            Advanced::Student => v = String::from("Student"),
            Advanced::Teacher => v = String::from("Teacher"),
            Advanced::Spiritual => v = String::from("Spiritual"),
            Advanced::Farmer => v = String::from("Farmer"),
            Advanced::Metalurgist => v = String::from("Metalurgist"),
            Advanced::Archivist => v = String::from("Archivist"),
            Advanced::Janitor => v = String::from("Janitor"),
            Advanced::Cook => v = String::from("Cook"),
            Advanced::Florist => v = String::from("Florist"),
            Advanced::HomeMaker => v = String::from("HomeMaker"),
            Advanced::Actor => v = String::from("Actor"),
            Advanced::GameMaker => v = String::from("GameMaker"),
            _=> v = String::from("None"),
        }
        write!(f, "{}", v.as_str())
    }
}
impl Random for Advanced {
    type Type = Advanced;
    fn random_type(&self) -> Self::Type {
        let max = 42;
        let val = self.random_rate(max);
        match val {
            0 => Advanced::Adventurer,
            1 => Advanced::Artistan,
            2 => Advanced::Clergy,
            3 => Advanced::Governmental,
            4 => Advanced::Sailor,
            5 => Advanced::Worker,
            6 => Advanced::Horder,
            7 => Advanced::Community,
            8 => Advanced::Sport,
            9 => Advanced::Solo,
            10 => Advanced::Research,
            11 => Advanced::Scientist,
            12 => Advanced::Engineer,
            13 => Advanced::Clown,
            14 => Advanced::Musician,
            15 => Advanced::Baker,
            16 => Advanced::Gardener,
            17 => Advanced::Boring,
            18 => Advanced::Random,
            19 => Advanced::Gambler,
            20 => Advanced::Bicyclist,
            21 => Advanced::SkateBoarder,
            22 => Advanced::Climber,
            23 => Advanced::Watcher,
            24 => Advanced::Spy,
            25 => Advanced::Shinobi,
            26 => Advanced::Samurai,
            27 => Advanced::Shaolin,
            28 => Advanced::Knitter,
            29 => Advanced::Crocheter,
            30 => Advanced::Student,
            31 => Advanced::Teacher,
            32 => Advanced::Spiritual,
            33 => Advanced::Farmer,
            34 => Advanced::Metalurgist,
            35 => Advanced::Archivist,
            36 => Advanced::Janitor,
            37 => Advanced::Cook,
            38 => Advanced::Florist,
            39 => Advanced::HomeMaker,
            40 => Advanced::Actor,
            41 => Advanced::GameMaker,
            _=> Advanced::None,
        }
    }
    
}
