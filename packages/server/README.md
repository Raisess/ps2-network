# PS2 Network Server

This is the server package, to be runned on the host (Computer with Samba server).

### Running the container

You can use the `container.sh` script to easily setup the container using `podman`:

```shell
PS2S_SOURCE_PATH=/home/<user>/Downloads \
PS2S_TARGET_PATH=/media/usb0 \
    sudo -E ./container.sh
```

- `PS2S_SOURCE_PATH`: Where the games are store before installing on your external device (HDD, SDD),
it also can be path on the host if are not using any external disk driver.
- `PS2S_TARGET_PATH`: Where the games are installed, the directory that you sharing with the PS2.

### Running without container

This command you build the project and run it using the path you provided:

```shell
SOURCE_PATH=/home/<user>/Downloads TARGET_PATH=/media/usb0 cargo run
```
