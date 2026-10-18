# capsule_driver_rtl8821ce

## Role

`capsule_driver_rtl8821ce` is the Realtek RTL8821CE PCIe Wi-Fi hardware capsule.
It owns the Realtek wireless device claim, BAR mapping, DMA rings, firmware
download, the MAC/PHY/RF bring-up, scanning and joining for
`driver.rtl8821ce0`.

```text
net.core, Settings, setup wizard (the only senders the kernel admits)
 |
 v
driver.rtl8821ce0 -- brokered MMIO/DMA, polled --> Realtek RTL8821CE PCIe device
```

The link protocol, which puts raw frames on the air, is served only to the
process that owns `net.core`, as the registry names it on each request;
Settings and the wizard get the Wi-Fi control family (scan, join, status),
and a link request from them is refused (EPERM) (`src/serve/answer.rs`,
`nonos_wifi_core::netif::gate`).

The 802.11 management, the WPA3-SAE exchange (group 19, hash-to-element and
hunting-and-pecking), the WPA2 four-way and group key handshakes, key
derivation, CCMP and the receive checks run in the shared `nonos_wifi_core`
crate; IP, DHCP, DNS, and sockets stay in the upper network capsules. This
capsule drives the radio and moves frames; it does not own network profiles or
credentials.

Transmit power rests on two assumptions until the efuse power-by-rate tables
are ported: an antenna gain of 2 dBi, and a power index counted in 0.5 dB
steps of output. On them the index 0x24 is 18 dBm, the world domain's 20 dBm
EIRP less the gain (`src/phy/txpower.rs`, `band_index`).

## Parts

- [The microkernel contract](docs/microkernel.md)
- [The interface contract](docs/interface.md)
- [Firmware, authority, privacy and persistence](docs/firmware.md)
- [The runtime lifecycle and the failure model](docs/lifecycle.md)
- [The implemented surface, wire format, state and rules](docs/surface.md)
- [Release target, evidence and checklist](docs/release.md)
- [Real hardware bring-up, and what is not a goal](docs/hardware.md)
- [Verification](docs/verification.md)

