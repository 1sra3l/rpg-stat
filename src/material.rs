/*!# Materials 


*/
use std::fmt::Debug;
//use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign};
extern crate num;
use serde::{Deserialize, Serialize};
use crate::stats::Stats;
use crate::stats::Premade;

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
/*# Drop File
Read in a file with this format:
```toml
name = "Claw"
image  = "assets/drops/Claw.svg"
makes = ["assets/refinery/Necklace.toml", "assets/refinery/EarRing.toml"]
[stats]
id = 8
hp = 1000
mp = 90
xp = 10
hp_max = 1000
mp_max = 90
xp_next = 10
gp = 8
speed = 10
level = 1
atk = 90
def = 100
m_atk = 90
m_def = 90
agility = 10
strength = 90
intelligence = 10
dexterity = 10
constitution = 90
charisma = 10
wisdom = 10
age = 10
```
using:
```rs:no_run
let drop = DropFile::read(filename);
```

*/
pub struct DropFile {
    pub name:String,
    pub makes:Vec<String>,
    pub image:String,
    pub stats:Stats,
}
impl DropFile {
    pub fn read<P: AsRef<std::path::Path>>(filename:P) -> Self {
        if let Ok(file_string) = std::fs::read_to_string(filename) {
            let decoded:DropFile = match toml::from_str(file_string.as_str()) {
                Ok(decoded) => decoded,
                Err(e) => {
                    println!("DropFile::read()->toml::from_str() Error:{}",e);
                    return Self::default()
                },
            };
            return decoded;
        }
        Self::default()
    }
    pub fn sell(&self) -> f64 {
        self.gp()
    }
}
impl Premade for DropFile {
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

/*# Drop Inventory

*/
#[derive(Debug, Default, Clone, PartialEq)]
pub struct DropInventory {
    pub items:Vec<DropFile>,
    pub count:Vec<f64>,
}

/*# Drop List

*/
#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct DropList {
    pub items:Vec<String>,
}
impl DropList {
    pub fn get_item(&self, item:usize) -> Option<DropFile> {
        if item >= self.items.len() {
            return None;
        }
        Some(DropFile::read(self.items[item].clone()))
    }
    pub fn get_all(&self) -> Vec<DropFile> {
        let mut return_vec:Vec<DropFile> = vec![];
        for i in self.items.clone() {
            let c = DropFile::read(i.clone());
            if c != DropFile::default() {
                return_vec.push(c);
            }
        }
        return_vec
    }
    pub fn read<P: AsRef<std::path::Path>>(filename:P) -> Self {
        if let Ok(file_string) = std::fs::read_to_string(filename) {
            let decoded:DropList = match toml::from_str(file_string.as_str()) {
                Ok(decoded) => decoded,
                Err(e) => {
                    println!("DropList::read()->toml::from_str() Error:{}",e);
                    return Self::default()
                },
            };
            return decoded;
        }
        Self::default()
    }
}
