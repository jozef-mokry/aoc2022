#! /bin/bash
tr "ABCXYZ" "123036" | \
  awk '{count += $2; if ($2 == 3) count += $1; else if ($2 == 6) count += $1 % 3 + 1; else count += ($1 - 1 - 3) % 3 + 3; print count;}'
