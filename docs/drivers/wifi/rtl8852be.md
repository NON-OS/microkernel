# Realtek RTL8852BE

The RTL8852BE driver [capsule](../../overview/glossary-boot-root-record.md#capsule) (`driver.rtl8852be0`) runs the Realtek RTL8852BE PCIe Wi-Fi 6 card as a 20 MHz station on 2.4 and 5 GHz, with 802.11a/b/g rates. It is a port of Linux v6.12's rtw89 for this one chip; the station above the frames (scan, open and SAE authentication, association, the WPA2 and WPA3 handshakes, CCMP in software) is the shared `nonos_wifi_core` soft-MAC station.

## What works

Nothing has been run on the card. The driver is proven on the host only, in `userland/rtl8852_proofs`: its own bring-up runs against a modelled chip (registers, efuse, the firmware CPU's download and mailbox, the PCI DMA engine, RF channel registers) to Ready, then scans, joins WPA2 and WPA3 access points on 2.4 and 5 GHz and carries traffic both ways through the rings. One host mutant per rule and bound is in `verification/host_mutants/rtl8852be.json`, all killed. [The real-hardware checklist](../../hardware/real-hardware-checklist.md#realtek-rtl8852be-wi-fi) says how to give it its first run.

## Identity

The driver takes one PCI function: vendor 0x10EC, device 0xB852 or 0xB85B, the ids Linux v6.12 rtw89 binds the chip by (`userland/capsule_driver_rtl8852be/src/ids.rs`, `RTL8852BE_DEVICE_IDS`), with a memory BAR of at least 512 KiB; the kernel's inventory and the broker's scope row name the same two, and `kernel_proofs` holds both to the driver's list. The kernel starts the capsule only when the boot PCI scan found one of them (`src/userspace/init/spawn_plan/drivers_wifi/rtl8852be.rs`). Its service is `driver.rtl8852be0` on port 4268, reply port 4269. The RTL8852CE (10ec:c852) and RTL8852AE are listed by the inventory as Realtek Wi-Fi parts with no driver.

## Authority

The same mask as the RTL8821CE's, 0xB8038 (IPC, Memory, Crypto, Driver, DeviceEnum, Mmio, Dma), Debug optional; no Irq (the rings are polled), no FileSystem, no Network. The station address is drawn at random each boot; the efuse address is never read out.

## Stages

The bring-up steps, each with the rtw89 function it follows and the stage it reports, are in `userland/capsule_driver_rtl8852be/docs/stages.md`. Besides the shared codes, the driver reports `ChipInit` (12) for a PCIe, MAC, PHY or calibration step the chip did not finish.

## What it does not do yet

TSSI and DPK (TX power is open loop, from rtw89's world-wide tables), RSSI (frames report none), HT/VHT/HE rates, 40 and 80 MHz, 6 GHz, hardware crypto, power save (the chip stays awake), the Bluetooth coexistence engine (the antenna is granted to Wi-Fi alone), and boards with one antenna (`NoAirPath`).

## Licence

The register tables and calibration lists come from rtw89 (Linux v6.12), taken under its BSD-3-Clause option, with Realtek's notice beside them (`userland/capsule_driver_rtl8852be/tables/LICENSE`). The firmware, `rtw8852b_fw-1.bin` from linux-firmware, is Realtek's redistributable binary, pinned by sha256 (`userland/capsule_driver_rtl8852be/docs/licence.md`).
