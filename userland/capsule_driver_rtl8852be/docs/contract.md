# Contract, authority, privacy and the surface

Part of [capsule_driver_rtl8852be](../README.md).

## Microkernel contract

The capsule reaches its card only through the broker: `MkDeviceList` to find
10ec:b852 or 10ec:b85b, `MkDeviceClaim` for it, `MkMmioMap` for its first
memory BAR (BAR 2 on this card, at least 512 KiB), the PCI command register
for Memory Space and, only once a MAC a warm reboot left on is powered off,
Bus Master; `MkDmaMap` for coherent grants below 4 GiB (the rings and their
slots). No `MkIrqBind`: every ring is polled. Requests arrive with
`MkIpcRecvFrom` on the service endpoint and are answered with `MkIpcReply`;

## Interface contract

`driver.rtl8852be0` on port 4268, reply port 4269. It answers the Wi-Fi
control family (status, scan, connect, disconnect, link, radio) that
`nonos_wifi_client` sends, and net.core's link protocol, both through the
shared `SoftStation` (`src/serve.rs`). The status reply's first byte is the
bring-up stage (`src/stage.rs`), the numbers `nonos_wifi_client::DriverStage`
reads, and its five diagnostic words are the radio's (`src/radio/link.rs`,
`words`).

## Authority

`CAPSULE_REQUIRED_CAPS` is 0xB8038: IPC, Memory, Crypto, Driver, DeviceEnum,
Mmio and Dma; Debug (0x100) is optional (`Capsule.mk`). Crypto draws the
station address and the handshake nonces. No Irq, no FileSystem, no Network.

## Privacy and persistence

The station address is drawn at each start, locally administered and
unicast; the efuse's address is never read out of the map. No line the
capsule prints carries an address, an SSID or a BSSID, and every line is
written whole (held by `rtl8852_proofs`' `privacy_tests`). Nothing is
written anywhere: the passphrase arrives with each connect request and
stays in the shared station's memory.

## Current implemented surface

Bring-up to a station interface; channel switching on 2.4 GHz (1-14) and
5 GHz (36-177) at 20 MHz; scanning, WPA2 and WPA3 joins and data with
software CCMP through the shared station (all proven on the host);
management frames at a fixed basic rate, data with the firmware's rate
adaptation over the AP's legacy rates.

## Wire format

Firmware commands are H2C packets on CH12 (a 24-byte WD body, an 8-byte H2C
header); the register mailbox (`R_AX_H2CREG_*`) carries GET_FEATURE and
SCH_TX_EN. Frames go out on ACH0 (data) and CH8 (management) as a WD page
(48-byte WD, WP info, address info) and the frame in one 4 KiB slot; they
come in on RXQ behind the RX BD info word and the RX descriptor, and the
release reports come back on RPQ.

## State ownership

The capsule owns its claim, its grants and the chip's state; the shared
station owns the scan list, the session and the keys; net.core owns the
interface above the link.

## Operating rules

A frame slot is reused only after its release report. A ring index past
the ring, or a register reading all ones, is never believed. Every wait
ends after rtw89's own timeout. 6 GHz is never tuned. Power save is never
asked of the firmware.
