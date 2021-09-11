/*!

*/

use serde::{Deserialize, Serialize};

/*
Default to making an `Enemy`, because there are honestly few `Hero`s in games
*/
#[derive(Clone, PartialEq, Copy, Debug, Serialize, Deserialize)]
pub enum Basic {
    /// Obviously the protagonist
    Hero,
    /// Obviously the antagonist
    Enemy,
}
impl Default for Basic {
    fn default() -> Self {
        Self::Enemy
    }
}
/*
# The "Normal" type of class

This can be used in combination with `Basic` if both sides are the same, to provide PvP, or soldier battle scenarios

To use this alone something like `Necromancer` and `Soldier` could be the enemies in that scenario, or however one sees fit.

Not all variants need to be processed, so feel free to ignore the ones you don't like and extend it via something like:

```rs:no_run
   match normal {
    Adept => (), //whatever you want
    Archer => (), //whatever you want
    // etc ...
    _=> {
        // my stuff is cool
    },
   
   }

```

Obviously you can forgo using this for enemies, by using the enemy enumerations and the same trait


*/
#[derive(Clone, PartialEq, Copy, Debug, Serialize, Deserialize)]
pub enum Normal {
    /// 
    Adept,
    /// Aw, shoot you are good!
    Archer,
    /// Generally this is the good guy in the story...
    Knight,
    /// Devoted to reading really old books and figuring out really old combinations of substances
    Monk,
    /// Into death magic, which is hard to recover from
    Necromancer,
    /// Into life magic, which overcomes death magic
    Priest,
    /// This is the default, because a lot of RPG games involve the `Soldier` in a dungeon being an enemy for whatever reason...
    Soldier,
    /// Incredibly stealthy, this rounded character can survive any environment
    Ranger,
    /// The elite female warriors from the deepest coldest winters
    Valkyrie,
}
impl Default for Normal {
    fn default() -> Self {
        Self::Soldier
    }
}
// TODO impl ClassTrait for Normal {fn get_class(){}}

//TODO lots...
pub enum Advanced {
    DND,
}

pub trait ClassTrait {
    //TODO use type to abstract which enum is used!
    fn get_class();
    
}
