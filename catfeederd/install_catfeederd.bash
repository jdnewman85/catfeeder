#!/bin/bash

set -euo pipefail

install_files=('distribute_food.bash' 'check_feeders.bash')
install_dir='/opt/catfeederd/'

service_filename='catfeederd.service'
service_install_dir='/etc/systemd/system/'
service_timer_filename='catfeederd.timer'

systemctl stop "${service_filename}" || true
systemctl stop "${service_timer_filename}" || true

#Files
mkdir -p "${install_dir}"
cp -r ${install_files[@]} "${install_dir}"

#Services
cp "${service_filename}" "${service_timer_filename}" "${service_install_dir}"
#systemctl enable "${service_filename}"
systemctl enable "${service_timer_filename}"
#systemctl start "${service_filename}"
systemctl start "${service_timer_filename}"
