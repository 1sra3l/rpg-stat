/*!
# Stats

This contains the basic structures for the statistics library

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
 * Age can influence story mechanics but this would be a choice

 */
use std::default::Default;
use serde::{Deserialize, Serialize};
extern crate num;
//use num::NumCast;
use std::ops::{Add, AddAssign,  Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign};

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

/*
# Builder

The builder trait is how I create `rpgstat::stats::{Basic,Normal,Advance}` from enums

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
        let hundred = num::cast(100).unwrap();
        let val = self.atk();
        let def = other.def + hundred;
        let res = hundred / def;
        val * res
    }
    /// Scalable attack forumla
    /// [damage = att * att / (att + def)](https://gamedev.stackexchange.com/questions/129319/rpg-formula-attack-and-defense)
    fn attack(&self, other:Normal<T>) -> T {
        let val = self.atk();
        let mut res = val * val;
        let def = other.def + val;
        res /= def;
        res
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
    /// Scalable attack forumla
    /// [damage = att * att / (att + def)](https://gamedev.stackexchange.com/questions/129319/rpg-formula-attack-and-defense)
    fn attack(&self, other:Advanced<T>) -> T {
        let val = self.atk();
        let mut res = val * val;
        let mut def = other.def;
        def += val;
        res /= def;
        res
    }
}
/*
# The Advanced stat model
The entire stat sheet for fine tuned algorithms using all the information possible!
*/
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
# The FLTK

This model provides fine tuning of attack and defense without needing all the fine tuning of a full stat sheet
*/
#[derive( Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "fltkform", derive(FltkForm))]
pub struct Stats {
    /// Identification Number
    pub id:f64,
    // Name
    pub name:String,
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
impl Stats {
    
    /// make empty stats
    pub fn empty() -> Self {
        Stats {
            name: String::from("Ferris"),
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
}

impl Default for Stats {
    /// Default to empty
    fn default() -> Self {
        Self::empty()
    }
}
