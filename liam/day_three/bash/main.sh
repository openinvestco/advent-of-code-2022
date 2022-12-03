#! /bin/bash

part_one() {
    input="../input.txt"
    total=0
    for line in $(cat $input)
    do 
        # Find half length of full string
        half=$((`echo "$line" | tr -d "\n" | wc -c` /2));
        c=0
        # Iterate through first half of string
        for (( i=0; i<${half}; i++ ))
        do
            # Check if char in second half, if so, collect (always gets last one)
            if [[ "${line:half:half}" == *"${line:i:1}"* ]] 
            then
                c="${line:i:1}"
            fi
        done
        # Convert to byte offset of a-zA-z (offset by 1) and adds to total
        x=$((`echo {"!",{a..z},{A..Z}} | tr -d "[:space:]" | grep -b -o $c | cut -d: -f1`))
        total=$((total + $x))
    done < "$input"
    echo $total
}

part_two() {
    input="../input.txt"
    total=0
    lines=$((`wc -l <$input`+1))
    for (( i=1; i<$lines; i += 3 ))
    do 
        l1=`sed "${i}q;d" $input`
        l2=`sed "$((i+1))q;d" $input`
        l3=`sed "$((i+2))q;d" $input`
        # Find length of full first string
        len=$((`echo "$l1" | tr -d "\n" | wc -c`));
        c=0
        # Iterate through first string
        for (( j=0; j<${len}; j++ ))
        do
            # Check if char in second and third strings, if so, collect (always gets last one)
            if [[ "$l2" == *"${l1:$j:1}"* ]] && [[ "$l3" == *"${l1:$j:1}"* ]] 
            then
                c="${l1:$j:1}"
            fi
        done
        # Convert to byte offset of a-zA-z (offset by 1) and adds to total
        x=$((`echo {"!",{a..z},{A..Z}} | tr -d "[:space:]" | grep -b -o $c | cut -d: -f1`))
        total=$((total + $x))
    done < "$input"
    echo $total
}



part_one
part_two