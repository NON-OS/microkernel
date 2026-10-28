# capsule_driver_mt7921

## Role

`capsule_driver_mt7921` is the MediaTek MT7921, MT7922 and MT7920 PCIe Wi-Fi
hardware capsule (14c3:7961, 7922, 0608, 0616, 7920; 0b48:7922), written
from Linux v6.12's mt76 (ISC). It owns the device claim, the BAR mapping,
the WFDMA rings, the firmware download, the MCU's bring-up, the channel
requests and the frame descriptors for `driver.mt7921e0`.

```text
net.core, Settings, setup wizard (the only senders the kernel admits)
 |
 v
driver.mt7921e0 (shared soft-MAC station over this radio)
 |   brokered MMIO/DMA, polled            MkInstallSource (kinds 0x20..0x25)
 v                                         ^
MediaTek MT7921/MT7922 PCIe device         loader: EFI/nonos/firmware/mediatek
```

The 802.11 management, SAE and the WPA2 handshakes and the receive checks
run in the shared `nonos_wifi_core` crate; CCMP-128 runs in the chip, on the
keys the station installs once a join or a rekey is done, as Linux's
`mt7921_set_key` installs them. IP and above stay in the upper network
capsules. This capsule drives the radio and moves frames; it holds no
profile and no credential, and keeps no key past the command that carries
it. Host-proven only: no card has run it.

The card's firmware is never in this capsule: the loader reads the pinned
package from the boot medium and the driver refuses any file whose sha256 is
not the one in `src/fw/pins.rs`.

The stage map with its Linux citations, the firmware package and what is
not done: [docs/drivers/wifi/mt7921.md](../../docs/drivers/wifi/mt7921.md).

## Parts

- [Contract, authority, privacy and the surface](docs/contract.md)
- [Lifecycle, failures, release and verification](docs/release.md)

## Source

- `src/hw`: the register window (fixed map, L1 remap).
- `src/power`: ownership, identity, the WFSYS reset.
- `src/dma`: grants, descriptors, the six rings, the WFDMA.
- `src/mcu`: command frames, events, the send and wait.
- `src/fw`: the package's pins, its read and check, patch and RAM download.
- `src/init`: capabilities, efuse mode, MAC enable, channel domain, RX path.
- `src/vif`: the interface, BSS, station records, channel requests, filter.
- `src/mac`: TXD/TXP, RXD, tokens, transmit-free reports, rates.
- `src/radio`: the `Radio` the shared station drives.
- `src/bringup`, `src/serve`: the stages in order, and the service loop.
