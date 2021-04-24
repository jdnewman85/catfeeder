#!/bin/bash

set -euo pipefail

#feeder_cats=('linus' 'stormy' 'inga')
#feeder_ips=('10.1.49.26' '10.1.219.91' '10.1.49.27')
feeder_cats=('linus' 'stormy' 'inga')
feeder_ips=('10.1.49.26' '10.1.219.91' '10.1.49.27')

printf "Checking feeders..."
for f in "${!feeder_ips[@]}"; do
  printf "\t\tchecking: %s (%s)\n" "${feeder_cats[$f]}" "${feeder_ips[$f]}"
  ping -q -c 1 ${feeder_ips[$f]}
done
printf "All feeders up!\n"
