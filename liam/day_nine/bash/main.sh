#! /bin/bash
input="../test_input.txt"
while read -r line
do 
    echo "$line"
done < "$input"
