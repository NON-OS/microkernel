# Kernel grants and IPC

The device-grant broker, the PCI stack and its boot pass, sender admission to
driver endpoints, the input ring's capabilities and kernel entropy. Index and
notation: README.md. Paths under src/ unless they start with userland/.

## Grant broker: near

The MkDevice*, MkMmio*, MkIrq*, MkDma*, MkPio* and MkPciConfig* syscalls over crate::hardware::broker (claims, grant tables, per-capsule IOMMU domains).

- **high** `syscall/microkernel/device.rs:180`: no per-capsule device scope; any Driver holder can claim, map and DMA any unclaimed device, and the verifier found Admin passes the same gates (authority_broker.rs), so the installer could claim the NIC on an air-gapped profile. -> K6
- **medium**, was high (a device that lies about its BAR already has DMA before it is claimed) `hardware/broker/mmio/map.rs:76`: MkMmioMap maps a device-reported BAR base with no check against RAM, the kernel image or the APIC pages; the checked decoder the audit proposed would still admit RAM. -> K6
- **medium** `drivers/pci/manager/probe.rs:68`: the broker's device table is built by decode_all_bars_unchecked. -> K6 (its map-time check covers it)
- **medium** `hardware/broker/confine/attach.rs:41`: the unconfined fallback; the same defect as iommu.md's attach.rs:62. -> K1
- **refuted**, was medium (the capsule writes device_addr into its own device, and unconfined it already reaches all memory) `hardware/broker/dma/map/transaction.rs:48`: MkDmaMap returns a host-physical device_addr when unconfined. -> no fix needed; K1 removes the case
- **low**, was medium (the bases are MMIO-hole addresses, and i2c_pci needs its own for the LPSS remap) `syscall/microkernel/device.rs:153`: MkDeviceList gives every DeviceEnum holder all physical BAR bases. -> K10
- low: `syscall/microkernel/mmio/map.rs:66` unconditional serial traces and pid 7 hooks on the grant path; `hardware/broker/grant.rs:17` eleven TCB files over 75 lines.

## In-kernel PCI stack and security helpers: partial

Enumeration, capability walks, BAR decode, MSI programming, VMD, src/drivers/pci/security and src/drivers/security.

- **medium** (new; found by both the AHCI and the wired-NIC verifiers) `bus/pci/assign/device.rs:76`: the boot BAR-assignment pass writes Bus Master on for every endpoint, assigned or not, and the broker never clears it at claim, so every device can master from early boot and a driver's own ordering changes nothing. -> K6
- **medium** `drivers/pci/security/policy.rs:25`: the PCI and driver security helpers are permissive by default and have no caller on the live path: validation code in the TCB that enforces nothing. -> none
- low: `drivers/security/examples.rs:97` demo driver code built into the kernel; `drivers/pci/error.rs:1` seventeen files over 75 lines.

## Held endpoints and the reply route

- **high** (new; raised by the storage verifier and verified on its own) `syscall/microkernel/ipc/send.rs:115`: any capsule with IPC reaches all 15 held drivers through their reply endpoints, by id or by port (4201 to 4235), because kernel_ipc.rs:69-73 delivers every name a capsule owns into its one proc.<pid> inbox while the gate checks only the name used. MkIpcCall to 4207 returns xhci's bulk-IN data (raw SCSI on a USB stick without StoreWrite), and every net.* reply endpoint passes without Network. Sends by service name or by pid are refused as intended. -> K7
- **medium**, was high (a correct default deny; opening it waits on xHCI slot ownership) `services/registry/held_table.rs:32`: driver.xhci0 admits only usb_hid0 and usb_msc0, so the USB NIC capsules are refused. Found in the USB NIC audit. -> K7 with D10
- **low**, was medium (not spawned, the names cannot be registered at runtime, and kernel_proofs classified.rs fails a spawn missing from HELD) `services/registry/held_table.rs:24`: the five USB NIC endpoints are not held. -> K7 (held to net.core alone)

## Input capabilities

- **high**, was medium (a parser bug in usb_hid or i2c_hid becomes a keylogger for every keyboard and app) `capabilities/token/types/authority_broker.rs:72`: InputSource and Admin pass can_input_consumer, so usb_hid, i2c_hid, ps2_input, policy and the installer may drain the one keystroke ring and re-post it unnoticed. Found in the usb_hid audit. -> K8
- **medium** (found as userland/capsule_driver_i2c_pci/Capsule.mk:16) `capabilities/token/types/authority_broker.rs:61`: Irq and Admin pass can_input_source, so xhci, iwlwifi, ahci, nvme, virtio_blk, hda and i2c_pci may post keys the router delivers as typed; the wired NICs, rtl8821ce and virtio_gpu hold no Irq. -> K8 (i2c_pci dropping Irq: wave 4)
- **low** (new) `kernel_core/surface_registry/input_ring/drain.rs:29`: drain_input never clears the slots it hands out, so the last 1024 key events, with resolved codepoints, stay in kernel memory. -> K8

## Entropy

The in-kernel virtio_rng driver and the capsule are in media.md.

- **medium**, was high (bounded: the seed pool fails closed and every lane has RDRAND) `hardware/virtio_rng_capsule/client/fill_random.rs:27`: the brokered entropy path is dead; no kernel code calls the capsule client, and every consumer calls the in-kernel driver, which delivers nothing. A synchronous call from the CSPRNG cannot work (it needs CAP_DRIVER and would block under the RNG lock); the bytes must reach an accumulator asynchronously. -> K9
- **medium** `crypto/util/rng/entropy/collect/pool.rs:44`: device bytes are credited as full hardware entropy with no health test, and the vault returns raw device words when RDRAND and RDSEED are absent. -> K9
- **low**, was medium (the truncated PFN names no RAM, so nothing lands) `drivers/virtio_rng/queue/core.rs:59`: the in-kernel virtio_rng gives the device kernel virtual addresses; the media audit's copy of this finding is rated medium in media.md. -> K9
- low: `drivers/virtio_rng/device/random.rs:12` completion wait is a spin count.
