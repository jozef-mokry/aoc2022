#! /bin/bash
sed -E 's/^(.*)-(.*),(.*)-(.*)$/\1 \2 \3 \4/' | awk '{s=$1 > $3 ? $1 : $3; e=$2 < $4 ? $2 : $4;if (s <= e) count+=1; print count;}'
