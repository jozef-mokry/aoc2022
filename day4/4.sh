#! /bin/bash
sed -E 's/^(.*)-(.*),(.*)-(.*)$/\1 \2 \3 \4/' | awk '{if (($1 <= $3 && $2 >= $4) || ($3 <= $1 && $4 >= $2)) count+=1; print count;}'