use crate::stats::Premade;
use crate::stats::Stats;
use crate::list::Items;

#[cfg(feature = "fltkform")]
use fltk::{prelude::*, image::SharedImage, button::Button};

use serde::{Deserialize, Serialize};
use std::io::Write;
/* # Config
This is mostly a stub just to check for file validity

Equivalent to this TOML:
```
name = "Item"
image  = "assets/icons/Item.svg"
# optional
#icon  = "assets/icons/Item_icon.svg"
description = "An example, just to give an idea."
# simple string identifier
# optional
#object = ""
# See Items
# optional
#makes = []
[stat]
id = 17
hp = 1000
mp = 90
xp = 10
hp_max = 1000
mp_max = 90
xp_next = 10
gp = 17
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
# see Items
# optional
#[drops]
#items = []
# optional
# see Special
#[specials]
# optional
# see Special
#[moves]
# optional
```

*/
#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct Config  {
    pub name:String,
    pub icon:Option<String>,
    pub image:String,
    pub description:String,
    pub object:Option<String>,
    pub makes:Option<Vec<String>>,
    pub stat:Stats,
    pub drops:Option<Items>,
    pub moves:Option<Special>,
    pub specials:Option<Special>,
}
impl Config {
    #[allow(unused)]
    /// Read in a TOML file
    pub fn read<P: Clone + AsRef<std::path::Path> + std::fmt::Debug>(filename:P) -> Option<Self> {
        if let Ok(file_string) = std::fs::read_to_string(filename.clone()) {
            let decoded:Config = match toml::from_str(file_string.as_str()) {
                Ok(decoded) => decoded,
                Err(e) => {
                    println!("Config::read()->toml::from_str() Error:{}\nFilename:{:?}", e, filename);
                    return None
                },
            };
            return Some(decoded);
        }
        None
    }
}
/* # Character
This is used by `Items` below

Equivalent to this TOML:
```
name = "Item"
image = "assets/icons/Item.svg"
# optional
# icon = "assets/icons/Item_icon.svg"
description = "An example, just to give an idea."
[stat]
id = 17
hp = 1000
mp = 90
xp = 10
hp_max = 1000
mp_max = 90
xp_next = 10
gp = 17
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
[drops]
items = []
[specials]
[moves]
```
*/
#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct Character  {
    /// The name of the Item
    pub name:String,
    /// The icon for the Item
    pub icon:Option<String>,
    /// The image of the item
    pub image:String,
    /// the description of the item
    pub description:String,
    /// The type of object
    pub object:Option<String>,
    /* What can this item turn into?
    Rock makes: CutStone.toml, Gem,.toml Marble.toml, etc...
    Mint makes: Healthy.toml
    Metal makes: Sword.toml
    */
    pub makes:Option<Vec<String>>,
    /* The Stats associated with the item
    ```
    use rpg_stat::config::Character;
    use rpg_stat::stats::Premade;
    use rpg_stat::stats::Stats;
    let mut heal = match Character::read("heal.toml") {
        Some (p) => p,
        None => return (),
    };
    let mut victim = Character::default();
    let hp = heal.hp;
    victim.set_max_hp(hp);
    victim.heal(hp);
    assert_eq!(hp, victim.hp());
    
    ```
    */
    pub stat:Stats,
    /// see Items
    pub drops:Items,
    /// see Special
    pub moves:Special,
    /// see Special
    pub specials:Special,
}
impl Character {
    #[allow(unused)]
    #[cfg(feature = "fltkform")]
    /// Make an image on a button
    pub fn make_image_button<P: AsRef<std::path::Path>>(filename: P, mut button:Button)  {
        let img = match SharedImage::load(filename) {
            Ok(i) => i,
            Err(e) => {
                println!("Character::make_image_button()-> SharedImage::load() Error:{}",e);
                return
            },
        };
        button.set_image(Some(img));
    }
    #[allow(unused)]
    /// Read in a TOML file
    pub fn read<P: Clone + AsRef<std::path::Path> + std::fmt::Debug>(filename:P) -> Option<Self> {
        if let Ok(file_string) = std::fs::read_to_string(filename.clone()) {
            let decoded:Character = match toml::from_str(file_string.as_str()) {
                Ok(decoded) => decoded,
                Err(e) => {
                    println!("Character::read()->toml::from_str() Error:{}\nFilename:{:?}", e, filename);
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
        Character::write(self.clone(), path)
    }
    #[allow(unused)]
    /// Write a TOML file
    pub fn write(save:Character, path:&str) -> bool {
        let toml = match toml::to_string(&save){
            Ok(toml) => toml,
            Err(e) => {
                println!("Character::save problem:\ntoml::to_string error:{}", e);
                return false;
            },
        };
        let mut output = match std::fs::File::create(path) {
            Ok(out) => out,
            Err(e) => {
                println!("Character::save problem:\nFile::create({}) error:{}", path, e);
                return false;
            },
            
        };
        match write!(output, "{}", toml) {
            Ok(_) => (),
            Err(e) => {
                println!("Character::save problem:\nwrite! error:{}", e);
                return false;
            },
        }
        true
    }
    #[allow(unused)]
    /// Sell Character item
    pub fn sell(&self) -> f64 {
        self.gp()
    }
    #[allow(unused)]
    /// Price of config item
    pub fn price(&self) -> f64 {
        self.gp()
    }
}
// see `rpg_stat::stats::Premade` for more info
impl Premade for Character {
    fn stat(&self) -> Stats {
        self.stat
    }
    fn set_hp(&mut self, amount:f64) {
        self.stat.hp = amount;
    }
    fn set_mp(&mut self, amount:f64) {
        self.stat.mp = amount;
    }
    fn set_xp(&mut self, amount:f64) {
        self.stat.xp = amount;
    }
    fn set_hp_max(&mut self, amount:f64) {
        self.stat.hp_max = amount;
    }
    fn set_mp_max(&mut self, amount:f64) {
        self.stat.mp_max = amount;
    }
    fn set_xp_next(&mut self, amount:f64) {
        self.stat.xp_next = amount;
    }
    fn set_gp(&mut self, amount:f64) {
        self.stat.gp = amount;
    }
    fn set_atk(&mut self, amount:f64) {
        self.stat.atk = amount;
    }
    fn set_def(&mut self, amount:f64) {
        self.stat.def = amount;
    }
    fn set_m_atk(&mut self, amount:f64) {
        self.stat.m_atk = amount;
    }
    fn set_m_def(&mut self, amount:f64) {
        self.stat.m_def = amount;
    }
    fn set_level(&mut self, amount:f64) {
        self.stat.level = amount;
    }
    fn set_speed(&mut self, amount:f64) {
        self.stat.speed = amount;
    }
}
/* # Item
Specific configuration files for items
*/
#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct Item  {
    /// The name of the Item
    pub name:String,
    /// The icon for the Item
    pub icon:String,
    /// The image of the item
    pub image:String,
    /// the description of the item
    pub description:String,
    /// The type of object
    pub object:String,
    /* What can this item turn into?
    Rock makes: CutStone.toml, Gem,.toml Marble.toml, etc...
    Mint makes: Healthy.toml
    Metal makes: Sword.toml
    */
    pub makes:Vec<String>,
    /* The Stats associated with the item
    ```
    use uhambo::config::Character;
    use rpg_stat::stats::Premade;
    use rpg_stat::stats::Stats;
    let mut heal = match Character::read("heal.toml") {
        Some (p) => p,
        None => return (),
    };
    let mut victim = Character::default();
    let hp = heal.hp;
    victim.set_max_hp(hp);
    victim.heal(hp);
    assert_eq!(hp, victim.hp());
    
    ```
    */
    pub stat:Stats,
}
impl Item {
    #[allow(unused)]
    #[cfg(feature = "fltkform")]
    /// Make an image on a button
    pub fn make_image_button<P: AsRef<std::path::Path>>(filename: P, mut button:Button)  {
        let img = match SharedImage::load(filename) {
            Ok(i) => i,
            Err(e) => {
                println!("Item::make_image_button()-> SharedImage::load() Error:{}",e);
                return
            },
        };
        button.set_image(Some(img));
    }
    #[allow(unused)]
    /// Read in a TOML file
    pub fn read<P: Clone + AsRef<std::path::Path> + std::fmt::Debug>(filename:P) -> Option<Self> {
        if let Ok(file_string) = std::fs::read_to_string(filename.clone()) {
            let decoded:Self = match toml::from_str(file_string.as_str()) {
                Ok(decoded) => decoded,
                Err(e) => {
                    println!("Item::read()->toml::from_str() Error:{}\nFilename:{:?}", e, filename);
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
        Item::write(self.clone(), path)
    }
    #[allow(unused)]
    /// Write a TOML file
    pub fn write(save:Item, path:&str) -> bool {
        let toml = match toml::to_string(&save){
            Ok(toml) => toml,
            Err(e) => {
                println!("Item::save problem:\ntoml::to_string error:{}", e);
                return false;
            },
        };
        let mut output = match std::fs::File::create(path) {
            Ok(out) => out,
            Err(e) => {
                println!("Item::save problem:\nFile::create({}) error:{}", path, e);
                return false;
            },
            
        };
        match write!(output, "{}", toml) {
            Ok(_) => (),
            Err(e) => {
                println!("Item::save problem:\nwrite! error:{}", e);
                return false;
            },
        }
        true
    }
    #[allow(unused)]
    /// Sell Item item
    pub fn sell(&self) -> f64 {
        self.gp()
    }
    #[allow(unused)]
    /// Price of config item
    pub fn price(&self) -> f64 {
        self.gp()
    }
}
// see `rpg_stat::stats::Premade` for more info
impl Premade for Item {
    fn stat(&self) -> Stats {
        self.stat
    }
    fn set_hp(&mut self, amount:f64) {
        self.stat.hp = amount;
    }
    fn set_mp(&mut self, amount:f64) {
        self.stat.mp = amount;
    }
    fn set_xp(&mut self, amount:f64) {
        self.stat.xp = amount;
    }
    fn set_hp_max(&mut self, amount:f64) {
        self.stat.hp_max = amount;
    }
    fn set_mp_max(&mut self, amount:f64) {
        self.stat.mp_max = amount;
    }
    fn set_xp_next(&mut self, amount:f64) {
        self.stat.xp_next = amount;
    }
    fn set_gp(&mut self, amount:f64) {
        self.stat.gp = amount;
    }
    fn set_atk(&mut self, amount:f64) {
        self.stat.atk = amount;
    }
    fn set_def(&mut self, amount:f64) {
        self.stat.def = amount;
    }
    fn set_m_atk(&mut self, amount:f64) {
        self.stat.m_atk = amount;
    }
    fn set_m_def(&mut self, amount:f64) {
        self.stat.m_def = amount;
    }
    fn set_level(&mut self, amount:f64) {
        self.stat.level = amount;
    }
    fn set_speed(&mut self, amount:f64) {
        self.stat.speed = amount;
    }
}
/* # Special
This is used by `Items` below

Equivalent to this TOML:
```
name = "Item"
icon  = "assets/icons/Item.svg"
description = "An example, just to give an idea."
[stat]
id = 17
hp = 1000
mp = 90
xp = 10
hp_max = 1000
mp_max = 90
xp_next = 10
gp = 17
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
*/
#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct Special  {
    /// The name of the Item
    pub name:String,
    /// The icon for the Item
    pub icon:String,
    /// The image of the item
    pub image:String,
    /// the description of the item
    pub description:String,
    //TODO type?
    /// The Stats associated with the item
    pub stat:Stats,
}
impl Special {
    #[allow(unused)]
    /// Make an image on a button
    #[cfg(feature = "fltkform")]
    pub fn make_image_button<P: AsRef<std::path::Path>>(filename: P, mut button:Button)  {
        let img = SharedImage::load(filename).ok();
        button.set_image(img);
    }
    #[allow(unused)]
    /// Read in a TOML file
    pub fn read<P: Clone + AsRef<std::path::Path> + std::fmt::Debug>(filename:P) -> Option<Self> {
        if let Ok(file_string) = std::fs::read_to_string(filename.clone()) {
            let decoded:Special = match toml::from_str(file_string.as_str()) {
                Ok(decoded) => decoded,
                Err(e) => {
                    println!("Special::read()->toml::from_str() Error:{}\nFilename:{:?}", e, filename);
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
        Special::write(self.clone(), path)
    }
    #[allow(unused)]
    /// Write a TOML file
    pub fn write(save:Special, path:&str) -> bool {
        let toml = match toml::to_string(&save){
            Ok(toml) => toml,
            Err(e) => {
                println!("Special::save problem:\ntoml::to_string error:{}", e);
                return false;
            },
        };
        let mut output = match std::fs::File::create(path) {
            Ok(out) => out,
            Err(e) => {
                println!("Special::save problem:\nFile::create({}) error:{}", path, e);
                return false;
            },
            
        };
        match write!(output, "{}", toml) {
            Ok(_) => (),
            Err(e) => {
                println!("Special::save problem:\nwrite! error:{}", e);
                return false;
            },
        }
        true
    }
    #[allow(unused)]
    /// Sell Special item
    pub fn sell(&self) -> f64 {
        self.gp()
    }
    #[allow(unused)]
    /// Price of config item
    pub fn price(&self) -> f64 {
        self.gp()
    }
}
// see `rpg_stat::stats::Premade` for more info
impl Premade for Special {
    fn stat(&self) -> Stats {
        self.stat
    }
    fn set_hp(&mut self, amount:f64) {
        self.stat.hp = amount;
    }
    fn set_mp(&mut self, amount:f64) {
        self.stat.mp = amount;
    }
    fn set_xp(&mut self, amount:f64) {
        self.stat.xp = amount;
    }
    fn set_hp_max(&mut self, amount:f64) {
        self.stat.hp_max = amount;
    }
    fn set_mp_max(&mut self, amount:f64) {
        self.stat.mp_max = amount;
    }
    fn set_xp_next(&mut self, amount:f64) {
        self.stat.xp_next = amount;
    }
    fn set_gp(&mut self, amount:f64) {
        self.stat.gp = amount;
    }
    fn set_atk(&mut self, amount:f64) {
        self.stat.atk = amount;
    }
    fn set_def(&mut self, amount:f64) {
        self.stat.def = amount;
    }
    fn set_m_atk(&mut self, amount:f64) {
        self.stat.m_atk = amount;
    }
    fn set_m_def(&mut self, amount:f64) {
        self.stat.m_def = amount;
    }
    fn set_level(&mut self, amount:f64) {
        self.stat.level = amount;
    }
    fn set_speed(&mut self, amount:f64) {
        self.stat.speed = amount;
    }
}


#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
/* # Special
The premade special move holder you define by files!
These are used in the creature struct and the configuration file for it
```
# NO MOVES
[specials]
[moves]
```
To give moves, simply add a line to the list
```
[specials]
one = "path/to/special_file.toml"
[moves]
[moves.one]
name = "Slash"
icon = "assets/moves/Slash_icon.svg"
image = "assets/moves/Slash.svg"
description = "Learn the move Slash"
```
*/
pub struct Specials {
    /// The first move's configuration
    pub one:Option<Special>,
    /// The second move's configuration
    pub two:Option<Special>,
    /// The third move's configuration
    pub three:Option<Special>,
    /// The fourth move's configuration
    pub four:Option<Special>,
}
impl Specials {
    /* # Moves
    A vector of special moves
    */
    pub fn moves(&self) -> Vec<Option<Special>> {
        vec![
            self.one.clone(),
            self.two.clone(),
            self.three.clone(),
            self.four.clone(),
        ]
    }
    pub fn get(&mut self, special:Special) -> bool {
        if self.one.is_none() {
            self.one = Some(special);
        } else if self.two.is_none() {
            self.two = Some(special);
        } else if self.three.is_none() {
            self.three = Some(special);
        } else if self.four.is_none() {
            self.four  = Some(special);
        } else {
            return false;
        }
        true
    }
}
