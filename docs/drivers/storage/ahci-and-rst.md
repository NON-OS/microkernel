# AHCI SATA and Intel RST

What the SATA capsule `driver.ahci0` drives, what it does with an Intel controller in RST mode, and what it refuses.

## What it drives

The [capsule](../../overview/glossary.md#capsule) takes a PCI function as an AHCI controller by its ids (`userland/capsule_driver_ahci/src/discover/rule.rs:43-58`, `is_ahci_function`):

- any SATA controller, class 01h subclass 06h, whatever its prog-if;
- an Intel controller of class 01h subclass 04h (RAID), which is how Intel RST "RAID On" presents an AHCI controller with its ABAR in BAR5;
- never an Intel VMD, which reports the RAID subclass too but is a PCI domain, not a disk controller (`userland/capsule_driver_ahci/src/discover/rule.rs:33-41`, `INTEL_VMD_DEVICE_IDS`).

Its register block, the ABAR in BAR5, must be a memory BAR that holds the global registers and one port, so the 2 KiB ABAR common on Intel chipsets is enough (`userland/capsule_driver_ahci/src/discover/rule.rs:27-31`, `MIN_ABAR_BYTES`). The kernel's inventory makes the same check before it counts an Intel RAID-mode function as SATA (`src/hardware/inventory/classify.rs:51-61`, `classify_device`). The capsule walks every controller that matches, in device list order (`userland/capsule_driver_ahci/src/discover/find.rs:32-34`, `find_ahci`).

## Bring-up

- It takes the controller from the firmware with the BIOS/OS handoff: 25 ms, and up to 2 s more when the firmware says it is busy (`userland/capsule_driver_ahci/src/controller/enable.rs:71-96`, `take_from_bios`).
- It sets AHCI mode and resets the controller. A reset still running after 1 s leaves the controller alone (`userland/capsule_driver_ahci/src/controller/enable.rs:28-45`, `enable_ahci`).
- On every implemented port a COMRESET decides whether a disk is there. The link must come up within 2 s, and the disk must leave BSY within 10 s, time for a spinning disk to spin up (`userland/capsule_driver_ahci/src/constants/timing.rs:37-43`, `LINK_TIMEOUT_MS`, `DEVICE_READY_MS`).
- A port whose signature names a port multiplier, an ATAPI device or an enclosure bridge is skipped. Disks behind a port multiplier are not served (`userland/capsule_driver_ahci/src/setup/say_skipped.rs:23-41`, `say_skipped`).
- Every command completion is polled, so a controller with no routed interrupt line is served all the same (`userland/capsule_driver_ahci/src/discover/candidate.rs:30-35`, `irq_line`).

## Which disk it serves

One capsule serves one disk. Of the disks that came up, it serves the one that carries NONOS, the package store header at LBA 256 or the disk plan at LBA 245760, on the lowest controller and port. With none, it serves the lowest disk that came up, which is the installer's blank target (`userland/capsule_driver_ahci/src/choose/pick.rs:19-29`, `choose`). A second SATA disk on the machine is not served in this release.

A disk must offer (`userland/capsule_driver_ahci/src/identity/refusal.rs:17-29`, `Refusal`):

- the 48-bit address feature set, supported and enabled;
- 512-byte logical sectors;
- a capacity above 0 and below 2^48 sectors.

The capsule sends four ATA commands: IDENTIFY DEVICE, READ DMA EXT, WRITE DMA EXT and FLUSH CACHE EXT (`userland/capsule_driver_ahci/src/constants/ata.rs:17-20`, `ATA_IDENTIFY`). It has no NCQ, no TRIM and no SMART. One request moves at most 64 sectors, 32 KiB (`userland/capsule_driver_ahci/src/constants/ata.rs:29-34`, `MAX_SECTORS`). One command may take 30 s, after which the port is recovered with a COMRESET (`userland/capsule_driver_ahci/src/constants/timing.rs:47-55`, `COMMAND_MS`). The reply still reaches the kernel inside its own 35 s wait (`src/services/lifecycle/reply_wait.rs:28`, `SLOW_BUDGET_MS`).

## Intel RST

Intel Rapid Storage Technology changes what the firmware shows in ways that matter here.

RAID On with SATA disks. The controller reports the RAID subclass, but it is a standard AHCI controller with its ABAR in BAR5, so NONOS binds it as one (`src/hardware/inventory/classify_storage.rs:26-33`, `classify_storage`). The capsule does not read RST metadata, so a RAID volume made of several disks is not supported: to NONOS each member is a separate raw disk, and the capsule serves one disk.

NVMe hidden behind the SATA controller. In RAID mode RST can remap an NVMe drive behind the SATA controller's ABAR, and the NVMe function then disappears from PCI. The capsule reads the remap registers when the ABAR is 512 KiB or more: VSCAP at 0xA4, REMAP_CAP at 0x800, and a class code at 0x880 for each of three slots, 0x80 apart (`userland/capsule_driver_ahci/src/controller/remap.rs:23-42`, `may_remap`). When VSCAP bit 0 is set, each slot marked in REMAP_CAP whose class code is NVMe counts as one hidden drive (`userland/capsule_driver_ahci/src/controller/remap.rs:44-57`, `remapped_nvme`). NONOS does not drive the hidden drives. The capsule writes `Intel RST hides N NVMe drive(s) behind this controller; set the firmware's SATA mode to AHCI` with `mk_debug` (`userland/capsule_driver_ahci/src/setup/remap.rs:23-49`, `say_remapped`), but `driver.ahci0` holds no Debug capability (`src/hardware/ahci_capsule/spawn.rs:51-57`, `requested_caps`), so in this release that line, like every other line the capsule writes, does not reach the console. The installer's message below is what a person sees.

Intel VMD is a third arrangement, with its own page: [Intel VMD](vmd.md).

## What to change in firmware setup

When the installer finds no disk and an Intel RST or VMD controller is on the bus, it shows `Intel RST/VMD is on: set the BIOS storage mode to AHCI (or turn VMD off), then boot this stick again.` It adds that a Windows already on the computer may need switching to AHCI first, or it will not start after the change (`userland/capsule_install/src/install/ui/screens/disks.rs:36-37`, `RAID`, `RAID_WHY`). The message appears only when no disk was found, because a disk found means the firmware already lets NONOS reach one (`userland/nonos_blk_client/src/disks/scan.rs:68-73`, `raid_hides_disks`).

Firmware names this setting differently from one machine to the next; the installer calls it the storage mode.

## Access and capabilities

The capsule serves `driver.ahci0` on service endpoint 4216 (`userland/capsule_driver_ahci/Capsule.mk:14`, `CAPSULE_SERVICE_ENDPOINT`). Its operations are health check, controller info, port list, capacity, read, write, flush and identify (`userland/capsule_driver_ahci/src/protocol/ops.rs:17-26`, `OP_IDENTIFY`). Every operation but the health check answers only the kernel's own client and a holder of `StoreWrite` (`userland/capsule_driver_ahci/src/server/medium_rule.rs:25-29`, `allows`).

It holds the [capabilities](../../overview/glossary.md#capability) IPC, Memory, Driver, DeviceEnum, Mmio, Irq and Dma, the word 0xF8018 (`userland/capsule_driver_ahci/Capsule.mk:16-17`, `CAPSULE_REQUIRED_CAPS`).

The same capsule serves an eMMC disk when no SATA disk comes up; see [SD cards and eMMC](sd-and-emmc.md).

## How it was verified

- `userland/ahci_link_proofs` is the [proof crate](../../overview/glossary.md#proof-crate). It runs the link checks, the disk choice, the IDENTIFY rules, request spans, hostile completion waits, port recovery, the RST remap rule and the VMD refusal on the host, and checks that the capsule's VMD list matches the kernel's: 99 tests pass on this commit.
- No QEMU target in `mk/` and no `tools/nonos_qemu` option names an AHCI device, and no QEMU run of the SATA path is reported for this release.
- Not tested on hardware in this release.

## See also

- [Storage drivers](README.md)
- [NVMe](nvme.md)
- [Intel VMD](vmd.md)
- [SD cards and eMMC](sd-and-emmc.md)
- [Install to disk](../../install/install-to-disk.md)
- [Troubleshooting](../../install/troubleshooting.md)
- [Hardware support matrix](../../hardware/MATRIX.md)
