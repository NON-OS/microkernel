# USB hubs

What NONOS does with a USB hub in this release: the hub comes up, but the devices plugged into it are not reached yet.

## In short

Plug keyboards, mice and USB sticks straight into a port of the machine. A device behind an external USB hub does not work in NONOS 0.9.2. The hub itself is brought up and its ports are read, but the xHCI driver cannot yet address a device below a hub.
