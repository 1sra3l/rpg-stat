/*!
# Stats

This contains the basic structures for the entire statistics library
*/
use std::default::Default;
use serde::{Deserialize, Serialize};
extern crate num;
use num::NumCast;
use crate::class::Normal as ClassNormal;
use crate::class::Basic as ClassBasic;
use crate::creature::Animal;
use crate::creature::Person;
use std::ops::{Add, AddAssign,  Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign};

/*
# Builder

The builder trait is how I create `rpgstat::stats::{Basic,Normal,Advance}` from enums

*/
pub trait Builder<T:Copy 
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
    pub fn from_animal(id:T, animal:Animal, level:T)  -> Basic<T> {
        let mut hp = num::cast(10).unwrap();
        let mut mp = num::cast(5).unwrap();
        let mut xp = num::cast(1).unwrap();
        let mut gp = num::cast(5).unwrap();
        let mut speed = num::cast(5).unwrap();
        match animal {
            Animal::Rat => {
                gp += num::cast(2).unwrap();
            }
            Animal::Snake => {
                speed += num::cast(2).unwrap();
                mp += num::cast(1).unwrap();
                hp += num::cast(1).unwrap();
                xp += num::cast(1).unwrap();
            }
            Animal::Rabbit => {
                speed += num::cast(2).unwrap();
                hp += num::cast(2).unwrap();
                mp += num::cast(2).unwrap();
                xp += num::cast(2).unwrap();
            }
            Animal::Wolf => {
                speed += num::cast(3).unwrap();
                hp += num::cast(3).unwrap();
                mp += num::cast(3).unwrap();
                xp += num::cast(3).unwrap();
            }
            Animal::Crocodile => {
                speed -= num::cast(1).unwrap();
                hp += num::cast(5).unwrap();
                mp += num::cast(5).unwrap();
                xp += num::cast(5).unwrap();
            }
            Animal::Insect => {
                hp -= num::cast(2).unwrap();
                mp += num::cast(4).unwrap();
            }
            Animal::Lion => {
                hp += num::cast(10).unwrap();
                speed += num::cast(7).unwrap();
                mp += num::cast(4).unwrap();
                xp += num::cast(9).unwrap();
            }
            Animal::Tiger => {
                hp += num::cast(9).unwrap();
                speed += num::cast(9).unwrap();
                mp += num::cast(7).unwrap();
                xp += num::cast(7).unwrap();
            }
            Animal::Bear => {
                hp += num::cast(15).unwrap();
                mp += num::cast(10).unwrap();
                xp += num::cast(10).unwrap();
            }
            Animal::Bird => {
                hp += num::cast(3).unwrap();
                mp += num::cast(12).unwrap();
                xp += num::cast(7).unwrap();
            }
        }
        hp *= level;
        mp *= level;
        xp *= level;
        gp *= level;
        speed += level;
        Basic {
            id:id,
            xp:xp,
            xp_next:Default::default(),
            level:level,
            gp:gp,
            hp: hp,
            mp: mp,
            hp_max: hp,
            mp_max: mp,
            speed: speed,
        }
    }
    pub fn from_class(id:T, class:ClassBasic) -> Basic<T> {
        match class {
            ClassBasic::Hero => Basic {
                id:id,
                xp:Default::default(),
                xp_next:num::cast(10).unwrap(),
                level:Default::default(),
                gp:Default::default(),
                hp: num::cast(10).unwrap(),
                mp: num::cast(5).unwrap(),
                hp_max: num::cast(10).unwrap(),
                mp_max: num::cast(5).unwrap(),
                speed: num::cast(10).unwrap(),
                
            },
            ClassBasic::Enemy => Basic {
                id:id,
                xp:num::cast(1).unwrap(),
                xp_next:num::cast(10).unwrap(),
                level:Default::default(),
                gp:Default::default(),
                hp: num::cast(5).unwrap(),
                mp: num::cast(5).unwrap(),
                hp_max: num::cast(5).unwrap(),
                mp_max: num::cast(5).unwrap(),
                speed: num::cast(7).unwrap(),
            },
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
        let mut def = other.def;
        def += val;
        res = res / def;
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
    ///
    pub fn change_health(&mut self, amount:T) {
        self.hp -= amount;
        let none = num::cast(0).unwrap();
        if self.hp < none {
            self.hp = none;
        }
    }
    
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
    /// Make a `Normal` Stat from an `Animal`
    pub fn from_animal(id:T, animal:Animal, level:T)  -> Normal<T> {
        let mut hp = num::cast(10).unwrap();
        let mut mp = num::cast(5).unwrap();
        let mut xp = num::cast(1).unwrap();
        let mut gp = num::cast(5).unwrap();
        let mut speed = num::cast(5).unwrap();
        let mut atk = num::cast(10).unwrap();
        let mut def = num::cast(5).unwrap();
        let mut m_atk = num::cast(5).unwrap();
        let mut m_def = num::cast(2).unwrap();
        match animal {
            Animal::Rat => {
                gp += num::cast(2).unwrap();
            }
            Animal::Snake => {
                speed += num::cast(2).unwrap();
                mp += num::cast(1).unwrap();
                hp += num::cast(1).unwrap();
                xp += num::cast(1).unwrap();
                gp += num::cast(1).unwrap();
                atk += num::cast(1).unwrap();
                def += num::cast(1).unwrap();
                m_atk += num::cast(1).unwrap();
                m_def += num::cast(1).unwrap();
            }
            Animal::Rabbit => {
                speed += num::cast(2).unwrap();
                hp += num::cast(2).unwrap();
                mp += num::cast(2).unwrap();
                xp += num::cast(2).unwrap();
                atk += num::cast(2).unwrap();
                def += num::cast(2).unwrap();
                m_atk += num::cast(2).unwrap();
                m_def += num::cast(7).unwrap();
            }
            Animal::Wolf => {
                speed += num::cast(3).unwrap();
                hp += num::cast(3).unwrap();
                mp += num::cast(3).unwrap();
                xp += num::cast(3).unwrap();
                gp += num::cast(2).unwrap();
                atk += num::cast(10).unwrap();
                def += num::cast(5).unwrap();
                m_atk += num::cast(3).unwrap();
                m_def += num::cast(5).unwrap();
            }
            Animal::Crocodile => {
                speed -= num::cast(1).unwrap();
                hp += num::cast(5).unwrap();
                mp += num::cast(5).unwrap();
                xp += num::cast(5).unwrap();
                gp += num::cast(4).unwrap();
                atk += num::cast(7).unwrap();
                def += num::cast(7).unwrap();
                m_atk += num::cast(3).unwrap();
                m_def += num::cast(1).unwrap();
            }
            Animal::Insect => {
                hp -= num::cast(2).unwrap();
                mp += num::cast(4).unwrap();
                atk += num::cast(2).unwrap();
                def += num::cast(3).unwrap();
                m_atk += num::cast(1).unwrap();
                m_def += num::cast(1).unwrap();
            }
            Animal::Lion => {
                hp += num::cast(10).unwrap();
                speed += num::cast(7).unwrap();
                mp += num::cast(4).unwrap();
                xp += num::cast(9).unwrap();
                atk += num::cast(11).unwrap();
                def += num::cast(9).unwrap();
                m_atk += num::cast(5).unwrap();
                m_def += num::cast(8).unwrap();
            }
            Animal::Tiger => {
                hp += num::cast(9).unwrap();
                speed += num::cast(9).unwrap();
                mp += num::cast(7).unwrap();
                xp += num::cast(7).unwrap();
                atk += num::cast(12).unwrap();
                def += num::cast(5).unwrap();
                m_atk += num::cast(5).unwrap();
                m_def += num::cast(12).unwrap();
            }
            Animal::Bear => {
                hp += num::cast(15).unwrap();
                mp += num::cast(10).unwrap();
                xp += num::cast(10).unwrap();
                atk += num::cast(18).unwrap();
                def += num::cast(12).unwrap();
                m_atk += num::cast(2).unwrap();
                m_def += num::cast(5).unwrap();
            }
            Animal::Bird => {
                hp += num::cast(2).unwrap();
                mp += num::cast(12).unwrap();
                xp += num::cast(7).unwrap();
                atk += num::cast(11).unwrap();
                def += num::cast(1).unwrap();
                m_atk += num::cast(8).unwrap();
                m_def += num::cast(10).unwrap();
            }
            
        }
        hp *= level;
        mp *= level;
        xp *= level;
        gp *= level;
        speed += level;
        atk *= level;
        def *= level;
        m_atk *= level;
        m_def *= level;
        Normal {
            id:id,
            xp:xp,
            xp_next:Default::default(),
            level:level,
            gp:gp,
            hp: hp,
            mp: mp,
            hp_max: hp,
            mp_max: mp,
            speed: speed,
            atk: atk,
            def: def,
            m_atk: m_atk,
            m_def: m_def,
        }
    }
    /// Create a `Normal` Stat from a `Normal` Class
    pub fn from_class(id:T, class: &ClassNormal) -> Normal<T> {
        match class {
            ClassNormal::Alchemist => Normal {
                //name: "Alchemist",
                id:id,
                hp: num::cast(40).unwrap(),
                mp: num::cast(16).unwrap(),
                hp_max: num::cast(40).unwrap(),
                mp_max: num::cast(16).unwrap(),
                atk: num::cast(14).unwrap(),
                def: num::cast(30).unwrap(),
                m_atk: num::cast(20).unwrap(),
                m_def: num::cast(30).unwrap(),
                xp:Default::default(),
                xp_next:Default::default(),
                gp:Default::default(),
                level:Default::default(),
                speed: num::cast(5).unwrap(),
            },
            ClassNormal::Archer => Normal {
                //name: "Archer",
                id:id,
                hp_max: num::cast(50).unwrap(),
                mp_max: num::cast(25).unwrap(),
                hp: num::cast(50).unwrap(),
                mp: num::cast(25).unwrap(),
                atk: num::cast(15).unwrap(),
                def: num::cast(10).unwrap(),
                m_atk: num::cast(15).unwrap(),
                m_def: num::cast(35).unwrap(),
                xp:Default::default(),
                xp_next:Default::default(),
                gp:Default::default(),
                level:Default::default(),
                speed: num::cast(5).unwrap(),
            },
            ClassNormal::Knight => Normal {
                //name: "Knight",
                id:id,
                hp: num::cast(50).unwrap(),
                mp: num::cast(20).unwrap(),
                hp_max: num::cast(50).unwrap(),
                mp_max: num::cast(20).unwrap(),
                atk: num::cast(20).unwrap(),
                def: num::cast(20).unwrap(),
                m_atk: num::cast(20).unwrap(),
                m_def: num::cast(20).unwrap(),
                xp:Default::default(),
                xp_next:Default::default(),
                gp:Default::default(),
                level:Default::default(),
                speed: num::cast(7).unwrap(),
            },
            ClassNormal::Monk => Normal {
                //name: "Monk",
                id:id,
                hp: num::cast(50).unwrap(),
                mp: num::cast(20).unwrap(),
                hp_max: num::cast(50).unwrap(),
                mp_max: num::cast(20).unwrap(),
                atk: num::cast(10).unwrap(),
                def: num::cast(15).unwrap(),
                m_atk: num::cast(5).unwrap(),
                m_def: num::cast(40).unwrap(),
                xp:Default::default(),
                xp_next:Default::default(),
                gp:Default::default(),
                level:Default::default(),
                speed: num::cast(5).unwrap(),
            },
            ClassNormal::Elemental => Normal {
                //name: "Elemental",
                id:id,
                hp: num::cast(70).unwrap(),
                mp: num::cast(40).unwrap(),
                hp_max: num::cast(70).unwrap(),
                mp_max:num::cast( 40).unwrap(),
                atk: num::cast(1).unwrap(),
                def: num::cast(8).unwrap(),
                m_atk: num::cast(30).unwrap(),
                m_def: num::cast(1).unwrap(),
                xp:Default::default(),
                xp_next:Default::default(),
                gp:Default::default(),
                level:Default::default(),
                speed: num::cast(4).unwrap(),
            },
            ClassNormal::Priest => Normal {
                //name: "Priest",
                id:id,
                hp: num::cast(60).unwrap(),
                mp: num::cast(10).unwrap(),
                hp_max: num::cast(60).unwrap(),
                mp_max: num::cast(10).unwrap(),
                atk: num::cast(20).unwrap(),
                def:num::cast( 10).unwrap(),
                m_atk: num::cast(10).unwrap(),
                m_def: num::cast(40).unwrap(),
                xp:Default::default(),
                xp_next:Default::default(),
                gp:Default::default(),
                level:Default::default(),
                speed: num::cast(4).unwrap(),
            },
            ClassNormal::Soldier => Normal {
                //name: "Soldier",
                id:id,
                hp: num::cast(90).unwrap(),
                mp: num::cast(0).unwrap(),
                hp_max: num::cast(90).unwrap(),
                mp_max: num::cast(0).unwrap(),
                atk: num::cast(30).unwrap(),
                def: num::cast(12).unwrap(),
                m_atk: num::cast(0).unwrap(),
                m_def: num::cast(18).unwrap(),
                xp:Default::default(),
                xp_next:Default::default(),
                gp:Default::default(),
                level:Default::default(),
                speed: num::cast(5).unwrap(),
            },
            ClassNormal::Ranger => Normal {
                //name: "Ranger",
                id:id,
                hp: num::cast(40).unwrap(),
                mp: num::cast(70).unwrap(),
                hp_max: num::cast(40).unwrap(),
                mp_max: num::cast(70).unwrap(),
                atk: num::cast(15).unwrap(),
                def:num::cast( 9).unwrap(),
                m_atk: num::cast(11).unwrap(),
                m_def: num::cast(30).unwrap(),
                xp:Default::default(),
                xp_next:Default::default(),
                gp:Default::default(),
                level:Default::default(),
                speed: num::cast(8).unwrap(),
            },
            ClassNormal::Valkyrie => Normal {
                //name: "Valkyrie",
                id:id,
                hp: num::cast(50).unwrap(),
                mp: num::cast(10).unwrap(),
                hp_max: num::cast(50).unwrap(),
                mp_max: num::cast(10).unwrap(),
                atk: num::cast(20).unwrap(),
                def: num::cast(20).unwrap(),
                m_atk: num::cast(20).unwrap(),
                m_def: num::cast(30).unwrap(),
                xp:Default::default(),
                xp_next:Default::default(),
                gp:Default::default(),
                level:Default::default(),
                speed: num::cast(7).unwrap(),
            },
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
        res = res / def;
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

