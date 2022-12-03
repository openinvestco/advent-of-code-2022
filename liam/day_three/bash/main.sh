#! /bin/bash

part_one() {
    input="../test_input.txt"
    total=0
    for line in $(cat $input)
    do 
        half=$((`echo "$line" | tr -d "\n" | wc -c` /2));
        c=0
        for (( i=0; i<${half}; i++ ))
        do
            if [[ "${line:half:half}" == *"${line:i:1}"* ]] 
            then
                c="${line:i:1}"
            fi
        done
        x=$((`echo {"!",{a..z},{A..Z}} | tr -d "[:space:]" | grep -b -o $c | cut -d: -f1`))
        total=$((total + $x))
    done < "$input"
    echo $total
}
part_one