#!/bin/bash

set -euo pipefail

feeder_cats=('linus' 'stormy' 'inga')
feeder_ips=('10.1.49.26' '10.1.219.91' '10.1.49.27')
port=6000

printf "Checking bad ones feeders\n"
for i in "${!feeder_ips[@]}"; do
  printf "\tchecking: %s (%s:%s)\n" "${feeder_cats[$i]}" "${feeder_ips[$i]}" "${port}"
  ping -c 4 "${feeder_ips[$i]}"
done
