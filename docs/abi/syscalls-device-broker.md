# Syscalls: Device broker

Part of [Syscalls](syscalls.md).

### Device broker

These are the calls of the device broker. `MkDeviceClaim` returns a [claim
epoch](../overview/glossary-claim-epoch.md#claim-epoch) that the later calls on the device
pass back. See [Broker](broker.md) for the arguments and the records.

| Tag | Number | Name | Capability | Gate | Meaning |
|---|---|---|---|---|---|
| `MDLS` | `0x534C444D` | `MkDeviceList` | DeviceEnum | `can_device_enum` `src/syscall/contract/cap_table/mk_device.rs:24` | List broker devices, all or one class. |
| `MDCL` | `0x4C43444D` | `MkDeviceClaim` | Driver | `can_driver` `src/syscall/contract/cap_table/mk_device.rs:25` | Claim a device; returns the claim epoch. |
| `MDRL` | `0x4C52444D` | `MkDeviceRelease` | Driver | `can_driver` `src/syscall/contract/cap_table/mk_device.rs:25` | Release a claimed device and every grant on it. |
| `MBFB` | `0x4246424D` | `MkBootFramebuffer` | Mmio | `can_mmio` `src/syscall/contract/cap_table/mk_platform.rs:71` | The boot framebuffer's geometry, for the holder of its claim. |
| `MMMP` | `0x504D4D4D` | `MkMmioMap` | Mmio | `can_mmio` `src/syscall/contract/cap_table/mk_device.rs:26` | Map part of a claimed device's MMIO BAR; `flags` bit 0 (`MK_MMIO_MAP_WC`) asks write-combining, given only on a prefetchable BAR (`src/hardware/broker/mmio/cache.rs`), and the kernel's own type where it maps the range (`src/hardware/broker/mmio/memory_type.rs`). |
| `MMUM` | `0x4D554D4D` | `MkMmioUnmap` | Mmio | `can_mmio` `src/syscall/contract/cap_table/mk_device.rs:26` | Unmap an MMIO grant. |
| `MIRB` | `0x4252494D` | `MkIrqBind` | Irq | `can_irq` `src/syscall/contract/cap_table/mk_device.rs:30` | Bind a claimed device's interrupt: INTx, MSI or MSI-X. |
| `MIRU` | `0x5552494D` | `MkIrqUnbind` | Irq | `can_irq` `src/syscall/contract/cap_table/mk_device.rs:30` | Unbind an interrupt grant. |
| `MIRA` | `0x4152494D` | `MkIrqAck` | Irq | `can_irq` `src/syscall/contract/cap_table/mk_device.rs:30` | Acknowledge an interrupt on a grant. |
| `MIRP` | `0x5052494D` | `MkIrqPoll` | Irq | `can_irq` `src/syscall/contract/cap_table/mk_device.rs:30` | Read a grant's interrupt sequence without waiting. |
| `MIRW` | `0x5752494D` | `MkIrqWait` | Irq | `can_irq` `src/syscall/contract/cap_table/mk_device.rs:31` | Wait for an interrupt on one grant or on all of the caller's. |
| `MDMM` | `0x4D4D444D` | `MkDmaMap` | Dma | `can_dma` `src/syscall/contract/cap_table/mk_device.rs:32` | Get a DMA buffer for a claimed device. |
| `MDMU` | `0x554D444D` | `MkDmaUnmap` | Dma | `can_dma` `src/syscall/contract/cap_table/mk_device.rs:32` | Free a DMA grant. |
| `MDIS` | `0x5349444D` | `MkDmaImportSurface` | Dma, GraphicsSurfaceMap | `can_dma` and `can_surface_map` `src/syscall/contract/cap_table/mk_device.rs:35` | Lend a claimed device a surface the caller holds attached, read only, at one IOVA run in its IOMMU domain, for direct scanout (`src/hardware/broker/dma/import/`). `ENODEV` when no IOMMU confines the device; `EPERM` without the claim or the attach. Writes `{device_addr, length, import_id}`. |
| `MDUS` | `0x5355444D` | `MkDmaUnimportSurface` | Dma, GraphicsSurfaceMap | `can_dma` and `can_surface_map` `src/syscall/contract/cap_table/mk_device.rs:35` | Give an import back: out of the device's domain, the IOTLB invalidation waited on, before it returns. A release of the attach, a device release and the holder's exit do the same. |
| `MPCR` | `0x5243504D` | `MkPciConfigRead` | Driver | `can_driver` `src/syscall/contract/cap_table/mk_device.rs:33` | Read an allowed PCI configuration register of a claimed device. |
| `MPCW` | `0x5743504D` | `MkPciConfigWrite` | Driver | `can_driver` `src/syscall/contract/cap_table/mk_device.rs:33` | Write an allowed 16-bit PCI configuration register. |
| `MPGT` | `0x5447504D` | `MkPioGrant` | Pio | `can_pio` `src/syscall/contract/cap_table/mk_device.rs:38` | Get a grant over a claimed device's port BAR; x86_64 only. |
| `MPRD` | `0x4452504D` | `MkPioRead` | Pio | `can_pio` `src/syscall/contract/cap_table/mk_device.rs:38` | Read 1, 2 or 4 bytes from a granted port. |
| `MPWR` | `0x5257504D` | `MkPioWrite` | Pio | `can_pio` `src/syscall/contract/cap_table/mk_device.rs:38` | Write 1, 2 or 4 bytes to a granted port. |
| `MPRL` | `0x4C52504D` | `MkPioRelease` | Pio | `can_pio` `src/syscall/contract/cap_table/mk_device.rs:38` | Drop a port grant. |
| `MHVB` | `0x4256484D` | `MkHvBackdoor` | Pio | `can_pio` `src/syscall/contract/cap_table/mk_device.rs:38` | One VMware backdoor call from the absolute pointer's set, where the host answers it; -19 elsewhere. |
| `MHVP` | `0x5056484D` | `MkHvPost` | Driver | `can_driver` `src/syscall/contract/cap_table/mk_platform.rs:69` | Post a VMBus message: the bus's on the root, a channel's own on its record. |
| `MHVR` | `0x5256484D` | `MkHvRecv` | Driver | `can_driver` `src/syscall/contract/cap_table/mk_platform.rs:69` | Take the oldest VMBus message: the core's on the root, a channel's answers on its record. |
| `MHVE` | `0x4556484D` | `MkHvEvents` | Driver | `can_driver` `src/syscall/contract/cap_table/mk_platform.rs:69` | Take and clear VMBus's channel event flags: the core's on the root, a channel's on its record. |
| `MHVS` | `0x5356484D` | `MkHvSignal` | Driver | `can_driver` `src/syscall/contract/cap_table/mk_platform.rs:69` | Signal the host on a channel's connection: any on the root, the channel's own on its record. |
| `MHVC` | `0x4356484D` | `MkHvChannel` | Driver | `can_driver` `src/syscall/contract/cap_table/mk_platform.rs:69` | Give a channel the host offered a record of its own, for its driver; the VMBus root's holder only, never a host service. |

### ACPI capsule

These are capsule_acpi's calls (`src/syscall/microkernel/acpi/dispatch.rs`). Every
one but `MADN`, `MUPF` and `MAMT` also answers EPERM to anyone outside the `acpi`
service owner's thread group (`src/syscall/microkernel/acpi/owner_gate.rs`). Where
there is no ACPI, as on aarch64 and riscv64, each answers ENOSYS.

| Tag | Number | Name | Capability | Gate | Meaning |
|---|---|---|---|---|---|
| `MTBL` | `0x4C42544D` | `MkAcpiTable` | AcpiOwner | `can_acpi_owner` `src/syscall/contract/cap_table/mk_platform.rs:61` | A copy of one firmware table, checked against its own length and checksum; a null buffer asks the length, a short one gets ERANGE. |
| `MRGN` | `0x4E47524D` | `MkAcpiRegion` | AcpiOwner | `can_acpi_owner` `src/syscall/contract/cap_table/mk_platform.rs:61` | Grant one OperationRegion window the broker's rule allows: SystemMemory is mapped and its address and MMIO grant id written out; SystemIO is recorded for `MkAcpiPort`. |
| `MRIO` | `0x4F49524D` | `MkAcpiPort` | AcpiOwner | `can_acpi_owner` `src/syscall/contract/cap_table/mk_platform.rs:61` | One `in` or `out` of 1, 2 or 4 bytes on a SystemIO window `MkAcpiRegion` granted. |
| `MRPC` | `0x4350524D` | `MkAcpiPci` | AcpiOwner | `can_acpi_owner` `src/syscall/contract/cap_table/mk_platform.rs:61` | One PCI_Config access; the address is an ECAM offset with the segment in bits 32-47. |
| `MAPW` | `0x5750414D` | `MkAcpiPower` | AcpiOwner | `can_acpi_owner` `src/syscall/contract/cap_table/mk_platform.rs:61` | The latest battery and AC reading, kept for `MkBatteryStatus`. |
| `MAEV` | `0x5645414D` | `MkAcpiEvent` | AcpiOwner | `can_acpi_owner` `src/syscall/contract/cap_table/mk_platform.rs:61` | A button, the lid, the hottest thermal zone or a low battery; a critical zone or battery asks the desktop to shut down. |
| `MASL` | `0x4C53414D` | `MkAcpiSleep` | AcpiOwner | `can_acpi_owner` `src/syscall/contract/cap_table/mk_platform.rs:61` | `\_S5`'s SLP_TYP values for the kernel's soft-off; state 5 only. |
| `MSMB` | `0x424D534D` | `MkSmbiosTable` | AcpiOwner | `can_acpi_owner` `src/syscall/contract/cap_table/mk_platform.rs:61` | The SMBIOS entry point (0) or its structure table (1), checked and copied; a null buffer asks the length. |
| `MACP` | `0x5043414D` | `MkAcpiCpu` | AcpiOwner | `can_acpi_owner` `src/syscall/contract/cap_table/mk_platform.rs:61` | One processor's performance and idle states (_PSS, _PCT, _PPC, _CST, _CPC, _LPI) as `cpu_layout` lays them out. |
| `MAHO` | `0x4F48414D` | `MkAcpiHolds` | AcpiOwner | `can_acpi_owner` `src/syscall/contract/cap_table/mk_platform.rs:61` | Whether a PCI function is claimed, by anyone (mode 0) or by a pid (mode 1); 1 or 0. |
| `MPPF` | `0x4650504D` | `MkPciProbeFunction` | AcpiOwner | `can_acpi_owner` `src/syscall/contract/cap_table/mk_platform.rs:61` | Read one function after its _PS0 and add it to the device table; returns its device id. |
| `MPDP` | `0x5044504D` | `MkPciDepart` | AcpiOwner | `can_acpi_owner` `src/syscall/contract/cap_table/mk_platform.rs:61` | A device leaves the table (modes 0 and 2, its holder's grants revoked first), or EBUSY before `_EJ0` while it is held (modes 1 and 3). |
| `MAFO` | `0x4F46414D` | `MkAcpiFunctionOf` | AcpiOwner | `can_acpi_owner` `src/syscall/contract/cap_table/mk_platform.rs:61` | The PCI function behind a broker device id, as segment << 16 | bus << 8 | devfn; mode 1 also asks that the pid holds it. |
| `MAHE` | `0x4548414D` | `MkAcpiHwError` | AcpiOwner | `can_acpi_owner` `src/syscall/contract/cap_table/mk_platform.rs:61` | One decoded hardware error record, counted on the posture line by its severity. |
| `MAPD` | `0x4450414D` | `MkAcpiPlatformDevice` | AcpiOwner | `can_acpi_owner` `src/syscall/contract/cap_table/mk_platform.rs:61` | Add a platform controller described by its evaluated `_CRS` to the device table; returns its device id. |
| `MADN` | `0x4E44414D` | `MkAcpiDeviceName` | DeviceEnum | `can_device_enum` `src/syscall/contract/cap_table/mk_platform.rs:31` | An ACPI-bus device's `_HID`, `_UID` and namespace path; mode 1 (a holder's question) is the acpi owner's alone. |
| `MAUP` | `0x5055414D` | `MkAcpiUsbPorts` | AcpiOwner | `can_acpi_owner` `src/syscall/contract/cap_table/mk_platform.rs:61` | Publish a USB host's port facts (`_UPC`, `_PLD`), eight bytes a port; returns the host's device id. |
| `MUPF` | `0x4650554D` | `MkUsbPortFacts` | DeviceEnum | `can_device_enum` `src/syscall/contract/cap_table/mk_platform.rs:31` | Read back the port facts `MkAcpiUsbPorts` published for a USB host. |
| `MAXP` | `0x5058414D` | `MkAcpiExternalPort` | AcpiOwner | `can_acpi_owner` `src/syscall/contract/cap_table/mk_platform.rs:61` | Mark a PCIe port whose `_DSD` says ExternalFacingPort, and every function behind it, external; returns how many. |
| `MAMR` | `0x524D414D` | `MkAcpiReserved` | AcpiOwner | `can_acpi_owner` `src/syscall/contract/cap_table/mk_platform.rs:61` | The I/O and memory the motherboard resource devices keep, 24 bytes a range; the broker refuses any grant into one from then on. |
| `MAPX` | `0x5850414D` | `MkAcpiProximity` | AcpiOwner | `can_acpi_owner` `src/syscall/contract/cap_table/mk_platform.rs:61` | A PCI root bridge's buses and its _PXM, for each record's `numa_node`. |
| `MNNO` | `0x4F4E4E4D` | `MkNumaNodeOf` | DeviceEnum | `can_device_enum` `src/syscall/contract/cap_table/mk_platform.rs:31` | The NUMA domain of a physical range, from the SRAT; ENOENT where the SRAT names no memory there, EXDEV when the range spans two domains. |
| `MNDS` | `0x53444E4D` | `MkNumaDistance` | DeviceEnum | `can_device_enum` `src/syscall/contract/cap_table/mk_platform.rs:31` | The SLIT distance from one domain to another; ENOENT without a SLIT or past its localities. |
| `MDNN` | `0x4E4E444D` | `MkDeviceNode` | DeviceEnum | `can_device_enum` `src/syscall/contract/cap_table/mk_platform.rs:31` | The full NUMA domain of a broker device, which the record's one byte cannot name past 254; ENOENT when nothing names one. |
| `MNHM` | `0x4D484E4D` | `MkNumaHmat` | DeviceEnum | `can_device_enum` `src/syscall/contract/cap_table/mk_platform.rs:31` | The HMAT latency (kinds 0 to 2, in picoseconds) or bandwidth (kinds 3 to 5, in MB/s) from an initiator domain to a target domain, at a memory level 0 to 3, packed as `level << 8 \| kind`; ENOENT where the HMAT gives no value, ERANGE when the target cannot be reached, EINVAL for a kind or level out of range. |
| `MAOG` | `0x474F414D` | `MkAcpiOscGrant` | AcpiOwner | `can_acpi_owner` `src/syscall/contract/cap_table/mk_platform.rs:61` | What a root bridge's _OSC offered and what NONOS committed; a grant past the kernel's claim is cut to it. |
| `MALP` | `0x504C414D` | `MkAcpiLpit` | AcpiOwner | `can_acpi_owner` `src/syscall/contract/cap_table/mk_platform.rs:61` | One field of LPIT's indexed low-power idle state, for suspend-to-idle. |
| `MAFP` | `0x5046414D` | `MkAcpiFirstPass` | AcpiOwner | `can_acpi_owner` `src/syscall/contract/cap_table/mk_platform.rs:61` | The capsule's first publish pass is done; nothing clears it. |
| `MRSL` | `0x4C53524D` | `MkAcpiSlot` | AcpiOwner | `can_acpi_owner` `src/syscall/contract/cap_table/mk_platform.rs:61` | A native hot-plug slot's four registers, and what is below it. |
| `MAMT` | `0x544D414D` | `MkAcpiMeasured` | AttestRead | `can_attest_read` `src/syscall/contract/cap_table/mk.rs:57` | The measured firmware tables, 44 bytes each, for the About screen and a verifier; a short buffer is refused unwritten. |
| `MALD` | `0x444C414D` | `MkAcpiLoaded` | AcpiOwner | `can_acpi_owner` `src/syscall/contract/cap_table/mk_platform.rs:61` | A table AML loads at run time, copied, checked whole and measured before it loads. |
| `MATL` | `0x4C54414D` | `MkAcpiThermalLimit` | AcpiOwner | `can_acpi_owner` `src/syscall/contract/cap_table/mk_platform.rs:61` | The ceiling passive cooling allows one processor, in percent; 100 lifts it. |

### Graphics and input

| Tag | Number | Name | Capability | Gate | Meaning |
|---|---|---|---|---|---|
| `MSRG` | `0x4752534D` | `MkSurfaceRegister` | GraphicsSurfaceCreate | `can_surface_create` `src/syscall/contract/cap_table/mk_trust.rs:41` | Register the caller's own memory as a surface; returns its id. A range in a broker window is refused unless it lies wholly inside one DMA grant the caller's process holds (a GPU driver's scanout); such a surface's frames stay the grant's, and when the grant goes they wait for the last process mapping them before the broker frees them (`src/kernel_core/surface_registry/pin/grant_run.rs`). |
| `MSSH` | `0x4853534D` | `MkSurfaceShare` | GraphicsSurfaceCreate | `can_surface_create` `src/syscall/contract/cap_table/mk_trust.rs:41` | Share a surface the caller owns with `receiver_pid`, a live process other than the caller's, and return its handle; `flags` bit 0 maps that receiver's view writable, any other bit is `EINVAL`, a fifth receiver of one surface `ENOMEM`. Naming the caller's own process names nobody and only returns the handle (`do_share` in `src/syscall/dispatch/router/surface_handlers/share.rs`, `receiver` in `src/syscall/dispatch/router/surface_handlers/receiver.rs`). |
| `MSAT` | `0x5441534D` | `MkSurfaceAttach` | GraphicsSurfaceMap | `can_surface_map` `src/syscall/contract/cap_table/mk_trust.rs:42` | Map a surface into the caller and write its descriptor: only its owner or a receiver the project named, read-only unless named to write; anyone else gets `EPERM` whether or not the handle names a surface (`attach` in `src/kernel_core/surface_registry/share/rights_attach.rs`). |
| `MSRL` | `0x4C52534D` | `MkSurfaceRelease` | GraphicsSurfaceCreate | `can_surface_create` `src/syscall/contract/cap_table/mk_trust.rs:41` | Drop the caller's reference to a surface. the project's release takes back every receiver's right; a view already mapped stays until that receiver releases it (`owner_releases` in `src/kernel_core/surface_registry/share/rights.rs`). |
| `MSPR` | `0x5250534D` | `MkSurfacePresent` | GraphicsPresent | `can_present` `src/syscall/contract/cap_table/mk_trust.rs:43` | Present a surface, whole or one damage rectangle. |
| `MDVW` | `0x5756444D` | `MkDisplayVsyncWait` | GraphicsDisplayQuery | `can_display_query` `src/syscall/contract/cap_table/mk_trust.rs:44` | Sleep to the next tick of the kernel's 60 Hz frame clock; not the display's own sync. |
| `MSFR` | `0x5246534D` | `MkSurfaceFenceRelease` | GraphicsSurfaceMap | `can_surface_map` `src/syscall/contract/cap_table/mk_surface.rs:33` | A process holding the surface attached is done reading it: the surface's release count moves on and its owner's waiters wake (`src/kernel_core/surface_registry/fence/`). Returns the new count. |
| `MSFW` | `0x5746534D` | `MkSurfaceFenceWait` | GraphicsSurfaceCreate | `can_surface_create` `src/syscall/contract/cap_table/mk_surface.rs:34` | the project of a surface (by sid), or for a surface `MkPeerPin` made the guest's supervisor, waits until its release count is not `seen`, at most `timeout_ms` (0 only reads it, 60 s at most). Returns the count; a surface freed meanwhile answers `EINVAL`. |
| `MIEP` | `0x5045494D` | `MkInputEventPost` | InputSource | `can_input_source` `src/syscall/contract/cap_table/mk_surface.rs:36` | Post one input event to the kernel's input ring. |
| `MIED` | `0x4445494D` | `MkInputEventDrain` | InputConsumer | `can_input_consumer` `src/syscall/contract/cap_table/mk_surface.rs:37` | Drain input events from the ring. |
| `MIEW` | `0x5745494D` | `MkInputEventWait` | InputConsumer | `can_input_consumer` `src/syscall/contract/cap_table/mk_surface.rs:38` | Wait for new input events. |
| `GDIM` | `0x4D494447` | `GraphicsDisplayDimensions` | GraphicsDisplayQuery | `Capability::GraphicsDisplayQuery` `src/syscall/contract/cap_table/graphics.rs:29` | The display's width and height, and its physical size when the firmware gives it; with no framebuffer, ENOTSUP and, in the optional fifth argument, why (`nonos_display_refusal`). |

Both display calls know display 0 only. `wait_for_vsync` sleeps to the next
multiple of a period taken from `TARGET_HZ`, 60 by default, and reads no
hardware sync (`src/kernel_core/surface_registry/vsync.rs:19-58`).
`handle_display_dimensions` refuses any other display with `EINVAL`
(`src/syscall/dispatch/router/graphics_backend.rs:47-50`).
