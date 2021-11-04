/*!
# Stats

This contains the basic structures for the entire statistics library
*/
use std::default::Default;
extern crate num;
use num::NumCast;
use crate::class::Normal as ClassNormal;
use crate::class::Basic as ClassBasic;
/*
Premade trait for Basic Stat
You simply define the function `stat()` to return the `Basic<T>` associated with your code.
*/
pub trait BasicPremade<T:Default
                        + std::ops::AddAssign
                        + std::ops::Add
                        + std::ops::Sub
                        + std::ops::Div
                        + std::ops::DivAssign
                        + std::ops::Mul
                        + std::ops::MulAssign
                        + std::ops::Neg
                        + std::ops::Rem
                        + std::ops::RemAssign
                        + std::ops::Sub
                        + std::ops::SubAssign
                        + num::NumCast> {
    fn stat(&self) -> Basic<T>;
    fn id(&self) -> T {
        self.stat().id
    }
    fn hp(&self) -> T {
        self.stat().hp
    }
    fn mp(&self) -> T {
        self.stat().mp
    }
    fn xp(&self) -> T {
        self.stat().xp
    }
    fn hp_max(&self) -> T {
        self.stat().hp_max
    }
    fn mp_max(&self) -> T {
        self.stat().mp_max
    }
    fn xp_next(&self) -> T {
        self.stat().xp_next
    }
    fn level(&self) -> T {
        self.stat().level
    }
    fn speed(&self) -> T {
        self.stat().level
    }
    fn gp(&self) -> T {
        self.stat().gp
    }
    fn damage(&mut self, amount:T) -> T {
        let mut val = self.hp();
        val -= amount;
        val
    }
    fn heal(&mut self, amount:T) -> T {
        let mut val = self.hp();
        val += amount;
        val
    }
}
/*
Generic trait so you can build anything you want
*/
pub trait BasicStat<T> {
    fn hp(&self) -> T;
    fn mp(&self) -> T;
    fn xp(&self) -> T;
    fn hp_max(&self) -> T;
    fn mp_max(&self) -> T;
    fn xp_next(&self) -> T;
    fn level(&self) -> T;
    fn speed(&self) -> T;
    fn gp(&self) -> T;
    fn damage(&mut self, amount:T);
    fn heal(&mut self, amount:T);
}
/*
# The Basic HP/MP/XP stat model

This basic model of stats is easy to work with for beginners, but powerful enough to be used by the most experienced.
*/
#[derive( Debug, Clone, Copy, PartialEq)]
pub struct Basic<T:Default
                + std::ops::AddAssign
                + std::ops::Add
                + std::ops::Sub
                + std::ops::Div
                + std::ops::DivAssign
                + std::ops::Mul
                + std::ops::MulAssign
                + std::ops::Neg
                + std::ops::Rem
                + std::ops::RemAssign
                + std::ops::Sub
                + std::ops::SubAssign
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
impl<T:Default
   + std::ops::AddAssign
   + std::ops::Add
   + std::ops::Sub
   + std::ops::Div
   + std::ops::DivAssign
   + std::ops::Mul
   + std::ops::MulAssign
   + std::ops::Neg
   + std::ops::Rem
   + std::ops::RemAssign
   + std::ops::Sub
   + std::ops::SubAssign
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
impl<T:Default
   + std::ops::AddAssign
   + std::ops::Add
   + std::ops::Sub
   + std::ops::Div
   + std::ops::DivAssign
   + std::ops::Mul
   + std::ops::MulAssign
   + std::ops::Neg
   + std::ops::Rem
   + std::ops::RemAssign
   + std::ops::Sub
   + std::ops::SubAssign
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
pub trait NormalPremade<T:Default
                        + std::ops::AddAssign
                        + std::ops::Add
                        + std::ops::Sub
                        + std::ops::Div
                        + std::ops::DivAssign
                        + std::ops::Mul
                        + std::ops::MulAssign
                        + std::ops::Neg
                        + std::ops::Rem
                        + std::ops::RemAssign
                        + std::ops::Sub
                        + std::ops::SubAssign
                        + num::NumCast> {
    fn stat(&self) -> Normal<T>;
    fn id(&self) -> T {
        self.stat().id
    }
    fn hp(&self) -> T {
        self.stat().hp
    }
    fn mp(&self) -> T {
        self.stat().mp
    }
    fn xp(&self) -> T {
        self.stat().xp
    }
    fn hp_max(&self) -> T {
        self.stat().hp_max
    }
    fn mp_max(&self) -> T {
        self.stat().mp_max
    }
    fn xp_next(&self) -> T {
        self.stat().xp_next
    }
    fn level(&self) -> T {
        self.stat().level
    }
    fn speed(&self) -> T {
        self.stat().level
    }
    fn atk(&self) -> T {
        self.stat().atk
    }
    fn def(&self) -> T {
        self.stat().def
    }
    fn m_atk(&self) -> T {
        self.stat().m_atk
    }
    fn m_def(&self) -> T {
        self.stat().m_def
    }
    fn gp(&self) -> T {
        self.stat().gp
    }
    fn damage(&mut self, amount:T) -> T {
        let mut val = self.hp();
        val -= amount;
        val
    }
    fn heal(&mut self, amount:T) -> T {
        let mut val = self.hp();
        val += amount;
        val
    }
}
/*
# The Normal

This model provides fine tuning of attack and defense without needing all the fine tuning of a full stat sheet
*/
#[derive( Debug, Clone, Copy, PartialEq)]
pub struct Normal<T:Default
                + std::ops::AddAssign
                + std::ops::Add
                + std::ops::Sub
                + std::ops::Div
                + std::ops::DivAssign
                + std::ops::Mul
                + std::ops::MulAssign
                + std::ops::Neg
                + std::ops::Rem
                + std::ops::RemAssign
                + std::ops::Sub
                + std::ops::SubAssign
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
impl<T:Default
   + std::ops::AddAssign
   + std::ops::Add
   + std::ops::Sub
   + std::ops::Div
   + std::ops::DivAssign
   + std::ops::Mul
   + std::ops::MulAssign
   + std::ops::Neg
   + std::ops::Rem
   + std::ops::RemAssign
   + std::ops::Sub
   + std::ops::SubAssign
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

impl<T:Default
   + std::ops::AddAssign
   + std::ops::Add
   + std::ops::Sub
   + std::ops::Div
   + std::ops::DivAssign
   + std::ops::Mul
   + std::ops::MulAssign
   + std::ops::Neg
   + std::ops::Rem
   + std::ops::RemAssign
   + std::ops::Sub
   + std::ops::SubAssign
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
pub trait AdvancedPremade<T:Default
                        + std::ops::AddAssign
                        + std::ops::Add
                        + std::ops::Sub
                        + std::ops::Div
                        + std::ops::DivAssign
                        + std::ops::Mul
                        + std::ops::MulAssign
                        + std::ops::Neg
                        + std::ops::Rem
                        + std::ops::RemAssign
                        + std::ops::Sub
                        + std::ops::SubAssign
                        + num::NumCast> {
    fn stat(&self) -> Advanced<T>;
    fn id(&self) -> T {
        self.stat().id
    }
    fn hp(&self) -> T {
        self.stat().hp
    }
    fn mp(&self) -> T {
        self.stat().mp
    }
    fn xp(&self) -> T {
        self.stat().xp
    }
    fn hp_max(&self) -> T {
        self.stat().hp_max
    }
    fn mp_max(&self) -> T {
        self.stat().mp_max
    }
    fn xp_next(&self) -> T {
        self.stat().xp_next
    }
    fn level(&self) -> T {
        self.stat().level
    }
    fn speed(&self) -> T {
        self.stat().level
    }
    fn atk(&self) -> T {
        self.stat().atk
    }
    fn def(&self) -> T {
        self.stat().def
    }
    fn m_atk(&self) -> T {
        self.stat().m_atk
    }
    fn m_def(&self) -> T {
        self.stat().m_def
    }
    fn gp(&self) -> T {
        self.stat().gp
    }
    fn agi(&self) -> T {
        self.stat().agility
    }
    fn str(&self) -> T {
        self.stat().strength
    }
    fn dex(&self) -> T {
        self.stat().dexterity
    }
    fn con(&self) -> T {
        self.stat().constitution
    }
    fn char(&self) -> T {
        self.stat().charisma
    }
    fn wis(&self) -> T {
        self.stat().wisdom
    }
    fn age(&self) -> T {
        self.stat().age
    }
    fn damage(&mut self, amount:T) -> T {
        let mut val = self.hp();
        val -= amount;
        val
    }
    fn heal(&mut self, amount:T) -> T {
        let mut val = self.hp();
        val += amount;
        val
    }
}
/*
# The Advanced stat model
The entire stat sheet for fine tuned algorithms using all the information possible!
*/
#[derive( Debug, Clone, Copy, PartialEq)]
pub struct Advanced<T:Default
                + std::ops::AddAssign
                + std::ops::Add
                + std::ops::Sub
                + std::ops::Div
                + std::ops::DivAssign
                + std::ops::Mul
                + std::ops::MulAssign
                + std::ops::Neg
                + std::ops::Rem
                + std::ops::RemAssign
                + std::ops::Sub
                + std::ops::SubAssign
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
impl<T:Default
   + std::ops::AddAssign
   + std::ops::Add
   + std::ops::Sub
   + std::ops::Div
   + std::ops::DivAssign
   + std::ops::Mul
   + std::ops::MulAssign
   + std::ops::Neg
   + std::ops::Rem
   + std::ops::RemAssign
   + std::ops::Sub
   + std::ops::SubAssign
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
impl<T:Default
   + std::ops::AddAssign
   + std::ops::Add
   + std::ops::Sub
   + std::ops::Div
   + std::ops::DivAssign
   + std::ops::Mul
   + std::ops::MulAssign
   + std::ops::Neg
   + std::ops::Rem
   + std::ops::RemAssign
   + std::ops::Sub
   + std::ops::SubAssign
   + num::NumCast> Default for Advanced<T> {
    /// Default to empty
    fn default() -> Self {
        Self::empty::<T>()
    }
}

