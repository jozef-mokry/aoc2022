#! /bin/bash
awk 'BEGIN { count = 0; } { if ($0 == "") { print count; count = 0; } else { count += $1 } } END { print count}' | sort -nr | head -n 3 | awk '{ count += $0 } END { print count}'
