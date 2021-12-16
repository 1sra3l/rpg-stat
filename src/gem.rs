/*!
# Gems


*/
use std::fmt;
use rpg_stat::random::Random;
use rpg_stat::stats::Stats;
use rpg_stat::stats::Builder;
use serde::{Deserialize, Serialize};

#[cfg(feature = "fltkform")]
use fltk::{prelude::*, *};
#[cfg(feature = "fltkform")]
use fltk_form_derive::*;
#[cfg(feature = "fltkform")]
use fltk_form::FltkForm;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[cfg_attr(feature = "fltkform", derive(FltkForm))]
/*# Advanced
The rarest gems, according to some place on the internet one time....
*/
pub enum Advanced {
    /// Painite - Borate, Calcium, Zirconium, Boron, Aluminium  mineral
    Painite,
    /// Hibonite - Calcium, Aluminium, Magnesium, Cerium, Titanium, 
    Hibonite,
    /// RedBeryl - Beryllium, Aluminium, Silicon
    RedBeryl,
    /// Jeremejevite - Aluminium, Boron
    Jeremejevite,
    /// Chambersite - Manganese, Boron
    Chambersite,
    /// Musgravite - 
    Musgravite,
    /// Grandidirite - 
    Grandidirite,
    /// Poudretteite - 
    Poudretteite,
    /// Serendibite - 
    Serendibite,
    /// Zektzerite - 
    Zektzerite,
    /// Nothing
    None,
}
impl Advanced {
    pub fn composition(&self) -> Vec<Periodic> {
        match *self {
           Advanced::Painite =>       vec![Periodic::Borate, Periodic::Calcium, Periodic::Zirconium, Periodic::Boron, Periodic::Aluminium],
           Advanced::Hibonite =>      vec![Periodic::Calcium, Periodic::Aluminium, Periodic::Magnesium, Periodic::Cerium, Periodic::Titanium],
           Advanced::RedBeryl =>      vec![Periodic::Beryllium, Periodic::Aluminium, Periodic::Silicon],
           Advanced::Jeremejevite =>  vec![Periodic::Aluminium, Periodic::Boron],
           Advanced::Chambersite =>   vec![Periodic::Manganese, Periodic::Boron],
           Advanced::Musgravite =>    vec![Periodic::],
           Advanced::Grandidirite =>  vec![Periodic::],
           Advanced::Poudretteite =>  vec![Periodic::],
           Advanced::Serendibite =>   vec![Periodic::],
           Advanced::Zektzerite =>    vec![Periodic::],
           _=> vec![],
        }
    }
    /// A list of all items (except Advanced::None)
    pub fn gems() -> Vec<Advanced> {
        vec![
            Advanced::Painite,
            Advanced::Hibonite,
            Advanced::RedBeryl,
            Advanced::Jeremejevite,
            Advanced::Chambersite,
            Advanced::Musgravite,
            Advanced::Grandidirite,
            Advanced::Poudretteite,
            Advanced::Serendibite,
            Advanced::Zektzerite,
            ]
    }
    /// Get an image filename: "assets/gems/"`self.to_string()`".svg"
    pub fn image_filename(&self) -> String {
        let mut result = String::from("assets/gems/");
        result.push_str(self.to_string().as_str());
        result.push_str(".svg");
        result
    }
    pub fn get_price(gem:Advanced) -> f64 {
        gem.price()
    }
    pub fn price(&self) -> f64 {
        match *self {
           Advanced::Painite =>       400.0,
           Advanced::Hibonite =>      420.0,
           Advanced::RedBeryl =>      530.0,
           Advanced::Jeremejevite =>  550.0,
           Advanced::Chambersite =>   670.0,
           Advanced::Musgravite =>    770.0,
           Advanced::Grandidirite =>  800.0,
           Advanced::Poudretteite =>  880.0,
           Advanced::Serendibite =>   900.0,
           Advanced::Zektzerite =>   1000.0,
           _=> 0.0,
        }
    }
    pub fn build_stats(&self, id:f64, level:f64) -> Stats {
        match *self {
            Advanced::Painite => {
                let hp = 0.0;
                let mp = 0.0;
                let xp = 0.0;
                let hp_max = 0.0;
                let mp_max = 0.0;
                let xp_next = 10.0;
                let gp = 0.0;
                let speed = 0.0;
                let atk = .0;
                let def = 10.0;
                let m_atk = 0.0;
                let m_def = 0.0;
                Stats {
                    id,
                    xp,
                    xp_next,
                    level,
                    gp,
                    hp,
                    mp,
                    hp_max,
                    mp_max,
                    speed,
                    atk,
                    def,
                    m_atk,
                    m_def,
                }
            },
            Advanced::Hibonite => {
                let hp = 0.0;
                let mp = 0.0;
                let xp = 0.0;
                let hp_max = 0.0;
                let mp_max = 0.0;
                let xp_next = 10.0;
                let gp = 0.0;
                let speed = 0.0;
                let atk = .0;
                let def = 20.0;
                let m_atk = 0.0;
                let m_def = 0.0;
                Stats {
                    id,
                    xp,
                    xp_next,
                    level,
                    gp,
                    hp,
                    mp,
                    hp_max,
                    mp_max,
                    speed,
                    atk,
                    def,
                    m_atk,
                    m_def,
                }
            },
            Advanced::RedBeryl => {
                let hp = 0.0;
                let mp = 0.0;
                let xp = 0.0;
                let hp_max = 0.0;
                let mp_max = 0.0;
                let xp_next = 10.0;
                let gp = 0.0;
                let speed = 0.0;
                let atk = .0;
                let def = 30.0;
                let m_atk = 0.0;
                let m_def = 0.0;
                Stats {
                    id,
                    xp,
                    xp_next,
                    level,
                    gp,
                    hp,
                    mp,
                    hp_max,
                    mp_max,
                    speed,
                    atk,
                    def,
                    m_atk,
                    m_def,
                }
            },
            Advanced::Jeremejevite => {
                let hp = 0.0;
                let mp = 0.0;
                let xp = 0.0;
                let hp_max = 0.0;
                let mp_max = 0.0;
                let xp_next = 10.0;
                let gp = 0.0;
                let speed = 0.0;
                let atk = .0;
                let def = 40.0;
                let m_atk = 0.0;
                let m_def = 0.0;
                Stats {
                    id,
                    xp,
                    xp_next,
                    level,
                    gp,
                    hp,
                    mp,
                    hp_max,
                    mp_max,
                    speed,
                    atk,
                    def,
                    m_atk,
                    m_def,
                }
            },
            Advanced::Chambersite => {
                let hp = 0.0;
                let mp = 0.0;
                let xp = 0.0;
                let hp_max = 0.0;
                let mp_max = 0.0;
                let xp_next = 10.0;
                let gp = 0.0;
                let speed = 0.0;
                let atk = .0;
                let def = 50.0;
                let m_atk = 0.0;
                let m_def = 0.0;
                Stats {
                    id,
                    xp,
                    xp_next,
                    level,
                    gp,
                    hp,
                    mp,
                    hp_max,
                    mp_max,
                    speed,
                    atk,
                    def,
                    m_atk,
                    m_def,
                }
            },
            Advanced::Musgravite => {
                let hp = 0.0;
                let mp = 0.0;
                let xp = 0.0;
                let hp_max = 0.0;
                let mp_max = 0.0;
                let xp_next = 10.0;
                let gp = 0.0;
                let speed = 0.0;
                let atk = .0;
                let def = 60.0;
                let m_atk = 0.0;
                let m_def = 0.0;
                Stats {
                    id,
                    xp,
                    xp_next,
                    level,
                    gp,
                    hp,
                    mp,
                    hp_max,
                    mp_max,
                    speed,
                    atk,
                    def,
                    m_atk,
                    m_def,
                }
            },
            Advanced::Grandidirite => {
                let hp = 0.0;
                let mp = 0.0;
                let xp = 0.0;
                let hp_max = 0.0;
                let mp_max = 0.0;
                let xp_next = 10.0;
                let gp = 0.0;
                let speed = 0.0;
                let atk = .0;
                let def = 70.0;
                let m_atk = 0.0;
                let m_def = 0.0;
                Stats {
                    id,
                    xp,
                    xp_next,
                    level,
                    gp,
                    hp,
                    mp,
                    hp_max,
                    mp_max,
                    speed,
                    atk,
                    def,
                    m_atk,
                    m_def,
                }
            },
            Advanced::Poudretteite => {
                let hp = 0.0;
                let mp = 0.0;
                let xp = 0.0;
                let hp_max = 0.0;
                let mp_max = 0.0;
                let xp_next = 10.0;
                let gp = 0.0;
                let speed = 0.0;
                let atk = .0;
                let def = 80.0;
                let m_atk = 0.0;
                let m_def = 0.0;
                Stats {
                    id,
                    xp,
                    xp_next,
                    level,
                    gp,
                    hp,
                    mp,
                    hp_max,
                    mp_max,
                    speed,
                    atk,
                    def,
                    m_atk,
                    m_def,
                }
            },
            Advanced::Serendibite => {
                let hp = 0.0;
                let mp = 0.0;
                let xp = 0.0;
                let hp_max = 0.0;
                let mp_max = 0.0;
                let xp_next = 10.0;
                let gp = 0.0;
                let speed = 0.0;
                let atk = .0;
                let def = 90.0;
                let m_atk = 0.0;
                let m_def = 0.0;
                Stats {
                    id,
                    xp,
                    xp_next,
                    level,
                    gp,
                    hp,
                    mp,
                    hp_max,
                    mp_max,
                    speed,
                    atk,
                    def,
                    m_atk,
                    m_def,
                }
            },
            Advanced::Zektzerite => {
                let hp = 0.0;
                let mp = 0.0;
                let xp = 0.0;
                let hp_max = 0.0;
                let mp_max = 0.0;
                let xp_next = 10.0;
                let gp = 0.0;
                let speed = 0.0;
                let atk = .0;
                let def = 100.0;
                let m_atk = 0.0;
                let m_def = 0.0;
                Stats {
                    id,
                    xp,
                    xp_next,
                    level,
                    gp,
                    hp,
                    mp,
                    hp_max,
                    mp_max,
                    speed,
                    atk,
                    def,
                    m_atk,
                    m_def,
                }
            },
            _=> Stats::default(),
         }
    }
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
            Advanced::Painite => v = String::from("Painite"),
            Advanced::Hibonite => v = String::from("Hibonite"),
            Advanced::RedBeryl => v = String::from("RedBeryl"),
            Advanced::Jeremejevite => v = String::from("Jeremejevite"),
            Advanced::Chambersite => v = String::from("Chambersite"),
            Advanced::Musgravite => v = String::from("Musgravite"),
            Advanced::Grandidirite => v = String::from("Grandidirite"),
            Advanced::Poudretteite => v = String::from("Poudretteite"),
            Advanced::Serendibite => v = String::from("Serendibite"),
            Advanced::Zektzerite => v = String::from("Zektzerite"),
            _=> v = String::from("None"),
        }
        write!(f, "{}", v.as_str())
    }
}
impl Random for Advanced {
    type Type = Advanced;
    fn random_type(&self) -> Self::Type {
        let max = 10;
        let val = self.random_rate(max);
        match val {
            0 => Advanced::Painite,
            1 => Advanced::Hibonite,
            2 => Advanced::RedBeryl,
            3 => Advanced::Jeremejevite,
            4 => Advanced::Chambersite,
            5 => Advanced::Musgravite,
            6 => Advanced::Grandidirite,
            7 => Advanced::Poudretteite,
            8 => Advanced::Serendibite,
            9 => Advanced::Zektzerite,
            _=> Advanced::None,
        }
    }
    
}
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[cfg_attr(feature = "fltkform", derive(FltkForm))]
/*
# AdvancedBag
A container of:
 * Painite
 * Hibonite
 * RedBeryl
 * Jeremejevite
 * Chambersite
 * Musgravite
 * Grandidirite
 * Poudretteite
 * Serendibite
 * Zektzerite
*/
pub struct AdvancedBag {
    /// Number of painite gems
    pub painite:u32,
    /// Number of hibonite gems
    pub hibonite:u32,
    /// Number of redberyl gems
    pub redberyl:u32,
    /// Number of jeremejevite gems
    pub jeremejevite:u32,
    /// Number of chambersite gems
    pub chambersite:u32,
    /// Number of musgravite gems
    pub musgravite:u32,
    /// Number of grandidirite gems
    pub grandidirite:u32,
    /// Number of poudretteite gems
    pub poudretteite:u32,
    /// Number of serendibite gems
    pub serendibite:u32,
    /// Number of zektzerite gems
    pub zektzerite:u32,
}
impl AdvancedBag {
    pub fn new() -> Self {
            AdvancedBag {
            painite:0,
            hibonite:0,
            redberyl:0,
            jeremejevite:0,
            chambersite:0,
            musgravite:0,
            grandidirite:0,
            poudretteite:0,
            serendibite:0,
            zektzerite:0,
        }
    }
/*
# Sell

Sell 1 gem from your bag
*/
    pub fn sell(&mut self, gem:Advanced) -> Option<f64> {
        let mut price:f64 = 0.0;
        match gem {
            Advanced::Painite =>  {
                if self.painite >= 1 {
                    self.painite -= 1;
                    return Some(gem.price());
                }

            },
            Advanced::Hibonite =>  {
                if self.hibonite >= 1 {
                    self.hibonite -= 1;
                    return Some(gem.price());
                }

            },
            Advanced::RedBeryl =>  {
                if self.redberyl >= 1 {
                    self.redberyl -= 1;
                    return Some(gem.price());
                }

            },
            Advanced::Jeremejevite =>  {
                if self.jeremejevite >= 1 {
                    self.jeremejevite -= 1;
                    return Some(gem.price());
                }

            },
            Advanced::Chambersite =>  {
                if self.chambersite >= 1 {
                    self.chambersite -= 1;
                    return Some(gem.price());
                }

            },
            Advanced::Musgravite =>  {
                if self.musgravite >= 1 {
                    self.musgravite -= 1;
                    return Some(gem.price());
                }

            },
            Advanced::Grandidirite =>  {
                if self.grandidirite >= 1 {
                    self.grandidirite -= 1;
                    return Some(gem.price());
                }

            },
            Advanced::Poudretteite =>  {
                if self.poudretteite >= 1 {
                    self.poudretteite -= 1;
                    return Some(gem.price());
                }

            },
            Advanced::Serendibite =>  {
                if self.serendibite >= 1 {
                    self.serendibite -= 1;
                    return Some(gem.price());
                }

            },
            Advanced::Zektzerite =>  {
                if self.zektzerite >= 1 {
                    self.zektzerite -= 1;
                    return Some(gem.price());
                }

            },
            _=> (),
        }
        None
    }
/*
# Get

Put 1 gem into your bag
*/
    pub fn get(&mut self, gem:Advanced) {
        match gem {
            Advanced::Painite =>  self.painite += 1,
            Advanced::Hibonite =>  self.hibonite += 1,
            Advanced::RedBeryl =>  self.redberyl += 1,
            Advanced::Jeremejevite =>  self.jeremejevite += 1,
            Advanced::Chambersite =>  self.chambersite += 1,
            Advanced::Musgravite =>  self.musgravite += 1,
            Advanced::Grandidirite =>  self.grandidirite += 1,
            Advanced::Poudretteite =>  self.poudretteite += 1,
            Advanced::Serendibite =>  self.serendibite += 1,
            Advanced::Zektzerite =>  self.zektzerite += 1,
            _=> (),
        }
    }
}
