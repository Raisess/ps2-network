# PS2 Network

A PS2 Store made to be used in combination with Open PS2 Loader + SMB connection.

### Disclaimer

This project is just for studies purpose and the data fetched on it is shared
by any contributor.

### Server

The server is written in Rust, it is capable of fetching games from different
providers, it also fetch cover arts and convert games with less than 700MB
from BIN/CUE to ISO.

You'll need a computer for running the server, you PS2 should connect with it
via OPL using the ETH option, for linux you can use a samba server, for windows
you can setup a file share on the network.

I highly recommend using Podman/Docker to make management of the server.

### Client

TODO
