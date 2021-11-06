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
echo "impl fmt::Display for Legendary {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let v:String;
        match *self {" > "${FILE1}"
while read LL
do
    L="${LL/\/*}"
    if [[ -n $L ]]
    then
        L="${L/,}"
        echo "            Legendary::${L} =>  v = String::from(\"${L}\"),"
    fi
done < legendary_list >> "${FILE1}"

echo "        }
        write!(f, \"{}\", v.as_str())
    }
}" >> "${FILE1}"
echo "
    /// Get a short descriptive string of the \`Legendary\` creature
    pub fn short_description(&self) -> String {
        let v:String;
        match *self {" > "${FILE2}"
while read LINE
do
    ENUM="${LINE/\/*}"
    ENUM="${ENUM/,}"
    if [[ -n $ENUM ]] && [[ -n $ENUM2 ]]
    then
        echo "            Legendary::${ENUM2} =>  v = String::from(\"${DESCRIPTION}\")," >> "${FILE2}"
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
echo "            Legendary::${ENUM2} =>  v = String::from(\"${DESCRIPTION}\")," >> "${FILE2}"
echo "        }
        // We **finally** return the string
        v
    }" >> "${FILE2}"
