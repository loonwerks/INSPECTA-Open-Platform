#!/bin/sh
set -eu

image=$1
image_size=$2
dockardu_tar=$3
compose_template=$4
qgc_host_ip=$5
start_script=$6

staging="${image}.staging"
temporary="${image}.tmp"

cleanup()
{
    rm -rf "${staging}" "${temporary}"
}
trap cleanup EXIT INT TERM

mkdir -p "${staging}/docker/containers" "${staging}/docker_data"
cp "${dockardu_tar}" "${staging}/docker/containers/dockardu.tar"
install -m 0755 "${start_script}" "${staging}/docker/startArdu.sh"
sed "s/@QGC_HOST_IP@/${qgc_host_ip}/g" "${compose_template}" \
    > "${staging}/docker/docker-compose.yml"

truncate -s "${image_size}" "${temporary}"
mkfs.ext4 -F -m 0 -L INSPECTA_DATA -d "${staging}" "${temporary}"
mv "${temporary}" "${image}"
