# AHCI SATA and Intel RST

What the SATA capsule `driver.ahci0` drives, what it does with an Intel controller in RST mode, and what it refuses.

## What it drives

The [capsule](../../overview/glossary.md#capsule) takes a PCI function as an AHCI controller by its ids (`userland/capsule_driver_ahci/src/discover/rule.rs:43-58`, `is_ahci_function`):

- any SATA controller, class 01h subclass 06h, whatever its prog-if;
- an Intel controller of class 01h subclass 04h (RAID), which is how Intel RST "RAID On" presents an AHCI controller with its ABAR in BAR5;
- never an Intel VMD, which reports the RAID subclass too but is a PCI domain, not a disk controller (`userland/capsule_driver_ahci/src/discover/rule.rs:33-41`, `INTEL_VMD_DEVICE_IDS`).

Its register block, the ABAR in BAR5, must be a memory BAR that holds the global registers and one port, so the 2 KiB ABAR common on Intel chipsets is enough (`userland/capsule_driver_ahci/src/discover/rule.rs:27-31`, `MIN_ABAR_BYTES`). The kernel's inventory makes the same check before it counts an Intel RAID-mode function as SATA (`src/hardware/inventory/classify.rs:51-61`, `classify_device`). The capsule walks every controller that matches, in device list order (`userland/capsule_driver_ahci/src/discover/find.rs:32-34`, `find_ahci`).
