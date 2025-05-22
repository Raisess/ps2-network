#! /usr/bin/env sh

podman build -t ps2-network-server .
podman container create \
  -p 8080:8080 \
  -e SOURCE_PATH=/media/source \
  -e TARGET_PATH=/media/target \
  -v $(echo $PS2S_SOURCE_PATH):/media/source \
  -v $(echo $PS2S_TARGET_PATH):/media/target \
  --name ps2-network-server \
  ps2-network-server
