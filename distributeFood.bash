#!/bin/bash

set -euo pipefail

#feeder_cats=('linus' 'stormy' 'inga')
#feeder_ips=('10.1.49.26' '10.1.219.91' '10.1.49.27')
feeder_cats=('linus' 'stormy' 'inga')
feeder_ips=('10.1.49.26' '10.1.219.91' '10.1.49.27')
portions=1
delay=0 #s
port=6000

cd "$(dirname "$0")"
./checkFeeders.bash

printf "Feeding bad ones\n"
for ((i=0; i<portions; i++)); do
  ((i_plus_one=i+1))
  printf "\tround %d/%d\n" "${i_plus_one}" "${portions}"
  for f in "${!feeder_ips[@]}"; do
    printf "\t\tfeeding: %s (%s:%s)\n" "${feeder_cats[$f]}" "${feeder_ips[$f]}" "${port}"
    printf "Be fed %s" "${feeder_cats[$f]}" > "/dev/udp/${feeder_ips[$f]}/${port}"
  done
  printf "\tsleeping %d...\n" "${delay}"
  sleep "${delay}"
done
printf "Bad ones fed!\n"
