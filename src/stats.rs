/*!
# Stats

This contains the basic structures for the entire statistics library
*/
/*
# The Basic HP/MP/XP stat model

This basic model of stats is easy to work with for beginners, but powerful enough to be used by the most experienced.
*/
use std::default::Default;
/*
Trait for Basic Stat
*/
pub trait BasicPremade<T:Default + std::ops::SubAssign> {
    fn stat(&self) -> Basic<T>;
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
    fn damage(&mut self, amount:T) -> T {
        let mut val = self.hp();
        val -= amount;
        val
    }
}
/*
Trait for Basic Stat
*/
pub trait BasicStat<T> {
    fn hp(&self) -> T;
    fn mp(&self) -> T;
    fn xp(&self) -> T;
    fn hp_max(&self) -> T;
    fn mp_max(&self) -> T;
    fn xp_next(&self) -> T;
    fn damage(&mut self, amount:T);
}
/// Basic Stat model hp/mp/xp/level for mechanics
#[derive( Debug, Clone, Copy, PartialEq)]
pub struct Basic<T:Default> {
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
impl<T:Default> Basic<T> {
    /// make empty stats
    pub fn empty() -> Self where Self:Sized {
        Basic {
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
impl<T:Default> Default for Basic<T> {
    /// Default to empty
    fn default() -> Self where Self:Sized {
        Self::empty()
    }
}

/*
# The Normal

This model provides fine tuning of attack and defense without needing all the fine tuning of a full stat sheet
*/
#[derive( Debug, Clone, Copy, PartialEq)]
pub struct Normal<T> {
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
impl<T:Default> Normal<T> {
    /// make empty stats
    pub fn empty<U:Default>() -> Self {
        Normal {
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
impl<T:Default> Default for Normal<T> {
    /// Default to empty
    fn default() -> Self {
        Self::empty::<T>()
    }
}

/*
# The Advanced stat model
The entire stat sheet for fine tuned algorithms using all the information possible!
*/
#[derive( Debug, Clone, Copy, PartialEq)]
pub struct Advanced<T> {
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
impl<T:Default> Advanced<T> {
    /// make empty stats
    pub fn empty<U:Default>() -> Self {
        Advanced {
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
impl<T:Default> Default for Advanced<T> {
    /// Default to empty
    fn default() -> Self {
        Self::empty::<T>()
    }
}

pub trait StatTrait<C> {
    fn from_class (&self, class:C) -> Self;
}
