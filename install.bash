#!/bin/bash

install_files='catfeeder'
install_dir='/opt/catfeeder/'
service_filename='catfeeder.service'
service_install_dir='/etc/systemd/system/'

mkdir -p "${install_dir}"
systemctl stop "${service_filename}"
cp -r ${install_files} "${install_dir}"
cp "${service_filename}" "${service_install_dir}"
systemctl enable "${service_filename}"
systemctl start "${service_filename}"
