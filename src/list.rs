/*! # List
The way to store config files

*/
use serde::{Deserialize, Serialize};
use std::io::Write;

use crate::config::{Character, Item, Special, Config};
use crate::random::Random;

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
/// The List of items
pub struct List {
    /// This is a string of items in a list
    pub items:Vec<String>,
}
impl List {
    #[allow(unused)]
    /* Get an item from the list as an Optional `Item`
    this basically looks at the item as a toml file and loads it
    */
    pub fn get_item(&self, item:usize) -> Option<Item> {
        if item >= self.items.len() {
            return None;
        }
        Item::read(self.items[item].clone())
    }
    #[allow(unused)]
    /* Get an item from the list as an Optional `Item`
    this basically looks at the item as a toml file and loads it
    */
    pub fn get_special(&self, item:usize) -> Option<Special> {
        if item >= self.items.len() {
            return None;
        }
        Special::read(self.items[item].clone())
    }
    #[allow(unused)]
    /* Get an item from the list as an Optional `Character`
    this basically looks at the item as a toml file and loads it
    */
    pub fn get_character(&self, item:usize) -> Option<Character> {
        if item >= self.items.len() {
            return None;
        }
        Character::read(self.items[item].clone())
    }
    #[allow(unused)]
    /// Get a vector of items
    /// Reads each item as a file or returns an empty vector
    pub fn get_all_characters(&self) -> Vec<Character> {
        let mut return_vec:Vec<Character> = vec![];
        for i in self.items.clone() {
            let c = match Character::read(i.clone()){
                Some(c) =>{
                    return_vec.push(c.clone());
                    c
                },
                None => continue,
            };
        }
        return_vec
    }
    #[allow(unused)]
    /// Get a vector of items
    /// Reads each item as a file or returns an empty vector
    pub fn get_all_items(&self) -> Vec<Item> {
        let mut return_vec:Vec<Item> = vec![];
        for i in self.items.clone() {
            let c = match Item::read(i.clone()){
                Some(c) =>{
                    return_vec.push(c.clone());
                    c
                },
                None => continue,
            };
        }
        return_vec
    }
    #[allow(unused)]
    /// Get a vector of items
    /// Reads each item as a file or returns an empty vector
    pub fn get_all_specials(&self) -> Vec<Special> {
        let mut return_vec:Vec<Special> = vec![];
        for i in self.items.clone() {
            let c = match Special::read(i.clone()){
                Some(c) =>{
                    return_vec.push(c.clone());
                    c
                },
                None => continue,
            };
        }
        return_vec
    }
    #[allow(unused)]
    /// Read in a TOML file
    pub fn read<P: Clone + AsRef<std::path::Path> + std::fmt::Debug>(filename:P) -> Self {
        if let Ok(file_string) = std::fs::read_to_string(filename.clone()) {
            let decoded:List = match toml::from_str(file_string.as_str()) {
                Ok(decoded) => decoded,
                Err(e) => {
                    println!("List::read()->toml::from_str() Error:{}\nFilename:{:?}",e, filename);
                    return Self::default()
                },
            };
            return decoded;
        }
        Self::default()
    }
    #[allow(unused)]
    /// Save a TOML FILE
    pub fn save(&self, path:&str) -> bool {
        List::write(self.clone(), path)
    }
    #[allow(unused)]
    /// Write a TOML file
    pub fn write(save:List, path:&str) -> bool {
        let toml = match toml::to_string(&save){
            Ok(toml) => toml,
            Err(e) => {
                println!("List::save problem:\ntoml::to_string error:{}", e);
                return false;
            },
        };
        let mut output = match std::fs::File::create(path) {
            Ok(out) => out,
            Err(e) => {
                println!("List::save problem:\nFile::create({}) error:{}", path, e);
                return false;
            },
            
        };
        match write!(output, "{}", toml) {
            Ok(_) => (),
            Err(e) => {
                println!("List::save problem:\nwrite! error:{}", e);
                return false;
            },
        }
        true
    }
}

#[derive(Debug, Clone, Default, PartialEq, Deserialize, Serialize)]
/*
This is used to hold The filename of an item in the List above, as well as an amount of those items
in the TOML file these are usually stored in an array
```
items = [ {filename = "", amount = 0}, {filename = "", amount = 0} ]
```
*/
pub struct Object{
    /// The TOML file's path
    pub filename:String,
    /// The amount, mainly used for saving
    pub amount:f64,
}
impl Object {
    /// Return is as an optional Character
    pub fn as_character(&self) -> Option<Character> {
        Character::read(self.filename.clone())
    }
    /// Return is as an optional Item
    pub fn as_item(&self) -> Option<Item> {
        Item::read(self.filename.clone())
    }
    /// Return is as an optional Special
    pub fn as_special(&self) -> Option<Special> {
        Special::read(self.filename.clone())
    }
    /// wrapper for config
    pub fn sell(&self) -> f64 {
        match self.as_item(){
            Some(l) => l.sell(),
            None => 0.0,
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
/* # Items
This is the container for the Objects.

Here is what the files look like:
```
items = [ {filename = "", amount = 0}, {filename = "", amount = 0} ]
```
*/
pub struct Items {
    /// The type of list being used
    pub object:String,
    /// The vector of items
    pub items:Vec<Object>,
}
impl Random for Items {
    type Type = Object;
    fn random_type(&self) -> Self::Type {
        let max:u32 = self.items.clone().len() as u32;
        let val:usize = self.random_rate(max) as usize;
        self.items[val].clone()
   }
    
}
impl Items {
    /// Create a new one with an "object
    pub fn new(object:String) -> Self {
        Items {
            object,
            items:vec![],
        }
    }
    #[allow(unused)]
    /// Read in a TOML file
    pub fn read<P: Clone + AsRef<std::path::Path> + std::fmt::Debug>(filename:P) -> Option<Self> {
        if let Ok(file_string) = std::fs::read_to_string(filename.clone()) {
            let decoded:Items = match toml::from_str(file_string.as_str()) {
                Ok(decoded) => decoded,
                Err(e) => {
                    println!("Items::read()->toml::from_str() Error:{}\nFilename:{:?}",e, filename);
                    return None;
                },
            };
            return Some(decoded);
        }
        None
    }
    /// Give a default if the config file is bad
    pub fn read_default<P: Clone + AsRef<std::path::Path> + std::fmt::Debug>(filename:P) -> Self {
        match Self::read(filename) {
            Some(list) => list,
            None => Self::default(),
        }
    }
    #[allow(unused)]
    /// Save a TOML FILE
    pub fn save(&self, path:&str) -> bool {
        Items::write(self.clone(), path)
    }
    #[allow(unused)]
    /// Write a TOML file
    pub fn write(save:Items, path:&str) -> bool {
        let toml = match toml::to_string(&save){
            Ok(toml) => toml,
            Err(e) => {
                println!("Items::save problem:\ntoml::to_string error:{}", e);
                return false;
            },
        };
        let mut output = match std::fs::File::create(path) {
            Ok(out) => out,
            Err(e) => {
                println!("Items::save problem:\nFile::create({}) error:{}", path, e);
                return false;
            },
            
        };
        match write!(output, "{}", toml) {
            Ok(_) => (),
            Err(e) => {
                println!("Items::save problem:\nwrite! error:{}", e);
                return false;
            },
        }
        true
    }
    #[allow(unused)]
    /* # Add an item
    aquire a Object
    This will check the filename and either add it to those specific items, or add the new thing
    */
    pub fn add(&mut self, item:Object) -> bool {
        let filename = item.filename.clone();
        // must be a real file
        let _c = match Config::read(filename.clone()) {
            Some(c) => c ,
            None => {
                println!("{} must be a real file", filename);
                return false
            },
        };
        let one:f64 = 1.0;
        for mut i in self.items.clone() {
            let f = i.filename.clone();
            if f == filename {
                // add one
                i.amount += one;
                return true;
            }
        }
        // add new item
        self.items.push(item);
        true
    }
    #[allow(unused)]
    /* # Use
    Use an item from the `Items`
    */
    pub fn consume(&mut self, item:Object) -> bool{
        let filename = item.filename;
        let one:f64 = 1.0;
        for mut i in self.items.clone() {
            let f = i.filename.clone();
            if f == filename {
                if i.amount < one {
                    
                }
                // subtract one
                i.amount -= one;
                return true;
            }
        }
        false
    }
    #[allow(unused)]
    /* # Use
    Use an item from the `Items`
    */
    pub fn remove(&mut self, item:Character) -> bool{
        let one:f64 = 1.0;
        for mut i in self.items.clone() {
            let f = i.filename.clone();
            let c = match Character::read(f) {
                Some(c) => c,
                None => return false,
            };
            
            if item.name.clone() == c.name.clone() {
                if i.amount < one {
                    
                }
                // subtract one
                i.amount -= one;
                return true;
            }
        }
        false
    }
    #[allow(unused)]
    /// Get an item as a configuration file by looking up the name
    pub fn get_listitem_from_name(&self, find_item:&str) -> Option<Object> {
        for item in self.items.clone() {
            //println!("item:{:?}",item.clone());
            let i = match Character::read(item.filename.clone()) {
                Some(i) => i,
                None => continue,
            };
            if i.name == find_item {
                return Some(item)
            }
        }
        None
    }
    #[allow(unused)]
    /// Get an special as a configuration file by looking up the name
    pub fn get_special_from_name(&self, find_item:&str) -> Option<Special> {
        for item in self.items.clone() {
            //println!("item:{:?}",item.clone());
            let i = match Special::read(item.filename.clone()) {
                Some(i) => i,
                None => continue,
            };
            if i.name == find_item {
                return Some(i)
            }
        }
        None
    }
    #[allow(unused)]
    /// Get an item as a configuration file by looking up the name
    pub fn get_item_from_name(&self, find_item:&str) -> Option<Item> {
        for item in self.items.clone() {
            //println!("item:{:?}",item.clone());
            let i = match Item::read(item.filename.clone()) {
                Some(i) => i,
                None => continue,
            };
            if i.name == find_item {
                return Some(i)
            }
        }
        None
    }
    #[allow(unused)]
    /// Get an character as a configuration file by looking up the name
    pub fn get_character_from_name(&self, find_item:&str) -> Option<Character> {
        for item in self.items.clone() {
            //println!("item:{:?}",item.clone());
            let i = match Character::read(item.filename.clone()) {
                Some(i) => i,
                None => continue,
            };
            if i.name == find_item {
                return Some(i)
            }
        }
        None
    }
    #[allow(unused)]
    /// Get an item as a configuration file (no amount)
    pub fn get_character(&self, item:usize) -> Option<Character> {
        if item >= self.items.len() {
            return None;
        }
        let list_item = self.items[item].clone();
        Character::read(list_item.filename)
    }
    #[allow(unused)]
    /* Get an item from the list as an Optional `Item`
    this basically looks at the item as a toml file and loads it
    */
    pub fn get_item(&self, item:usize) -> Option<Item> {
        if item >= self.items.len() {
            return None;
        }
        let list_item = self.items[item].clone();
        Item::read(list_item.filename)
    }
    #[allow(unused)]
    /* Get an item from the list as an Optional `Item`
    this basically looks at the item as a toml file and loads it
    */
    pub fn get_special(&self, item:usize) -> Option<Special> {
        if item >= self.items.len() {
            return None;
        }
        let list_item = self.items[item].clone();
        Special::read(list_item.filename)
    }
    #[allow(unused)]
    /// Get all items as configuration files (no amount)
    pub fn get_all(&self) -> Vec<Character> {
        let mut return_vec:Vec<Character> = vec![];
        for i in self.items.clone() {
            let c = match Character::read(i.filename.clone()) {
                Some(c) => {
                    return_vec.push(c.clone());
                    c
                },
                None => continue,
            };
        }
        return_vec
    }
}
