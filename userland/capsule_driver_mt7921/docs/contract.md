# Contract, authority, privacy and the surface

Part of [capsule_driver_mt7921](../README.md).

## Microkernel contract

The capsule reaches its card only through the broker: `MkDeviceList` to find
a supported function, `MkDeviceClaim` for it, `MkMmioMap` for BAR0 (at least
1 MiB), the PCI command register for Memory Space and, only once the rings
are armed, Bus Master; `MkDmaMap` for coherent grants below 4 GiB (rings,
buffers, the download staging). No `MkIrqBind`: every ring is polled. The
firmware files are read with `MkInstallSource`, kinds 0x20 to 0x25, which the
loader recorded from the boot medium. Requests arrive with `MkIpcRecvFrom` on
the service endpoint and are answered with `MkIpcReply`.

## Interface contract

`driver.mt7921e0` on port 4264, reply port 4265. It answers the Wi-Fi
control family (status, scan, connect, disconnect, link, radio) that
`nonos_wifi_client` sends, and net.core's link protocol, both through the
shared `SoftStation`. The status reply's first byte is the bring-up stage
(`src/stage.rs`), the numbers `nonos_wifi_client::DriverStage` reads.

## Authority

`CAPSULE_REQUIRED_CAPS` is 0xB8038: IPC, Memory, Crypto, Driver, DeviceEnum,
Mmio and Dma; Debug (0x100) is optional (`Capsule.mk`). Crypto draws the
station address and the handshake nonces; DeviceEnum admits both the device
list and `MkInstallSource`. No Irq, no FileSystem, no Network.

## Privacy and persistence

The station address is drawn at each start, locally administered; the card's
own address is never taken (its capability reply is read for the PHY alone). No
line the capsule prints carries an address, an SSID or a BSSID (held by
`mt7921_proofs`' `serve_tests`). Nothing is written anywhere: the passphrase
arrives with each connect request and stays in the shared station's memory.

## Current implemented surface

Bring-up to a scanning station; passive scan of 2.4 and 5 GHz by channel
requests; WPA2 and WPA3 joins through the shared station (WPA2 proven on the
host); data both ways with software CCMP; fixed-rate transmit.

## Wire format

Commands are connac2 MCU frames on ring 17 (64-byte TXD, UNI commands with
a 48-byte header), the download on ring 16; events come back on rings 0 and
4. Frames go out on ring 0 with a 64-byte TXWI and TXP, and come in on ring
2 with their RXD; transmit-free reports on ring 2 return the tokens.

## State ownership

The capsule owns its claim, its grants and the radio's state; the shared
station owns the scan list, the session and the keys; net.core owns the
interface above the link.

## Operating rules

A file of the package that is not the pinned one is never sent to the card.
A ring index past the ring is never believed. A grant for another request's
token is not a grant. The address is never changed under an association.
6 GHz is never tuned.
