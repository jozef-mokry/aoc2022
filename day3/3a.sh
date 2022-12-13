#! /bin/bash
X="abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
while read line; do half=$((${#line}/2)); first=${line:0:$half};tr -c -d -s $first $first  <<< ${line:$half}; echo;  done | \
  awk -v x="$X" '{print $0; ans+=index(x, $0);} END {print ans}'
