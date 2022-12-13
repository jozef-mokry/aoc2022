#! /bin/bash
X="abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
idx=0
a=""
(while read line; do 
  if [[ $((idx%3)) -eq 0 ]]; then
    [[ $a != "" ]] && echo $a;
    a=$line;
  else 
    a=$(tr -d -c -s $line $a <<< $a);
  fi
  ((idx++));
done; echo $a) | awk -v x="$X" '{print $0; ans+=index(x, $0);} END {print ans}'
