# Realtek RTL8822CE

The RTL8822CE driver [capsule](../../overview/glossary-boot-root-record.md#capsule) (`driver.rtl8822ce0`) runs the Wi-Fi function of the Realtek RTL8822CE PCIe card on 2.4 GHz, the same way the [RTL8821CE driver](rtl8821ce.md) does: it is that driver with the chip's own parts replaced, and shares its scanning, joining, security and link code file for file.

## What works

Nothing has been run on the card. The driver is proven on the host only, against a modelled register file, in `userland/rtl8822ce_proofs`, with one host mutant per rule (`verification/host_mutants/rtl8822ce.json`). [The real-hardware checklist](../../hardware/real-hardware-checklist.md#realtek-rtl8822ce-wi-fi) says how to give it its first run.

## Identity

The driver takes one PCI function: vendor 0x10EC, device 0xC822 or 0xC82F, the two ids Linux v6.12 rtw88 binds the chip by (`userland/capsule_driver_rtl8822ce/src/constants/pci.rs`, `RTL8822CE_DEVICE_IDS`); the kernel's inventory and the broker's scope row name the same two, and `kernel_proofs` holds the inventory to the driver's list. The kernel starts the capsule only when the boot PCI scan found one of them (`src/userspace/init/spawn_plan/drivers_wifi/rtl8822ce.rs`). Its service is `driver.rtl8822ce0` on port 4264, reply port 4265.

## Authority

The same mask as the RTL8821CE's, 0xB8038 (IPC, Memory, Crypto, Driver, DeviceEnum, Mmio, Dma), Debug optional; no Irq, no FileSystem, no Network. The station address is drawn at random each boot; the efuse address is never read out.

## What it does not do yet

IQK, DPK, TX gain calibration, DIG, thermal power tracking, the power-by-rate and limit tables, and 5 GHz. The DAC calibration and the factory trims do run. The full list, and how the chip differs from the RTL8821CE, is in `userland/capsule_driver_rtl8822ce/docs/differences.md`.

## Licence

The register values and the PHY tables come from rtw88 (Linux v6.12), taken under its BSD-3-Clause option; the tables carry Realtek's notice (`userland/capsule_driver_rtl8822ce/README.md`). The firmware, `rtw8822c_fw.bin` from linux-firmware, is Realtek's redistributable binary (`userland/capsule_driver_rtl8822ce/firmware/README.md`).
