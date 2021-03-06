/*!
# Stats

This module offers a few options.

### FLTK

The use of `fltk-form` allows easy integration into a GUI toolkit via the FLTK specific `Stats`

### Premade traits to add to your own struct

```
use rpg_stat::stats::Basic as Stats;
use rpg_stat::stats::BasicPremade as Premade;

struct MyStruct {
    stats:Stats<f64>,
    // whatever else you need
}
// now you can get the stats `hp` via `my_struct.hp()`, etc..
impl Premade<f64> for MyStruct {
    fn stat(&self) -> Stats<f64> {
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
    
}
```

And for FLTK premade f64:

```
use rpg_stat::stats::Stats;
use rpg_stat::stats::Premade;

struct MyStruct {
    stats:Stats,
    // whatever else you need
}
// now you can get the stats `hp` via `my_struct.hp()`, etc..
impl Premade for MyStruct {
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

```

# Structure
This contains the basic structures for the RPG statistics library

## `Basic` contains only the most needed for a generic game

### id
this can be used in any way to ID the stats

### xp
The experience points the stats currently hold

### xp_next
the amount needed to reach the next level

### level
the level of proficiency of the stats

### gp
the currency the stats currently have

### hp
the current health in the stats

### mp
the current mana in the stats

### hp_max
the total health possible

### mp_max
the total mana possible

### speed
the speed the stats move at

## `Normal` includes a few more for the generic RPG battle system as well as everything in `Basic`

### atk
used specifically in battle as the attack variable

### def
used specifically in battle as the defense variable

### m_atk
used specifically in battle as the mana attack variable

### m_def
used specifically in battle as the mana defense variable

## `Advanced` contains the finer details seen in tabletop RPG stats as well as everything in `Normal` and `Basic`

### agility

Fight mechanics:
 * Increases dodge
 * Increases accuracy

Story mechanics:
 * Increases success rate
 * Increases options
 * Decreases confrontation rate

### strength

Fight mechanics:
 * Increases attack
 * Increases defense

Story mechanics:
 * Increases confrontations
 * Increases special item finds
 * Increases success rate

### dexterity

Fight mechanics:
 * Increases accuracy

Story mechanics:
 * Increases options
 * Decreases confrontation rate

### constitution

Fight mechanics:
 * Increases dodge
 * Increases defense

Story mechanics:
 * Increases success rate
 * Decreases confrontation rate

### intelligence

Fight mechanics:
 * Increases accuracy

Story mechanics:
 * Increases confrontations
 * Increases special item finds
 * Increases success rate

### charisma

Fight mechanics:
 * Increases dodge

Story mechanics:
 * Increases options
 * Increases reward
 * Decreases confrontation rate

### wisdom

Fight mechanics:
 * Increases leveling

Story mechanics:
 * Increases reward
 * Decreases confrontation rate

### age

Age is really to allow things to grow through a Stage

Fight mechanics:
 * Items, Weapons, Armor, etc may be formulated for specific age ranges and prevent users who are too young or old to use them.

Story mechanics:
 * Age could potentially influence story mechanics, being to young to leave an area would promote grinding.

 */
use std::default::Default;
use serde::{Deserialize, Serialize};
extern crate num;

use std::ops::{Add, AddAssign,  Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign};
use std::fmt::Debug;

use std::fs::File;
use std::io::Write;
use std::path::Path;

#[cfg(feature = "fltkform")]
use fltk::{prelude::*, *};
#[cfg(feature = "fltkform")]
use fltk_form_derive::*;
#[cfg(feature = "fltkform")]
use fltk_form::FltkForm;


use crate::random::*;
use crate::equation::Math;

pub trait Functions <T:Copy
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
    + num::NumCast> {
    type Statistics;
    fn damage(&self);
}
/*
# Builder

The builder trait is how I create `rpg_stat::stats::{Basic,Normal,Advance}` from enums

*/
pub trait Builder <T:Copy
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
    + num::NumCast> {
    /// Build a `Basic` stat
    fn build_basic(&self, id:T, level:T) -> Basic<T>;
    // Build a `Normal` stat
    fn build_normal(&self, id:T, level:T) -> Normal<T>;
    // Build an `Advanced` stat
    fn build_advanced(&self, id:T, level:T) -> Advanced<T>;
}

/*
Premade trait for Basic Stat
You simply define the function `stat()` to return the `Basic<T>` associated with your code.
*/
pub trait BasicPremade<T:Copy
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
    + num::NumCast> {
    /// # Function you need to imlement
    /// stat returns the `Basic<T>` you created
    fn stat(&self) -> Basic<T>;
    /// # Function you need to imlement
    /// Set the `Basic<T>` Health Points
    fn set_hp(&mut self, amount:T);
    /// # Function you need to imlement
    /// Set the `Basic<T>` Mana Points
    fn set_mp(&mut self, amount:T);
    /// # Function you need to imlement
    /// Set the `Basic<T>` Experience Points
    fn set_xp(&mut self, amount:T);
    /// # Function you need to imlement
    /// Set the `Basic<T>` Max Health Points
    fn set_hp_max(&mut self, amount:T);
    /// # Function you need to imlement
    /// Set the `Basic<T>` Max Mana Points
    fn set_mp_max(&mut self, amount:T);
    /// # Function you need to imlement
    /// Set the `Basic<T>` Next Experience Points
    fn set_xp_next(&mut self, amount:T);
    /// # Function you need to imlement
    /// Set the `Basic<T>` Gold Points
    fn set_gp(&mut self, amount:T);
    /// Return  the `Basic<T>` id number

// PREMADE FUNCTIONS   
    fn id(&self) -> T {
        self.stat().id
    }
    /// Return  the `Basic<T>` Health Points
    fn hp(&self) -> T {
        self.stat().hp
    }
    /// Return  the `Basic<T>` Mana Points
    fn mp(&self) -> T {
        self.stat().mp
    }
    /// Return  the `Basic<T>` Experience Points
    fn xp(&self) -> T {
        self.stat().xp
    }
    /// Return  the `Basic<T>` Max Health Points
    fn hp_max(&self) -> T {
        self.stat().hp_max
    }
    /// Return  the `Basic<T>` Max Mana Points
    fn mp_max(&self) -> T {
        self.stat().mp_max
    }
    /// Return  the `Basic<T>` Next Experience Points
    fn xp_next(&self) -> T {
        self.stat().xp_next
    }
    /// Return  the `Basic<T>` Level
    fn level(&self) -> T {
        self.stat().level
    }
    /// Return  the `Basic<T>` Speed
    fn speed(&self) -> T {
        self.stat().level
    }
    /// Return  the `Basic<T>` Gold Points
    fn gp(&self) -> T {
        self.stat().gp
    }
    /// Damage the character by an amount
    fn damage(&mut self, amount:T) {
        let mut val = self.hp();
        val -= amount;
        let none = num::cast(0).unwrap();
        if val < none {
            val = none;
        }
        self.set_hp(val)
    }
    /// Add health to character but not beyond their Max Healh Points
    fn heal(&mut self, amount:T) {
        let mut val = self.hp();
        val += amount;
        let max = self.hp_max();
        if val > max {
            val = max;
        }
        self.set_hp(val)
    }
}

/*
# The Basic HP/MP/XP stat model

This basic model of stats is easy to work with for beginners, but powerful enough to be used by the most experienced.
*/

#[derive( Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Basic<T:Copy
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
    + num::NumCast> {
    /// Identification Number
    pub id:T,
    /// Experience Points
    pub xp:T,
    /// Health Points
    pub hp:T,
    /// Mana Points
    pub mp:T,
    /// Experience Points multiplier for next level
    pub xp_next:T,
    /// Max Health Points
    pub hp_max:T,
    /// Max Mana Points
    pub mp_max:T,
    /// Current Level
    pub level:T,
    /// The speed
    pub speed:T,
    /// your currency points
    pub gp:T,
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
    + num::NumCast> Basic<T> {

    /// make empty stats
    #[allow(unused)]
    pub fn empty() -> Self where Self:Sized {
        Basic {
            id:Default::default(),
            xp:Default::default(),
            xp_next:Default::default(),
            mp:Default::default(),
            hp:Default::default(),
            mp_max:Default::default(),
            hp_max:Default::default(),
            level:Default::default(),
            speed:Default::default(),
            gp:Default::default(),
        }
    }
    /// People like new
    #[allow(unused)]
    pub fn new() -> Self {
        Self::empty()
    }
    #[allow(unused)]
    /// Get the next amount of XP needed to level up
    pub fn next(&self) -> T {
        self.level * self.xp_next
    }
    #[allow(unused)]
    /// a vector of stats used to get the standard deviation
    pub fn stats_vec(&self) -> Vec<T>{
        vec![
            self.hp_max,
            self.mp_max,
            self.speed,
        ]
    }

    #[allow(unused)]
    /// This function levels up our stats
    pub fn level_up(&mut self) -> bool {
        if self.xp > self.next() {
            let stats_vec:Vec<T> = self.stats_vec();
            let mut num:T = Math::population_standard_deviation(stats_vec);
            let one:T = num::cast::<u32, T>(1).unwrap();
            if num < one {
                num = one;
            }
            num *= self.level;
            self.level += num;
            self.mp_max += num;
            self.hp_max += num;
            self.speed += num;
            return true;
        }
        false
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
    + num::NumCast> Default for Basic<T> {
    /// Default to empty
    fn default() -> Self where Self:Sized {
        Self::empty()
    }
}

/*
Premade trait for Normal Stat
You simply define the function `stat()` to return the `Normal<T>` associated with your code.
*/
pub trait NormalPremade<T:Copy
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
    + num::NumCast> {
    /// # Function you need to imlement
    /// stat returns the `Normal<T>` you created
    fn stat(&self) -> Normal<T>;
    /// # Function you need to imlement
    /// Set the `Normal<T>` Health Points
    fn set_hp(&mut self, amount:T);
    /// # Function you need to imlement
    /// Set the `Normal<T>` Mana Points
    fn set_mp(&mut self, amount:T);
    /// # Function you need to imlement
    /// Set the `Normal<T>` Experience Points
    fn set_xp(&mut self, amount:T);
    /// # Function you need to imlement
    /// Set the `Normal<T>` Max Health Points
    fn set_hp_max(&mut self, amount:T);
    /// # Function you need to imlement
    /// Set the `Normal<T>` Max Mana Points
    fn set_mp_max(&mut self, amount:T);
    /// # Function you need to imlement
    /// Set the `Normal<T>` Next Experience Points
    fn set_xp_next(&mut self, amount:T);
    /// # Function you need to imlement
    /// Set the `Normal<T>` Gold Points
    fn set_gp(&mut self, amount:T);
    /// # Function you need to imlement
    /// Set the `Normal<T>` Attack Points
    fn set_atk(&mut self, amount:T);
    /// # Function you need to imlement
    /// Set the `Normal<T>` Defense Points
    fn set_def(&mut self, amount:T);
    /// # Function you need to imlement
    /// Set the `Normal<T>` Mana Attack Points
    fn set_m_atk(&mut self, amount:T);
    /// # Function you need to imlement
    /// Set the `Normal<T>` Mana Defense Points
    fn set_m_def(&mut self, amount:T);

// PREMADE FUNCTIONS   
    /// Return  the `Normal<T>` id number
    fn id(&self) -> T {
        self.stat().id
    }
    /// Return  the `Normal<T>` Health Points
    fn hp(&self) -> T {
        self.stat().hp
    }
    /// Return  the `Normal<T>` Mana Points
    fn mp(&self) -> T {
        self.stat().mp
    }
    /// Return  the `Normal<T>` Experience Points
    fn xp(&self) -> T {
        self.stat().xp
    }
    /// Return  the `Normal<T>` Max Health Points
    fn hp_max(&self) -> T {
        self.stat().hp_max
    }
    /// Return  the `Normal<T>` Max Mana Points
    fn mp_max(&self) -> T {
        self.stat().mp_max
    }
    /// Return  the `Normal<T>` Next Experience Points
    fn xp_next(&self) -> T {
        self.stat().xp_next
    }
    /// Return  the `Normal<T>` Level
    fn level(&self) -> T {
        self.stat().level
    }
    /// Return  the `Normal<T>` Speed
    fn speed(&self) -> T {
        self.stat().level
    }
    /// Return  the `Normal<T>` Gold Points
    fn gp(&self) -> T {
        self.stat().gp
    }
    /// Return  the `Normal<T>` Attack Points
    fn atk(&self) -> T {
        self.stat().atk
    }
    /// Return  the `Normal<T>` Defense Points
    fn def(&self) -> T {
        self.stat().def
    }
    /// Return  the `Normal<T>` Mana Attack Points
    fn m_atk(&self) -> T {
        self.stat().m_atk
    }
    /// Return  the `Normal<T>` Mana Defense Points
    fn m_def(&self) -> T {
        self.stat().m_def
    }
    /// Damage the character by an amount
    fn damage(&mut self, amount:T) {
        let mut val = self.hp();
        val -= amount;
        let none = num::cast(0).unwrap();
        if val < none {
            val = none;
        }
        self.set_hp(val)
    }
    /// Add health to character but not beyond their Max Healh Points
    fn heal(&mut self, amount:T) {
        let mut val = self.hp();
        val += amount;
        let max = self.hp_max();
        if val > max {
            val = max;
        }
        self.set_hp(val)
    }
    /// Stable attack forumla
    /// [attack*(100/(100+defense))](https://rpg.fandom.com/wiki/Damage_Formula)
    fn attack_stable(&self, other:Normal<T>) -> T { 
        let hundred = num::cast::<u32,T>(100).unwrap();
        let val = self.atk();
        let def = other.def + hundred;
        let res = hundred / def;
        val * res
    }
    /// Full attack formula
    /// This attack is based off of current `hp` level as well as `hp_max`, and `atk`
    fn attack(&self, other:Normal<T>) -> T {
        let atk:T = Math::quarter(self.hp() 
                       //rand(Math::half(max), max)
                     ) + self.atk();
        let def:T = other.def;//ense();
        let ret:T = atk - def;
        let none = num::cast::<u32,T>(0).unwrap();
        if ret < none {
            return none
        }
        ret
    }
    /// An equation to simulate a full defense
    /// this relies on current `hp` as well as `def`
    fn defense(&self) -> T {
        Math::quarter(self.hp()) + self.def()
    }
}
/*
# The Normal

This model provides fine tuning of attack and defense without needing all the fine tuning of a full stat sheet
*/
#[derive( Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Normal<T:Copy 
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
    /// Identification Number
    pub id:T,
    // Name
    //pub name:str,
    /// Experience Points
    pub xp:T,
    /// Health Points
    pub hp:T,
    /// Mana Points
    pub mp:T,
    /// Experience Points multiplier for next level
    pub xp_next:T,
    /// Max Health Points
    pub hp_max:T,
    /// Max Mana Points
    pub mp_max:T,
    /// Current Level
    pub level:T,
    /// The speed
    pub speed:T,
    /// your currency points
    pub gp:T,
    /// Attack
    pub atk:T,
    /// Defense
    pub def:T,
    /// Mana Attack
    pub m_atk:T,
    /// Mana Defense
    pub m_def:T,

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
    + num::NumCast> Normal<T> {
    
    /// make empty stats
    #[allow(unused)]
    pub fn empty<U:Default>() -> Self {
        Normal {
            //name: "Ferris",
            id:Default::default(),
            xp:Default::default(),
            xp_next:Default::default(),
            mp:Default::default(),
            hp:Default::default(),
            mp_max:Default::default(),
            hp_max:Default::default(),
            level:Default::default(),
            speed:Default::default(),
            gp:Default::default(),
            atk:Default::default(),
            def:Default::default(),
            m_atk:Default::default(),
            m_def:Default::default(),
        }
    }/// People like new
    #[allow(unused)]
    pub fn new() -> Self {
        Self::empty::<T>()
    }
    #[allow(unused)]
    /// Get the next amount of XP needed to level up
    pub fn next(&self) -> T {
        self.level * self.xp_next
    }
    #[allow(unused)]
    /// a vector of stats used to get the standard deviation
    pub fn stats_vec(&self) -> Vec<T>{
        vec![
            self.hp_max,
            self.mp_max,
            self.speed,
            self.atk,
            self.def,
            self.m_atk,
            self.m_def,
        ]
    }

    #[allow(unused)]
    /// This function levels up our stats
    pub fn level_up(&mut self) -> bool {
        if self.xp > self.next() {
            let stats_vec:Vec<T> = self.stats_vec();
            let mut num:T = Math::population_standard_deviation(stats_vec);
            let one:T = num::cast::<u32, T>(1).unwrap();
            if num < one {
                num = one;
            }
            num *= self.level;
            self.level += num;
            self.mp_max += num;
            self.hp_max += num;
            self.speed += num;
            self.atk += num;
            self.def += num;
            self.m_atk += num;
            self.m_def += num;
            return true;
        }
        false
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
    + num::NumCast> Default for Normal<T> {
    /// Default to empty
    fn default() -> Self {
        Self::empty::<T>()
    }
}

/*
Premade trait for Advanced Stat
You simply define the function `stat()` to return the `Advanced<T>` associated with your code.
*/
pub trait AdvancedPremade<T:Copy
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
    + num::NumCast> {
    /// # Function you need to imlement
    /// stat returns the `Advanced<T>` you created
    fn stat(&self) -> Advanced<T>;
    /// # Function you need to imlement
    /// Set the `Advanced<T>` Health Points
    fn set_hp(&mut self, amount:T);
    /// # Function you need to imlement
    /// Set the `Advanced<T>` Mana Points
    fn set_mp(&mut self, amount:T);
    /// # Function you need to imlement
    /// Set the `Advanced<T>` Experience Points
    fn set_xp(&mut self, amount:T);
    /// # Function you need to imlement
    /// Set the `Advanced<T>` Max Health Points
    fn set_hp_max(&mut self, amount:T);
    /// # Function you need to imlement
    /// Set the `Advanced<T>` Max Mana Points
    fn set_mp_max(&mut self, amount:T);
    /// # Function you need to imlement
    /// Set the `Advanced<T>` Next Experience Points
    fn set_xp_next(&mut self, amount:T);
    /// # Function you need to imlement
    /// Set the `Advanced<T>` Gold Points
    fn set_gp(&mut self, amount:T);
    /// # Function you need to imlement
    /// Set the `Advanced<T>` Attack Points
    fn set_atk(&mut self, amount:T);
    /// # Function you need to imlement
    /// Set the `Advanced<T>` Defense Points
    fn set_def(&mut self, amount:T);
    /// # Function you need to imlement
    /// Set the `Advanced<T>` Mana Attack Points
    fn set_m_atk(&mut self, amount:T);
    /// # Function you need to imlement
    /// Set the `Advanced<T>` Mana Defense Points
    fn set_m_def(&mut self, amount:T);

// PREMADE FUNCTIONS
    /// Get the id number
    fn id(&self) -> T {
        self.stat().id
    }
    /// Get the Health Points
    fn hp(&self) -> T {
        self.stat().hp
    }
    /// Get the Mana Points
    fn mp(&self) -> T {
        self.stat().mp
    }
    /// Get the Experience Points
    fn xp(&self) -> T {
        self.stat().xp
    }
    /// Get the Max Health Points
    fn hp_max(&self) -> T {
        self.stat().hp_max
    }
    /// Get the Max Mana Points
    fn mp_max(&self) -> T {
        self.stat().mp_max
    }
    /// Get the Experience Points
    fn xp_next(&self) -> T {
        self.stat().xp_next
    }
    /// Get the Current Level
    fn level(&self) -> T {
        self.stat().level
    }
    /// Get the Speed
    fn speed(&self) -> T {
        self.stat().level
    }
    /// Get the Attack Points
    fn atk(&self) -> T {
        self.stat().atk
    }
    /// Get the Defense Points
    fn def(&self) -> T {
        self.stat().def
    }
    /// Get the Mana Attack Points
    fn m_atk(&self) -> T {
        self.stat().m_atk
    }
    /// Get the Mana Defense Points
    fn m_def(&self) -> T {
        self.stat().m_def
    }
    /// Get the Gold Points
    fn gp(&self) -> T {
        self.stat().gp
    }
    /// Get the agility Points
    fn agi(&self) -> T {
        self.stat().agility
    }
    /// Get the strength Points
    fn str(&self) -> T {
        self.stat().strength
    }
    /// Get the intelligence Points
    fn int(&self) -> T {
        self.stat().intelligence
    }
    /// Get the dexterity Points
    fn dex(&self) -> T {
        self.stat().dexterity
    }
    /// Get the constitution Points
    fn con(&self) -> T {
        self.stat().constitution
    }
    /// Get the charisma Points
    fn char(&self) -> T {
        self.stat().charisma
    }
    /// Get the wisdom Points
    fn wis(&self) -> T {
        self.stat().wisdom
    }
    /// Get the Age
    fn age(&self) -> T {
        self.stat().age
    }
    /// Damage the character by an amount
    fn damage(&mut self, amount:T) {
        let mut val = self.hp();
        val -= amount;
        let none = num::cast(0).unwrap();
        if val < none {
            val = none;
        }
        self.set_hp(val)
    }
    /// Add health to character but not beyond their Max Healh Points
    fn heal(&mut self, amount:T) {
        let mut val = self.hp();
        val += amount;
        let max = self.hp_max();
        if val > max {
            val = max;
        }
        self.set_hp(val)
    }
    // TODO taken from `Normal`
    /// Stable attack forumla
    /// [attack*(100/(100+defense))](https://rpg.fandom.com/wiki/Damage_Formula)
    fn attack_stable(&self, other:Advanced<T>) -> T { 
        let hundred = num::cast(100).unwrap();
        let val = self.atk();
        let def = other.def + hundred;
        let res = hundred / def;
        val * res
    }

}
/*
# The Advanced stat model
The entire stat sheet for fine tuned algorithms using all the information possible!
*/
#[allow(unused)]
#[derive( Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Advanced<T:Copy
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
    + num::NumCast> {
    /// Identification Number
    pub id:T,
    /// Experience Points
    pub xp:T,
    /// Health Points
    pub hp:T,
    /// Mana Points
    pub mp:T,
    /// Experience Points multiplier for next level
    pub xp_next:T,
    /// Max Health Points
    pub hp_max:T,
    /// Max Mana Points
    pub mp_max:T,
    /// Current Level
    pub level:T,
    /// The speed
    pub speed:T,
    /// your currency points
    pub gp:T,
    /// Attack
    pub atk:T,
    /// Defense
    pub def:T,
    /// Mana Attack
    pub m_atk:T,
    /// Mana Defense
    pub m_def:T,
    /// The agility Points
    pub agility:T,
    /// The strength Points
    pub strength:T,
    /// The dexterity Points
    pub dexterity:T,
    /// The constitution Points
    pub constitution:T,
    /// The intelligence Points
    pub intelligence:T,
    /// The charisma Points
    pub charisma:T,
    /// The wisdom Points
    pub wisdom:T,
    /// The current age
    pub age:T,
    
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
    + num::NumCast> Advanced<T> {
    /// make empty stats
    #[allow(unused)]
    pub fn empty<U:Default>() -> Self {
        Advanced {
            id:Default::default(),
            xp:Default::default(),
            xp_next:Default::default(),
            mp:Default::default(),
            hp:Default::default(),
            mp_max:Default::default(),
            hp_max:Default::default(),
            level:Default::default(),
            speed:Default::default(),
            gp:Default::default(),
            atk:Default::default(),
            def:Default::default(),
            m_atk:Default::default(),
            m_def:Default::default(),
            agility:Default::default(),
            strength:Default::default(),
            dexterity:Default::default(),
            constitution:Default::default(),
            intelligence:Default::default(),
            charisma:Default::default(),
            wisdom:Default::default(),
            age:Default::default(),
        }
    }
    
    /// People like new
    #[allow(unused)]
    pub fn new<U:Default>() -> Self {
        Self::empty::<U>()
    }
    #[allow(unused)]
    /// Get the next amount of XP needed to level up
    pub fn next(&self) -> T {
        self.level * self.xp_next
    }
    #[allow(unused)]
    /// a vector of stats used to get the standard deviation
    pub fn stats_vec(&self) -> Vec<T>{
        vec![
            self.hp_max,
            self.mp_max,
            self.speed,
            self.atk,
            self.def,
            self.m_atk,
            self.m_def,
            self.agility,
            self.strength,
            self.dexterity,
            self.constitution,
            self.intelligence,
            self.charisma,
        ]
    }

    #[allow(unused)]
    /// This function levels up our stats
    pub fn level_up(&mut self) -> bool {
        if self.xp > self.next() {
            let one:T = num::cast(1).unwrap();
            let value:T = self.wisdom;
            self.level += value;
            self.mp_max += value;
            self.hp_max += value;
            self.speed += value;
            self.atk += value;
            self.def += value;
            self.m_atk += value;
            self.m_def += value;
            self.agility += value;
            self.strength += value;
            self.dexterity += value;
            self.constitution += value;
            self.intelligence += value;
            self.charisma += value;
            self.wisdom += one;
            return true;
        }
        false
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
    + num::NumCast> Default for Advanced<T> {
    /// Default to empty
    fn default() -> Self {
        Self::empty::<T>()
    }
}

/*
# The FLTK Stats
This is designed to be used with FLTK, but can be used without FLTK
*/
#[derive( Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "fltkform", derive(FltkForm))]
pub struct Stats {
    /// Identification Number
    pub id:f64,
    /// Experience Points
    pub xp:f64,
    /// Health Points
    pub hp:f64,
    /// Mana Points
    pub mp:f64,
    /// Experience Points multiplier for next level
    pub xp_next:f64,
    /// Max Health Points
    pub hp_max:f64,
    /// Max Mana Points
    pub mp_max:f64,
    /// Current Level
    pub level:f64,
    /// The speed
    pub speed:f64,
    /// your currency points
    pub gp:f64,
    /// Attack
    pub atk:f64,
    /// Defense
    pub def:f64,
    /// Mana Attack
    pub m_atk:f64,
    /// Mana Defense
    pub m_def:f64,

}
impl Random for Stats {
    type Type = Stats;
    fn random_type(&self) -> Self::Type {
        let hp = self.random(10.0,50.0);
        let atk = self.random(5.0,50.0);
        let def = self.random(5.0,50.0);
        let mp = self.random(10.0,50.0);
        let m_atk = self.random(5.0,50.0);
        let m_def = self.random(5.0,50.0);
        let speed = self.random(5.0,50.0);
        let gp = self.random(0.0, 30.0);
        Stats {
            id:self.random(0.0, 100.0),
            xp:0.0,
            xp_next:10.0,
            level:1.0,
            hp_max:hp,
            hp,
            mp_max:mp,
            mp,
            atk,
            def,
            m_atk,
            m_def,
            speed,
            gp,
            
        }
    }
    
}
impl Stats {
    #[allow(unused)]
    /// read from a TOML file
    pub fn read<P: Clone + AsRef<Path> + std::fmt::Debug>(filename:P) -> Option<Self> {
        if let Ok(file_string) = std::fs::read_to_string(filename.clone()) {
            let decoded:Stats = match toml::from_str(file_string.as_str()) {
                Ok(decoded) => decoded,
                Err(e) => {
                    println!("Stats::read()->toml::from_str() Error:{}\nFilename:{:?}", e, filename);
                    return None
                },
            };
            return Some(decoded);
        }
        None
    }
    #[allow(unused)]
    /// Save a TOML FILE
    pub fn save(&self, path:&str) -> bool {
        Stats::write(*self, path)
    }
    #[allow(unused)]
    /// Write a TOML file
    pub fn write(save:Stats, path:&str) -> bool {
        let toml = match toml::to_string(&save){
            Ok(toml) => toml,
            Err(e) => {
                println!("Stats::save problem:\ntoml::to_string error:{}", e);
                return false;
            },
        };
        let mut output = match File::create(path) {
            Ok(out) => out,
            Err(e) => {
                println!("Stats::save problem:\nFile::create({}) error:{}", path, e);
                return false;
            },
            
        };
        match write!(output, "{}", toml) {
            Ok(_) => (),
            Err(e) => {
                println!("Stats::save problem:\nwrite! error:{}", e);
                return false;
            },
        }
        true
    }
    /// make empty stats
    #[allow(unused)]
    pub fn empty() -> Self {
        Stats {
            id:0.0,
            xp:0.0,
            xp_next:0.0,
            mp:0.0,
            hp:0.0,
            mp_max:0.0,
            hp_max:0.0,
            level:0.0,
            speed:0.0,
            gp:0.0,
            atk:0.0,
            def:0.0,
            m_atk:0.0,
            m_def:0.0,
        }
    }
    /// People like new
    #[allow(unused)]
    pub fn new() -> Self {
        Self::empty()
    }
    #[allow(unused)]
    /// Get the next amount of XP needed to level up
    pub fn next(&self) -> f64 {
        self.level * self.xp_next
    }
    #[allow(unused)]
    /// a vector of stats used to get the standard deviation
    pub fn stats_vec(&self) -> Vec<f64>{
        vec![
            self.hp_max,
            self.mp_max,
            self.speed,
            self.atk,
            self.def,
            self.m_atk,
            self.m_def,
        ]
    }

    #[allow(unused)]
    /// This function levels up our stats
    pub fn level_up(&mut self) {
        println!("xp:{} next:{}", self.xp, self.next());
        if self.xp > self.next() {
            let stats_vec:Vec<f64> = self.stats_vec();
            let mut num:f64 = Math::population_standard_deviation(stats_vec);
            let one = num::cast(1).unwrap();
            if num < one {
                num = one;
            }
            num *= self.level;
            self.level += num;
            self.mp_max += num;
            self.hp_max += num;
            self.speed += num;
            self.atk += num;
            self.def += num;
            self.m_atk += num;
            self.m_def += num;
        }
    }
    /// Damage the character by an amount
    #[allow(unused)]
    pub fn damage(&mut self, amount:f64) {
        let mut val = self.hp;
        val -= amount;
        let none = 0.0;
        if val < none {
            val = none;
        }
        self.hp = val;
    }
    /// Add health to character but not beyond their Max Healh Points
    #[allow(unused)]
    pub fn heal(&mut self, amount:f64) {
        let mut val = self.hp;
        val += amount;
        let max = self.hp_max;
        if val > max {
            val = max;
        }
        self.hp = val;
    }
    /* # Full attack formula
    This attack uses
     * `hp` level
     * `hp_max`
     * `atk`
     * `str`
    It uses the other stats to determine defense and returns the damage done
    */
    #[allow(unused)]
    pub fn attack(&self, other:Stats) -> f64 {
        let max:f64 = self.hp_max;
        let atk:f64 = Math::quarter(self.hp + 
                       rand(Math::half(max), max)
                     ) + self.atk;
        let def:f64 = other.defense();
        let ret:f64 = atk - def;
        let none = 0.0;
        if ret < none {
            return none
        }
        ret
    }
    /// Determine accuracy TODO 
    #[allow(unused)]
    pub fn accuracy(&self) -> bool {
        let entropy:f64 = self.hp_max - self.hp;
        let none:f64 = 0.0;
        let seed:f64 = rand(none, entropy);
        Math::quarter(entropy) <= seed
    }
    /// An equation to simulate a full defense
    /// this relies on current `hp` as well as `def`
    #[allow(unused)]
    pub fn defense(&self) -> f64 {
        Math::quarter(self.hp) + self.def
    }
    /// Buy something, if you have enough money
    #[allow(unused)]
    pub fn buy(&mut self, price:f64) -> bool {
        let total = self.gp - price;
        if total < 0.0 {
            return false;
        }
        self.gp = total;
        true
    }
    /// Earn some money
    #[allow(unused)]
    pub fn earn(&mut self, price:f64) {
        let total = self.gp + price;
        self.gp = total;
    }
}

impl Default for Stats {
    /// Default to empty
    fn default() -> Self {
        Self::empty()
    }
}

/*
# Premade trait for the f64/FLTK Stat
Define the function `stat()` to return the `Stats` associated with your code.
Define the methods to set the stats, and the getter functions already exist


*/
pub trait Premade {
    /// # Function you need to imlement
    /// stat returns the `Stats` you created
    fn stat(&self) -> Stats;
    /// # Function you need to imlement
    /// Set the `Stats` Health Points
    fn set_hp(&mut self, amount:f64);
    /// # Function you need to imlement
    /// Set the `Stats` Mana Points
    fn set_mp(&mut self, amount:f64);
    /// # Function you need to imlement
    /// Set the `Stats` Experience Points
    fn set_xp(&mut self, amount:f64);
    /// # Function you need to imlement
    /// Set the `Stats` Max Health Points
    fn set_hp_max(&mut self, amount:f64);
    /// # Function you need to imlement
    /// Set the `Stats` Max Mana Points
    fn set_mp_max(&mut self, amount:f64);
    /// # Function you need to imlement
    /// Set the `Stats` Next Experience Points
    fn set_xp_next(&mut self, amount:f64);
    /// # Function you need to imlement
    /// Set the `Stats` Gold Points
    fn set_gp(&mut self, amount:f64);
    /// # Function you need to imlement
    /// Set the `Stats` Attack Points
    fn set_atk(&mut self, amount:f64);
    /// # Function you need to imlement
    /// Set the `Stats` Defense Points
    fn set_def(&mut self, amount:f64);
    /// # Function you need to imlement
    /// Set the `Stats` Mana Attack Points
    fn set_m_atk(&mut self, amount:f64);
    /// # Function you need to imlement
    /// Set the `Stats` Mana Defense Points
    fn set_m_def(&mut self, amount:f64);
    /// # Function you need to imlement
    /// Set the `Stats` Level
    fn set_level(&mut self, amount:f64);
    /// # Function you need to imlement
    /// Set the `Stats` Level
    fn set_speed(&mut self, amount:f64);

// PREMADE FUNCf64IONS   
    /// Return  the `Stats` id number
    fn id(&self) -> f64 {
        self.stat().id
    }
    /// Return  the `Stats` Health Points
    fn hp(&self) -> f64 {
        self.stat().hp
    }
    /// Return  the `Stats` Mana Points
    fn mp(&self) -> f64 {
        self.stat().mp
    }
    /// Return  the `Stats` Experience Points
    fn xp(&self) -> f64 {
        self.stat().xp
    }
    /// Return  the `Stats` Max Health Points
    fn hp_max(&self) -> f64 {
        self.stat().hp_max
    }
    /// Return  the `Stats` Max Mana Points
    fn mp_max(&self) -> f64 {
        self.stat().mp_max
    }
    /// Return  the `Stats` Next Experience Points
    fn xp_next(&self) -> f64 {
        self.stat().xp_next
    }
    /// Return  the `Stats` Level
    fn level(&self) -> f64 {
        self.stat().level
    }
    /// Return  the `Stats` Speed
    fn speed(&self) -> f64 {
        self.stat().speed
    }
    /// Return  the `Stats` Gold Points
    fn gp(&self) -> f64 {
        self.stat().gp
    }
    /// Return  the `Stats` Attack Points
    fn atk(&self) -> f64 {
        self.stat().atk
    }
    /// Return  the `Stats` Defense Points
    fn def(&self) -> f64 {
        self.stat().def
    }
    /// Return  the `Stats` Mana Attack Points
    fn m_atk(&self) -> f64 {
        self.stat().m_atk
    }
    /// Return  the `Stats` Mana Defense Points
    fn m_def(&self) -> f64 {
        self.stat().m_def
    }
    /// Return  the `Stats` Attack Points
    fn add_atk(&mut self, amount:f64) {
        self.set_atk(self.stat().atk + amount);
    }
    /// Return  the `Stats` Defense Points
    fn add_def(&mut self, amount:f64) {
        self.set_def(self.stat().def + amount);
    }
    /// Return  the `Stats` Mana Attack Points
    fn add_m_atk(&mut self, amount:f64) {
        self.set_m_atk(self.stat().m_atk + amount);
    }
    /// Return  the `Stats` Mana Defense Points
    fn add_m_def(&mut self, amount:f64) {
        self.set_m_def(self.stat().m_def + amount);
    }/// Return  the `Stats` Experience Points
    fn add_xp(&mut self, amount:f64) {
        self.set_xp(self.stat().xp + amount);
    }
    /// Return  the `Stats` Max Health Points
    fn add_hp_max(&mut self, amount:f64) {
        self.set_hp_max(self.stat().hp_max + amount);
    }
    /// Return  the `Stats` Max Mana Points
    fn add_mp_max(&mut self, amount:f64) {
       self.set_mp_max( self.stat().mp_max + amount);
    }
    /// Return  the `Stats` Level
    fn add_level(&mut self, amount:f64) {
        self.set_level(self.stat().level + amount);
    }
    /// Return  the `Stats` Speed
    fn add_speed(&mut self, amount:f64) {
        self.set_speed(self.stat().speed + amount);
    }
    /// Damage the character by an amount
    fn damage(&mut self, amount:f64) {
        let mut val = self.hp();
        val -= amount;
        let none = 0.0;
        if val < none {
            val = none;
        }
        self.set_hp(val)
    }
    /// Add health to character but not beyond their Max Healh Points
    fn heal(&mut self, amount:f64) {
        let mut val = self.hp();
        val += amount;
        let max = self.hp_max();
        if val > max {
            val = max;
        }
        self.set_hp(val)
    }
    /* # Full attack formula
    This attack uses
     * `hp` level
     * `hp_max`
     * `atk`
     * `str`
    It uses the other stats to determine defense and returns the damage done
    */
    fn attack(&self, other:Stats) -> f64 {
        self.stat().attack(other)
    }
    /// 
    fn accuracy(&self) -> bool {
        self.stat().accuracy()
    }
    /// An equation to simulate a full defense
    /// this relies on current `hp` as well as `def`
    fn defense(&self) -> f64 {
        self.stat().defense()
    }
    fn buy(&mut self, price:f64) -> bool {
        let total = self.gp() - price;
        if total <= 0.0 {
            return false;
        }
        self.set_gp(total);
        true
    }
    fn earn(&mut self, price:f64) {
        let total = self.gp() + price;
        self.set_gp(total);
    }    #[allow(unused)]
    /// Get the next amount of XP needed to level up
    fn next(&self) -> f64 {
        self.stat().next()
    }
    /// a vector of stats used to get the standard deviation
    fn stats_vec(&self) -> Vec<f64>{
        self.stat().stats_vec()
    }

    #[allow(unused)]
    fn level_up(&mut self) -> bool{
        if self.xp() > self.next() {
            let stats_vec:Vec<f64> = self.stats_vec();
            let mut num:f64 = Math::population_standard_deviation(stats_vec);
            let one = 1.0;
            if num < one {
                num = one;
            }
            self.add_level(num);
            self.add_mp_max(num);
            self.add_hp_max(num);
            self.add_speed(num);
            self.add_atk(num);
            self.add_def(num);
            self.add_m_atk(num);
            self.add_m_def(num);
            return true;
        }
        false
    }
}
