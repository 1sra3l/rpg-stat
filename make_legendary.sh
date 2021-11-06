#!/bin/bash

FILE1=tmp1.rs
FILE2=tmp2.rs
FILE3=tmp3.rs

INI_FILE="assets/ini/legendary.ini"
OUTPUT="src/legendary.rs"
echo "/*!
# Legendary Creatures

A huge set of creatures from around the world.  This can be iterated through, and will hopefully 

*/
extern crate num;
use num::NumCast;
use std::fmt;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use crate::stats::{ Builder, Basic, Normal, Advanced };

use std::ops::{Add, AddAssign,  Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign};
/*
This is public information from [Wikipedia](https://en.wikipedia.org/wiki/Lists_of_legendary_creatures)

The following is a list of lists of legendary creatures, beings and entities from the folklore record. Entries consist of legendary and unique creatures, not of particularly unique individuals of a commonly known species.

*/
#[derive(Clone, PartialEq, Copy, Debug, EnumIter)]//, Serialize, Deserialize)]
pub enum Legendary {" > "${OUTPUT}"

# File 1
echo "impl fmt::Display for Legendary {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let v:String;
        match *self {" > "${FILE1}"

# File 2
echo "impl Legendary {
    /// Get a short descriptive string of the \`Legendary\` creature
    pub fn short_description(&self) -> String {
        let v:String;
        match *self {" > "${FILE2}"

# File 3
echo "    /// Get a long descriptive string of the \`Legendary\` creature
    pub fn long_description(&self) -> String {
        let v:String;
        match *self {" > "${FILE3}"

# INI file
echo "# Legendary Creatures from Wikipedia
" > "${INI_FILE}"
# increment ID for legendary creature ini
ID=0
unset FIRST_TIME
# read in the info
while read LINE
do
    ENUM="${LINE/\/*}"
    # File 1
    if [[ -n $ENUM ]]
    then
        ENUM="${ENUM/\[*}"
        echo "            Legendary::${ENUM} =>  v = String::from(\"${ENUM}\"),">> "${FILE1}"
        echo -n "    $ENUM," >> "${OUTPUT}"
        LONG_DESC="${LINE/\]}"
        LONG_DESC="${LONG_DESC/*\[}"
        #LONG_DESC="${LONG_DESC//}"
    else
        echo -n "    $LINE" >> "${OUTPUT}"
    fi
    # File 2 & INI
    if [[ -n $ENUM ]] && [[ -z $FIRST_TIME ]]
    then
        FIRST_TIME="DONE"
        [[ -z $REAL_NAME ]] && REAL_NAME="${ENUM}"
        [[ -z ${LONG_DESC} ]] && LONG_DESC="${REAL_NAME}${DESCRIPTION}"
        # File 2
        echo "            Legendary::${ENUM} =>  v = String::from(\"${REAL_NAME}${DESCRIPTION}\")," >> "${FILE2}"
        # File 3
        echo "            Legendary::${ENUM} =>  v = String::from(\"${LONG_DESC}\")," >> "${FILE3}"

        # INI file
        echo "[${ENUM}]
name = \"${REAL_NAME}\"
short_description = \"${REAL_NAME}${DESCRIPTION}\"
long_description = \"${LONG_DESC}\"
id = $ID
hp = 10
mp = 10
xp = 10
hp_max = 10
mp_max = 10
xp_next = 10
gp = 10
speed = 10
atk = 10
def = 10
m_atk = 10
m_def = 10
agi = 10
str = 10
int = 10
dex = 10
con = 10
char = 10
wis = 10
age = 10
" >> "${INI_FILE}"
        # increment ID
        ID=$(( ID + 1 ))
    fi

    # File 2 & INI
    if [[ -n $ENUM ]] && [[ -n $ENUM2 ]]
    then
        [[ -z $REAL_NAME ]] && REAL_NAME="${ENUM}"
        [[ -z ${LONG_DESC} ]] && LONG_DESC="${REAL_NAME}${DESCRIPTION}"
        # File 2
        echo "            Legendary::${ENUM} =>  v = String::from(\"${REAL_NAME}${DESCRIPTION}\")," >> "${FILE2}"
        # File 3
        echo "            Legendary::${ENUM} =>  v = String::from(\"${LONG_DESC}\")," >> "${FILE3}"

        # INI file
        echo "[${ENUM}]
name = \"${REAL_NAME}\"
short_description = \"${REAL_NAME}${DESCRIPTION}\"
long_description = \"${LONG_DESC}\"
id = $ID
hp = 10
mp = 10
xp = 10
hp_max = 10
mp_max = 10
xp_next = 10
gp = 10
speed = 10
atk = 10
def = 10
m_atk = 10
m_def = 10
agi = 10
str = 10
int = 10
dex = 10
con = 10
char = 10
wis = 10
age = 10
" >> "${INI_FILE}"
        # increment ID
        ID=$(( ID + 1 ))
    else
        DESCRIPTION="${LINE/*\/\/\/ }"
        DESCRIPTION="${DESCRIPTION//\"/\'}"
        REAL_NAME="${DESCRIPTION/\]*}"
        DESCRIPTION="${DESCRIPTION/*\)}"
        #DESCRIPTION="${DESCRIPTION/\[}"
        #DESCRIPTION="${DESCRIPTION/\]}"
        REAL_NAME="${REAL_NAME/\[}"
        export DESCRIPTION
        #echo $DESCRIPTION
    fi
    if [[ -n $ENUM ]]
    then
        ENUM2="${ENUM}"
        #echo $ENUM2
        export ENUM2
    fi
    echo >> "${OUTPUT}"
done < legendary_list
echo "}" >> "${OUTPUT}"
#File 1
echo "        }
        write!(f, \"{}\", v.as_str())
    }
}" >> "${FILE1}"

## File 2
echo "        }
        // We **finally** return the string
        v
    }">> "${FILE2}"

echo "        }
        // We **finally** return the *looooooong* string
        v
    }
}" >> "${FILE3}"

cp "${OUTPUT}" "${OUTPUT}.tmp"
cat "${OUTPUT}.tmp" "${FILE1}" "${FILE2}" "${FILE3}" > "${OUTPUT}"
rm "${OUTPUT}.tmp" "${FILE1}" "${FILE2}" "${FILE3}"

echo "impl<T:Copy 
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
    + num::NumCast> Builder<T> for Legendary {
    /// Build a \`Basic\` stat
    fn build_basic(&self, id:T, level:T) -> Basic<T>{
        let mut hp:T = num::cast(10).unwrap();
        let mut mp:T = num::cast(5).unwrap();
        let mut xp:T = num::cast(1).unwrap();
        let mut xp_next:T = num::cast(10).unwrap();
        let mut gp:T = num::cast(5).unwrap();
        let mut speed:T = num::cast(5).unwrap();
        //TODO OR ue legendary.ini + serde
        match *self {
            _=> {},
        }
        hp *= level;
        mp *= level;
        // TODO fixme:
        xp *= level;
        // TODO fixme:
        xp_next *= level;
        gp *= level;
        speed += level;
        Basic {
            id:id,
            xp:xp,
            xp_next:xp_next,
            level:level,
            gp:gp,
            hp: hp,
            mp: mp,
            hp_max: hp,
            mp_max: mp,
            speed: speed,
        }
        
    }
    // Build a \`Normal\` stat
    fn build_normal(&self, id:T, level:T) -> Normal<T>{
        let mut hp:T = num::cast(10).unwrap();
        let mut mp:T = num::cast(5).unwrap();
        let mut xp:T = num::cast(1).unwrap();
        let mut xp_next:T = num::cast(10).unwrap();
        let mut gp:T = num::cast(5).unwrap();
        let mut speed:T = num::cast(5).unwrap();
        let mut atk:T = num::cast(10).unwrap();
        let mut def:T = num::cast(10).unwrap();
        let mut m_atk:T = num::cast(10).unwrap();
        let mut m_def:T = num::cast(10).unwrap();
        //TODO OR use legendary.ini + serde
        match *self {
            _=> {},
        }
        hp *= level;
        mp *= level;
        // TODO fixme:
        xp *= level;
        // TODO fixme:
        xp_next *= level;
        gp *= level;
        speed += level;
        Normal {
            id:id,
            xp:xp,
            xp_next:xp_next,
            level:level,
            gp:gp,
            hp: hp,
            mp: mp,
            hp_max: hp,
            mp_max: mp,
            speed: speed,
            atk:atk,
            def:def,
            m_atk:m_atk,
            m_def:m_def,
        }
    }

    // Build an \`Advanced\` stat
    fn build_advanced(&self, id:T, level:T) -> Advanced<T>{
        let mut hp:T = num::cast(10).unwrap();
        let mut mp:T = num::cast(5).unwrap();
        let mut xp:T = num::cast(1).unwrap();
        let mut xp_next:T = num::cast(10).unwrap();
        let mut gp:T = num::cast(5).unwrap();
        let mut speed:T = num::cast(5).unwrap();
        let mut atk:T = num::cast(10).unwrap();
        let mut def:T = num::cast(10).unwrap();
        let mut m_atk:T = num::cast(10).unwrap();
        let mut m_def:T = num::cast(10).unwrap();
        let mut agility:T = num::cast(10).unwrap();
        let mut strength:T = num::cast(10).unwrap();
        let mut dexterity:T = num::cast(10).unwrap();
        let mut constitution:T = num::cast(10).unwrap();
        let mut intelligence:T = num::cast(10).unwrap();
        let mut charisma:T = num::cast(10).unwrap();
        let mut wisdom:T = num::cast(10).unwrap();
        let mut age:T = num::cast(10).unwrap();
        //TODO OR use legendary.ini + serde
        match *self {
            _=> {},
        }
        hp *= level;
        mp *= level;
        // TODO fixme:
        xp *= level;
        // TODO fixme:
        xp_next *= level;
        gp *= level;
        speed += level;
        Advanced {
            id:id,
            xp:xp,
            xp_next:xp_next,
            level:level,
            gp:gp,
            hp: hp,
            mp: mp,
            hp_max: hp,
            mp_max: mp,
            speed: speed,
            atk:atk,
            def:def,
            m_atk:m_atk,
            m_def:m_def,
            agility:agility,
            strength:strength,
            dexterity:dexterity,
            constitution:constitution,
            intelligence:intelligence,
            charisma:charisma,
            wisdom:wisdom,
            age:age,
        }
    }
}" >> "${OUTPUT}"
