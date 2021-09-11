/*!
# Stats

This contains the basic structures for the entire statistics library
*/
/*
# The Basic HP/MP/XP stat model

This basic model of stats is easy to work with for beginners, but powerful enough to be used by the most experienced.
*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Basic {
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
}
impl Basic {
    /// make empty stats
    pub fn empty() -> Self where Self:Sized {
        Basic {
            xp:0.0,
            xp_next:0.0,
            mp:0.0,
            hp:0.0,
            mp_max:0.0,
            hp_max:0.0,
            level:0.0,
            speed:0.0,
            gp:0.0,
        }
    }
}
impl Default for Basic {
    fn default() -> Self {
        Self::empty()
    }
}

/*
# The Normal

This model provides fine tuning of attack and defense without needing all the fine tuning of a full stat sheet
*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Normal {
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
impl Normal {
    /// make empty stats
    pub fn empty() -> Self where Self:Sized {
        Normal {
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
impl Default for Normal {
    fn default() -> Self {
        Self::empty()
    }
}

/*
# The Advanced stat model
The entire stat sheet for fine tuned algorithms using all the information possible!
*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Advanced {
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
    /// The agility Points
    pub agility:f64,
    /// The strength Points
    pub strength:f64,
    /// The dexterity Points
    pub dexterity:f64,
    /// The constitution Points
    pub constitution:f64,
    /// The intelligence Points
    pub intelligence:f64,
    /// The charisma Points
    pub charisma:f64,
    /// The wisdom Points
    pub wisdom:f64,
    /// The current age
    pub age:f64,
    
}
impl Advanced {
    /// make empty stats
    pub fn empty() -> Self where Self:Sized {
        Advanced {
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
            agility:0.0,
            strength:0.0,
            dexterity:0.0,
            constitution:0.0,
            intelligence:0.0,
            charisma:0.0,
            wisdom:0.0,
            age:0.0,
        }
    }
}
impl Default for Advanced {
    fn default() -> Self {
        Self::empty()
    }
}
