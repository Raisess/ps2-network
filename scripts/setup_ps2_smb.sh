#! /usr/bin/env bash

if [ -z $1 ]; then
  echo "option not provided: start|stop"
elif [ $1 = "start" ]; then
  sudo mount /dev/sdb /media/usb0
  sudo systemctl start smb nmb
elif [ $1 = "stop" ]; then
  sudo systemctl stop smb nmb
  sudo umount /media/usb0
fi
