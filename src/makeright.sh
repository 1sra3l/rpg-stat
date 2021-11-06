#!/bin/bash

#while read LINE
#do
#    ENUM="${LINE%%,*},"
#    COMMENT="${LINE#*,}"
#    echo "    ${COMMENT}
#    ${ENUM}"
#
#done < tmp
FILE1=tmp1.rs
FILE2=tmp2.rs
FILE3=../assets/ini/legendary.ini

# File 1
echo "impl fmt::Display for Legendary {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let v:String;
        match *self {" > "${FILE1}"

# File 2
echo "
    /// Get a short descriptive string of the \`Legendary\` creature
    pub fn short_description(&self) -> String {
        let v:String;
        match *self {" > "${FILE2}"
echo "# Legendary Creatures from Wikipedia
" > "${FILE3}"
ID=0
while read LINE
do
    ENUM="${LINE/\/*}"
    ENUM="${ENUM/,}"
    # File 1
    if [[ -n $ENUM ]]
    then
        echo "            Legendary::${ENUM} =>  v = String::from(\"${ENUM}\"),">> "${FILE1}"
    fi
    # File 2 & 3
    if [[ -n $ENUM ]] && [[ -n $ENUM2 ]]
    then
        echo "            Legendary::${ENUM2} =>  v = String::from(\"${DESCRIPTION}\")," >> "${FILE2}"
        echo "[${ENUM2}]
short_description = \"${DESCRIPTION}\"
long_description = \"\"
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
" >> "${FILE3}"
        ID=$(( ID + 1 ))
    else
        DESCRIPTION="${LINE/*\/\/\/ }"
        DESCRIPTION="${DESCRIPTION//\"/\'}"
        export DESCRIPTION
    fi
    if [[ -n $ENUM ]]
    then
        ENUM2="${ENUM}"
        export ENUM2
    fi
done < legendary_list

#File 1
echo "        }
        write!(f, \"{}\", v.as_str())
    }
}" >> "${FILE1}"

## File 2
echo "            Legendary::${ENUM2} =>  v = String::from(\"${DESCRIPTION}\")," >> "${FILE2}"
echo "        }
        // We **finally** return the string
        v
    }" >> "${FILE2}"
