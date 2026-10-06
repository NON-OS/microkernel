# Intel VMD

How NONOS reaches an NVMe drive that firmware hides behind an Intel Volume Management Device, what has not been tested, and what to change in firmware setup when the drive does not show.

## What VMD does to a disk

With RST or VMD on in firmware, the NVMe drives, and on some boards the SATA controller, leave PCI segment 0. They sit in a private PCI domain behind one Intel function: config space through its BAR0 (CFGBAR), memory through BAR2 and BAR4 (`src/drivers/pci/vmd/domain/mod.rs:17-30`, `ConfigPort`). No disk driver can see a drive there until something brings that domain up, and the installer then shows no disk.
