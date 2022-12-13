#! /bin/bash
tr "ABCXYZ" "123123" | \
  awk '{count += $2; if ($1 == $2) count+=3; else if (($2 - $1 + 3) % 3 == 1) count += 6} END {print count}'
