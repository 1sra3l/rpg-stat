/*!
# Stats Stats
*/
use std::fmt;
//use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use std::ops::{Add, AddAssign,  Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

// #Normal
// #Special
use crate::special::Normal as Special;
use crate::special::ManaCost;
use crate::item::Item;
use crate::item::Normal as MyItem;
// #Condition
use crate::effect::Normal as Condition;
// #Element
use crate::types::Normal as Element;
use crate::attributes::{Stage, Rate, Effectiveness};
use crate::random::*;

#[cfg(feature = "fltkform")]
use fltk::{prelude::*, *};
#[cfg(feature = "fltkform")]
use fltk_form_derive::*;
#[cfg(feature = "fltkform")]
use fltk_form::{FltkForm, HasProps};

/*
# Stats Stats

These stats exist for the sole purpose of raising and training creatures
This stat is
*/
#[derive( Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "fltkform", derive(FltkForm))]
pub struct Stats {
    /// Identification number
    pub id:u32,
    /// Name of creature
    pub name:String,
    /// Current Stage
    pub form:Stage,
    ///
    pub condition:Condition,
    pub element1:Element,
    pub element2:Element,
    pub rate:f64,
    /// this is the owner's id
    pub owner:u32,
    /// level cycle xp (for display)
    pub xp:f64,
    /// Total XP
    pub total_xp:f64,
    /// Experience points for leveling Health Points
    pub hp_xp:f64,
    /// Experience points for leveling Attack
    pub atk_xp:f64,
    /// Experience points for leveling Defense
    pub def_xp:f64,
    /// Experience points for leveling Speed
    pub speed_xp:f64,
    /// Experience points for leveling Special
    pub special_xp:f64,
    /// Current level
    pub level:f64,
    /// Maximum Health Points
    pub hp_max:f64,
    /// Current Health Points
    pub hp:f64,
    /// Attack power
    pub atk:f64,
    /// Defense capability
    pub def:f64,
    /// Speed of creature in battle, or as they move
    pub speed:f64,
    /// 
    pub special:f64,
    /// Putting a file name here generates a picture
    pub image:String,
    /// [Special Move](@TODO@)
    pub move0:Special,
    /// Mana Points for Special move 0
    pub move0_mp:f64,
    /// [Special Move](@TODO@)
    pub move1:Special,
    /// Mana Points for Special move 1
    pub move1_mp:f64,
    /// [Special Move](@TODO@)
    pub move2:Special,
    /// Mana Points for Special move 2
    pub move2_mp:f64,
    /// [Special Move](@TODO@)
    pub move3:Special,
    /// Mana Points for Special move 3
    pub move3_mp:f64,
    /// [Special Move](@TODO@)
    pub move4:Special,
    /// Mana Points for Special move 4
    pub move4_mp:f64,
    /// [MyItem type](@todo@)
    pub item0:MyItem,
    /// Number of items in slot 0
    pub items0:f64,
    /// [MyItem type](@todo@)
    pub item1:MyItem,
    /// Number of items in slot 1
    pub items1:f64,
    /// [MyItem type](@todo@)
    pub item2:MyItem,
    /// Number of items in slot 2
    pub items2:f64,
    /// [MyItem type](@todo@)
    pub item3:MyItem,
    /// Number of items in slot 3
    pub items3:f64,
    /// [MyItem type](@todo@)
    pub item4:MyItem,
    /// Number of items in slot 4
    pub items4:f64,
}
impl  Default for Stats {
    fn default() -> Self where Self:Sized {
        Self::new()
    }
}
impl Random for Stats {
    type Type = Stats;
    fn random_type(&self) -> Self::Type {
        let mut elem = Element::Rock;
        elem = elem.random_type();
        let hp = self.random(10.0,50.0);
        let atk = self.random(5.0,50.0);
        let def = self.random(5.0,50.0);
        let speed = self.random(5.0,50.0);
        let special = self.random(5.0,50.0);
        let form = Stage::Teen;
        let spec = Special::None;
        let item = MyItem::None;
        let move0 = spec.random_type();
        let move1 = spec.random_type();
        Stats {
            id:self.random_rate(100),
            name:random_creature_name(),
            form:form.random_type(),
            condition:Condition::None,
            element1:elem,
            element2:Element::None,
            rate:self.random(5.0,90.0),
            move0:move0,
            move0_mp:move0.mp_total(0.0),
            move1:move1,
            move1_mp:move1.mp_total(0.0),
            move2:spec,
            move2_mp:0.0,
            move3:spec,
            move3_mp:0.0,
            move4:spec,
            move4_mp:0.0,
            item0:item,
            items0:0.0,
            item1:item,
            items1:0.0,
            item2:item,
            items2:0.0,
            item3:item,
            items3:0.0,
            item4:item,
            items4:0.0,
            owner:0,
            xp:0.0,
            total_xp:0.0,
            hp_xp:hp,
            atk_xp:atk,
            def_xp:def,
            speed_xp:speed,
            special_xp:special,
            level:1.0,
            hp_max:hp,
            hp:hp,
            atk:atk,
            def:def,
            speed:speed,
            special:special,
            image:String::from(""),
            
        }
    }
    
}
impl Stats {
    pub fn heal(&mut self, value:f64) -> bool {
        if value < 0.0 {
            return false;
        }
        self.hp += value;
        if self.hp > self.hp_max {
            self.hp = self.hp_max;
        }
        true
    }
    pub fn next(&self) -> f64 {
        self.level * 20.0
    }
    pub fn level_up(&mut self) {
        println!("xp:{} next:{} total:{}", self.xp, self.next(), self.total_xp);
        self.total_xp += self.xp;
        if self.xp > self.next() {
            if self.hp_xp > self.hp_max {
               self.hp_max = self.hp_xp; 
            } else {
                self.hp_xp += self.level;
            }
            if self.atk_xp > self.atk {
                self.atk = self.atk_xp; 
            } else {
                self.atk_xp += self.level;
            }
            if self.def_xp > self.def {
                self.def = self.def_xp; 
            } else {
                self.def_xp += self.level;
            }
            if self.speed_xp > self.speed {
                self.speed = self.speed_xp; 
            } else {
                self.speed_xp += self.level;
            }
            if self.special_xp > self.special {
                self.special = self.special_xp; 
            } else {
                self.special_xp += self.level;
            }
            self.level += 1.0;
            self.xp = 0.0;
        }
        println!("level:{}",self.level);
    }
    pub fn new() -> Self {
        Stats {
            id:0,
            name:String::from(""),
            form:Stage::Baby,
            condition:Condition::None,
            element1:Element::None,
            element2:Element::None,
            rate:0.0,
            item0:MyItem::None,
            items0:0.0,
            item1:MyItem::None,
            items1:0.0,
            item2:MyItem::None,
            items2:0.0,
            item3:MyItem::None,
            items3:0.0,
            item4:MyItem::None,
            items4:0.0,
            move0:Special::None,
            move0_mp:0.0,
            move1:Special::None,
            move1_mp:0.0,
            move2:Special::None,
            move2_mp:0.0,
            move3:Special::None,
            move3_mp:0.0,
            move4:Special::None,
            move4_mp:0.0,
            owner:0,
            xp:0.0,
            total_xp:0.0,
            hp_xp:0.0,
            atk_xp:0.0,
            def_xp:0.0,
            speed_xp:0.0,
            special_xp:0.0,
            level:0.0,
            hp_max:0.0,
            hp:0.0,
            atk:0.0,
            def:0.0,
            speed:0.0,
            special:0.0,
            image:String::from(""),
        }
    }
    pub fn use_mp(&mut self, move_number:u32) -> bool {
        let value = 1.0;
        match move_number {
            0 => {
                self.move0_mp -= value;
                return true;
            },
            1 => {
                self.move1_mp -= value;
                return true;
            },
            2 => {
                self.move2_mp -= value;
                return true;
            },
            3 => {
                self.move3_mp -= value;
                return true;
            },
            4 => {
                self.move4_mp -= value;
                return true;
            },
            _=> return false,
        }
    }
    pub fn get_mp(&self, move_number:u32) -> f64 {
        let value = 1.0;
        match move_number {
            0 => return self.move0_mp,
            1 => return self.move1_mp,
            2 => return self.move2_mp,
            3 => return self.move3_mp,
            4 => return self.move4_mp,
            _=> return 0.0,
        }
    }
    pub fn restore_mp(&mut self, move_number:u32, value:f64) -> bool {
        match move_number {
            0 => {
                self.move0_mp += value;
                let limit = self.move0.mp_total(0.0);
                if self.move0_mp > limit {
                    self.move0_mp = limit;
                }
                return true;
            },
            1 => {
                self.move1_mp += value;
                let limit = self.move1.mp_total(0.0);
                if self.move1_mp > limit {
                    self.move1_mp = limit;
                }
                return true;
            },
            2 => {
                self.move2_mp += value;
                let limit = self.move2.mp_total(0.0);
                if self.move2_mp > limit {
                    self.move2_mp = limit;
                }
                return true;
            },
            3 => {
                self.move3_mp += value;
                let limit = self.move3.mp_total(0.0);
                if self.move3_mp > limit {
                    self.move3_mp = limit;
                }
                return true;
            },
            4 => {
                self.move4_mp += value;
                let limit = self.move4.mp_total(0.0);
                if self.move4_mp > limit {
                    self.move4_mp = limit;
                }
                return true;
            },
            _=> return false,
        }
    }
    pub fn moves(&self) -> Vec<Special> {
        let mut vec:Vec<Special> = vec![];
        if self.move0 != Special::None {
            vec.push(self.move0);
        }
        if self.move1 != Special::None {
            vec.push(self.move1);
        }
        if self.move2 != Special::None {
            vec.push(self.move2);
        }
        if self.move3 != Special::None {
            vec.push(self.move3);
        }
        if self.move4 != Special::None {
            vec.push(self.move4);
        }
        vec
    }
    pub fn add_move(&mut self, special:Special) -> bool {
        if self.move0 != Special::None {
            self.move0 = special;
            return true;
        }
        if self.move1 != Special::None {
            self.move1 = special;
            return true;
        }
        if self.move2 != Special::None {
            self.move2 = special;
            return true;
        }
        if self.move3 != Special::None {
            self.move3 = special;
            return true;
        }
        if self.move4 != Special::None {
            self.move4 = special;
            return true;
        }
        false
    }
    pub fn remove_move(&mut self, move_number:u32) -> bool {
        match move_number {
            0 => {
                self.move0 = Special::None;
                return true;
            },
            1 => {
                self.move1 = Special::None;
                return true;
            },
            2 => {
                self.move2 = Special::None;
                return true;
            },
            3 => {
                self.move3 = Special::None;
                return true;
            },
            4 => {
                self.move4 = Special::None;
                return true;
            },
            _=> false,
        }
    }
    pub fn get_move(&self, move_number:u32) -> Special {
        match move_number {
            4 => self.move4,
            1 => self.move1,
            2 => self.move2,
            3 => self.move3,
            _=> self.move0,
        }
    }
    pub fn valid_move(&self, move_number:u32) -> bool {
        match move_number {
            1 => {
                if self.move1 == Special::None {
                    return false
                } else {
                    return true
                }
            },
            2 => {
                if self.move2 == Special::None {
                    return false
                } else {
                    return true
                }
            },
            3 => {
                if self.move3 == Special::None {
                    return false
                } else {
                    return true
                }
            },
            4 => {
                if self.move4 == Special::None {
                    return false
                } else {
                    return true
                }
            },
            _=> {
                if self.move0 == Special::None {
                    return false
                } else {
                    return true
                }
            },
        }
    }
    
    pub fn damage_attack(&mut self, atk_move:Special, other:Stats) -> f64 {
        //first math
        let dmg = atk_move.damage(self.level);
        dmg * self.atk
    }
    pub fn special(&mut self, id:usize, other:Stats) -> Option<f64> {
        let vec = self.moves().clone();
        if vec.len() < id {
            return None
        }
        let atk_move = vec[id].clone();
        let atk = self.atk;
        let mut result = self.damage_attack(atk_move.clone(), other.clone());//dmg * self.atk
        let def = other.def + other.hp;
        if result == 0.0 {
            result = def;
        }
        result /= def;
        if result > other.hp {
            result = other.hp;
        }
        Some(result)
    }
    pub fn items(&self) -> Vec<MyItem> {
        let mut vec:Vec<MyItem> = vec![];
        if self.item0 != MyItem::None {
            vec.push(self.item0);
        }
        if self.item1 != MyItem::None {
            vec.push(self.item1);
        }
        if self.item2 != MyItem::None {
            vec.push(self.item2);
        }
        if self.item3 != MyItem::None {
            vec.push(self.item3);
        }
        if self.item4 != MyItem::None {
            vec.push(self.item4);
        }
        vec
    }
    pub fn add_item(&mut self, special:MyItem) -> bool {
        if self.item0 != MyItem::None {
            self.item0 = special;
            return true;
        }
        if self.item1 != MyItem::None {
            self.item1 = special;
            return true;
        }
        if self.item2 != MyItem::None {
            self.item2 = special;
            return true;
        }
        if self.item3 != MyItem::None {
            self.item3 = special;
            return true;
        }
        if self.item4 != MyItem::None {
            self.item4 = special;
            return true;
        }
        false
    }
    pub fn remove_item(&mut self, item_number:u32) -> bool {
        match item_number {
            0 => {
                self.item0 = MyItem::None;
                return true;
            },
            1 => {
                self.item1 = MyItem::None;
                return true;
            },
            2 => {
                self.item2 = MyItem::None;
                return true;
            },
            3 => {
                self.item3 = MyItem::None;
                return true;
            },
            4 => {
                self.item4 = MyItem::None;
                return true;
            },
            _=> false,
        }
    }
    pub fn get_item(&self, item_number:u32) -> MyItem {
        match item_number {
            0 => self.item0,
            1 => self.item1,
            2 => self.item2,
            3 => self.item3,
            _=> self.item4,
        }
    }

    pub fn use_item(&mut self, item_number:u32) -> bool {
        let item:MyItem;
        match item_number {
            0 => item = self.item0,
            1 => item = self.item1,
            2 => item = self.item2,
            3 => item = self.item3,
            4=> item = self.item4,
            _=> return false,
        }
        let val = item.value();
        match item {
            MyItem::Hp => return self.heal(val),
            MyItem::Mp => return self.restore_mp(item_number, val),
            MyItem::Heal => self.condition = Condition::None,
            MyItem::Exp => self.xp += val,
            MyItem::Def => self.def += val,
            MyItem::Atk => self.atk += val,
            MyItem::Speed => self.speed += val,
            MyItem::Special => self.special += val,
            _=> return false,
        }
        return true
    }
}
