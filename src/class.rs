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
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

// RPG Stat
use crate::stats::Basic as BasicStats;
use crate::stats::Normal as NormalStats;
use crate::stats::Advanced as AdvancedStats;
use crate::stats::Builder;

/*
Alignement allows for the creation of multile outcomes in situations

This is a WIP currently

*/
#[derive(Clone, PartialEq, Copy, Debug, Serialize, Deserialize, EnumIter)]
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
#[derive(Clone, PartialEq, Copy, Debug, Serialize, Deserialize, EnumIter)]
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
            id:id,
            xp:xp,
            xp_next:xp_next,
            level:level,
            gp:gp,
            hp: hp,
            mp: mp,
            hp_max: hp,
            mp_max: mp,
            speed: speed,
        }
        
    }
    fn build_normal(&self, id:T, level:T) -> NormalStats<T>{
        let mut hp:T = num::cast(10).unwrap();
        let mut mp:T = num::cast(5).unwrap();
        let mut xp:T = num::cast(1).unwrap();
        let mut xp_next:T = num::cast(10).unwrap();
        let mut gp:T = num::cast(5).unwrap();
        let mut speed:T = num::cast(10).unwrap();
        let mut atk:T = num::cast(10).unwrap();
        let mut def:T = num::cast(10).unwrap();
        let mut m_atk:T = num::cast(10).unwrap();
        let mut m_def:T = num::cast(10).unwrap();
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
            id:id,
            xp:xp,
            xp_next:xp_next,
            level:level,
            gp:gp,
            hp: hp,
            mp: mp,
            hp_max: hp,
            mp_max: mp,
            speed: speed,
            atk:atk,
            def:def,
            m_atk:m_atk,
            m_def:m_def,
        }
    }
    fn build_advanced(&self, id:T, level:T) -> AdvancedStats<T>{
        let mut hp:T = num::cast(10).unwrap();
        let mut mp:T = num::cast(5).unwrap();
        let mut xp:T = num::cast(1).unwrap();
        let mut xp_next:T = num::cast(10).unwrap();
        let mut gp:T = num::cast(5).unwrap();
        let mut speed:T = num::cast(10).unwrap();
        let mut atk:T = num::cast(10).unwrap();
        let mut def:T = num::cast(10).unwrap();
        let mut m_atk:T = num::cast(10).unwrap();
        let mut m_def:T = num::cast(10).unwrap();
        let mut agility:T = num::cast(10).unwrap();
        let mut strength:T = num::cast(10).unwrap();
        let mut dexterity:T = num::cast(10).unwrap();
        let mut constitution:T = num::cast(10).unwrap();
        let mut intelligence:T = num::cast(10).unwrap();
        let mut charisma:T = num::cast(10).unwrap();
        let mut wisdom:T = num::cast(10).unwrap();
        let mut age:T = num::cast(10).unwrap();
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
            id:id,
            xp:xp,
            xp_next:xp_next,
            level:level,
            gp:gp,
            hp: hp,
            mp: mp,
            hp_max: hp,
            mp_max: mp,
            speed: speed,
            atk:atk,
            def:def,
            m_atk:m_atk,
            m_def:m_def,
            agility:agility,
            strength:strength,
            dexterity:dexterity,
            constitution:constitution,
            intelligence:intelligence,
            charisma:charisma,
            wisdom:wisdom,
            age:age,
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
#[derive(Clone, PartialEq, Copy, Debug, Serialize, Deserialize, EnumIter)]
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
        let mut hp:T = num::cast(10).unwrap();
        let mut mp:T = num::cast(5).unwrap();
        let mut xp:T = num::cast(1).unwrap();
        let mut xp_next:T = num::cast(10).unwrap();
        let mut gp:T = num::cast(5).unwrap();
        let mut speed:T = num::cast(5).unwrap();
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
            id:id,
            xp:xp,
            xp_next:xp_next,
            level:level,
            gp:gp,
            hp: hp,
            mp: mp,
            hp_max: hp,
            mp_max: mp,
            speed: speed,
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
        NormalStats {
            id:id,
            xp:xp,
            xp_next:xp_next,
            level:level,
            gp:gp,
            hp: hp,
            mp: mp,
            hp_max: hp,
            mp_max: mp,
            speed: speed,
            atk:atk,
            def:def,
            m_atk:m_atk,
            m_def:m_def,
        }
    }

    // Build an `Advanced` stat
    fn build_advanced(&self, id:T, level:T) -> AdvancedStats<T>{
        let mut hp:T = num::cast(10).unwrap();
        let mut mp:T = num::cast(5).unwrap();
        let mut xp:T = num::cast(1).unwrap();
        let mut xp_next:T = num::cast(10).unwrap();
        let mut gp:T = num::cast(5).unwrap();
        let mut speed:T = num::cast(5).unwrap();
        let mut atk:T = num::cast(10).unwrap();
        let mut def:T = num::cast(10).unwrap();
        let mut m_atk:T = num::cast(10).unwrap();
        let mut m_def:T = num::cast(10).unwrap();
        let mut agility:T = num::cast(10).unwrap();
        let mut strength:T = num::cast(10).unwrap();
        let mut dexterity:T = num::cast(10).unwrap();
        let mut constitution:T = num::cast(10).unwrap();
        let mut intelligence:T = num::cast(10).unwrap();
        let mut charisma:T = num::cast(10).unwrap();
        let mut wisdom:T = num::cast(10).unwrap();
        let mut age:T = num::cast(10).unwrap();
        //TODO OR use legendary.ini + serde
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
        AdvancedStats {
            id:id,
            xp:xp,
            xp_next:xp_next,
            level:level,
            gp:gp,
            hp: hp,
            mp: mp,
            hp_max: hp,
            mp_max: mp,
            speed: speed,
            atk:atk,
            def:def,
            m_atk:m_atk,
            m_def:m_def,
            agility:agility,
            strength:strength,
            dexterity:dexterity,
            constitution:constitution,
            intelligence:intelligence,
            charisma:charisma,
            wisdom:wisdom,
            age:age,
        }
    }
}
/*
The Advanced group includes more support type characters.  Thhese classes can be used primarily for NPC type characters in games as well as the `Normal` types also included in `Advanced`

You can couple these things with `Alignment` to produce variations on Monks, Elementals, Alchemists, etc..

## Builder

Make anything pretty easily from built in classes

```
use rpgstat::stat::Builder;
use rpgstat::stat::Advanced as Stat;
use rpgstat::class::Advanced as Class;
use rpgstat::stats::Builder;
let class:Class = Class::Artisan;
let stat:Stat<f64> = class.build_advanced();
```
*/
pub enum Advanced {
    /// The goal is to discover new things without needing to get into fights
    Adventurer,
    /// Scientist who creates useful medicines
    Alchemist,
    /// Often found hunting for game
    Archer,
    /// Crafting and creating goods of many uses
    Artistan,
    /// Focused solely on support for other players/characters and their healing
    Clergy,
    /// Devoted to studying the elements and harnessing their powers
    Elemental,
    /// People devoted to keeping roads fixed and laws in place
    Governmental,
    /// A  protector of the `Governmental` class
    Knight,
    /// Devoted to reading really old books and figuring out really old combinations of substances
    Monk,
    /// Into life magic, which overcomes death magic
    Priest,
    /// Known to do well in fights, they generally keep to the sea
    Sailor,
    /// Travelling abroad to wage wars, they return infrequently
    Soldier,
    /// Incredibly stealthy, this rounded character can survive any environment
    Ranger,
    /// The elite female warriors from the deepest coldest winters
    Valkyrie,
    /// One of the various classes of people who work
    Worker,
}
impl Default for Advanced {
    fn default() -> Self {
        Self::Worker
    }
}
impl fmt::Display for  Advanced{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let v:String;
        match *self {
            Advanced::Adventurer => v = String::from("Adventurer"),
            Advanced::Alchemist => v = String::from("Alchemist"),
            Advanced::Archer => v = String::from("Archer"),
            Advanced::Artistan => v = String::from("Artistan"),
            Advanced::Clergy => v = String::from("Clergy"),
            Advanced::Elemental => v = String::from("Elemental"),
            Advanced::Governmental => v = String::from("Governmental"),
            Advanced::Knight => v = String::from("Knight"),
            Advanced::Monk => v = String::from("Monk"),
            Advanced::Priest => v = String::from("Priest"),
            Advanced::Sailor => v = String::from("Sailor"),
            Advanced::Soldier => v = String::from("Soldier"),
            Advanced::Ranger => v = String::from("Ranger"),
            Advanced::Valkyrie => v = String::from("Valkyrie"),
            Advanced::Worker => v = String::from("Worker"),
        }
        write!(f, "{}", v.as_str())
    }
}
