#!/bin/bash

set -euo pipefail

feeder_cats=('linus' 'stormy' 'inga')
feeder_ips=('10.1.49.26' '10.1.219.91' '10.1.49.27')
port=6000

printf "Feeding bad ones\n"
for i in "${!feeder_ips[@]}"; do
	printf "\tfeeding: %s (%s:%s)\n" "${feeder_cats[$i]}" "${feeder_ips[$i]}" "${port}"
	printf "Be fed %s" "${feeder_cats[$i]}" > "/dev/udp/${feeder_ips[$i]}/${port}"
done
