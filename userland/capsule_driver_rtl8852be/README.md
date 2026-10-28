# capsule_driver_rtl8852be

## Role

`capsule_driver_rtl8852be` is the Realtek RTL8852BE PCIe Wi-Fi hardware
capsule (PCI 10ec:b852 and 10ec:b85b, the ids of Linux v6.12
`rtw89/rtw8852be.c`). It owns the device claim, the BAR mapping, the DMA
rings, the firmware download, the MAC, BB and RF bring-up and calibration,
channel switching and the frames in and out for `driver.rtl8852be0`.

```text
net.core, Settings, setup wizard (the only senders the kernel admits)
 |
 v
driver.rtl8852be0 -- brokered MMIO/DMA, polled --> Realtek RTL8852BE PCIe device
```

It is a port of Linux v6.12's `drivers/net/wireless/realtek/rtw89/` for
this one chip, run as a 20 MHz legacy (802.11a/b/g rate) station. The
station above the frames is the shared `nonos_wifi_core` soft-MAC station
(`SoftStation`): scanning, authentication (open and WPA3-SAE),
association, the WPA2 and WPA3 handshakes and the receive checks;
CCMP-128 runs in the chip on keys the station installs, as rtw89 installs
them (the station writes each data frame's CCMP header). IP, DHCP, DNS and sockets stay in the upper network
capsules. The link protocol, which puts raw frames on the air, is served
only to the process that owns `net.core` (`src/serve.rs`,
`nonos_wifi_core::netif::gate`).

The chip is driven with no interrupt: the rings are polled between
requests, once every 100 ms when none comes.

## What it does not do

TSSI and DPK are not ported: TX power is open loop, from rtw89's
world-wide regulation tables (`tables/gen_txpwr.py`). No HT, VHT or HE
rates, no 40/80 MHz channels, no 6 GHz, no hardware crypto, no power
save (the chip stays awake; no PS command is ever sent), no Bluetooth
coexistence engine (the antenna is granted to Wi-Fi alone), no RSSI
reading (frames report 0, "no reading"), and no single-antenna board
variant (it stops at `NoAirPath`).

## Parts

- [Contract, authority, privacy and the surface](docs/contract.md)
- [Lifecycle, failures and release](docs/lifecycle.md)
- [The bring-up, stage by stage, with the Linux functions](docs/stages.md)
- [Firmware and table licences](docs/licence.md)
- [Verification: the host proofs and their mutants](docs/verification.md)
- [Real hardware bring-up, and what is not a goal](docs/hardware.md)
- [The firmware file](firmware/README.md)
- [The generated tables](tables/README.md)
