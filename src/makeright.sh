#!/bin/bash

#while read LINE
#do
#    ENUM="${LINE%%,*},"
#    COMMENT="${LINE#*,}"
#    echo "    ${COMMENT}
#    ${ENUM}"
#
#done < tmp

echo "impl fmt::Display for Legendary {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let v:String;
        match *self {" > tmp2
while read LL
do
    L="${LL/\/*}"
    if [[ -n $L ]]
    then
        echo "            Legendary::${L} =>  v = String::from(\"${L}\"),"
    fi
done < legendary_list >> tmp2

echo "        }
        write!(f, \"{}\", v.as_str())
    }
}" >> tmp2
