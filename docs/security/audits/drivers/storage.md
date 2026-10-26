# Storage

nvme, ahci, virtio_blk, usb_msc and the installer's nonos_blk_client.
Index and notation: README.md.

## capsule_driver_nvme: partial

NVMe SSD capsule behind driver.nvme0: one controller, one namespace, one 8-entry I/O queue, one command in flight. Paths under userland/capsule_driver_nvme/src/ unless they start with userland/.

- **high** `nvm/wait.rs:50`: a late completion that carries an error status leaves the command marked outstanding, so every later read, write and flush spins 30 s and fails until reboot; one unreadable sector kills the disk. -> D5
- **medium**, was high (fails safe, no corruption; with the wait.rs:50 fix a late completion recovers by itself) `nvm/wait.rs:35`: after an I/O timeout there is no Abort, no CSTS.CFS or all-ones check and no controller reset. -> D5
- **medium** `setup/driver.rs:26`: field drop order unmaps the admin-queue DMA before mk_device_release turns Bus Master off, when the disable failed. -> wave 4
- **medium** `admin/command/create_io_cq.rs:30`: I/O completions are busy-polled with interrupts disabled. -> wave 4
- **medium** `nvm/constants.rs:18`: one controller, one namespace, one command in flight, 32 KiB per command. -> wave 4
- **medium** `admin/controller.rs:34`: no TRIM, Write Zeroes, orderly shutdown (CC.SHN), power states or asynchronous events. -> wave 4
- **medium** `userland/nvme_proofs/src/nvm/mod.rs:4`: the I/O data path (settle, PRP builder, transfer, handlers) has no tests. -> D5 in part
- low: `server/handlers/identify_controller.rs:33` full serial over IPC (refuted: only pid 0 and StoreWrite holders get it, and they can read every sector); `setup/say/serving.rs:27` device model string written unfiltered to the console; `server/handlers/smart_health/handle.rs:30` SMART never refreshed; `nvm/setup.rs:33` queue size not checked against CAP.MQES; `nvm/submit.rs:33` doorbell ordering only correct on x86; `admin/queue/identify_controller.rs:33` &[u8] over device-writable DMA; `regs/mmio.rs:31` 43 unsafe sites, 5 SAFETY comments; `userland/capsule_driver_nvme/Cargo.toml:7` rule violations, stale description.

## capsule_driver_ahci: partial

SATA AHCI capsule behind driver.ahci0 (one disk, one command in slot 0), with an SDHCI/eMMC stack in the same process. Paths under userland/capsule_driver_ahci/src/ unless they start with userland/.

- **high** `engine/completion.rs:48`: a stale PxTFD.ERR left by a failed command fails the next command on its first poll, and the recovery then clears ST under a command the drive may be running (a write reported failed may land). -> D6
- **high**, was medium (wrong data, or a write to the wrong LBA, after a slow error recovery) `engine/kick.rs:45`: a timed-out command is cleared with CLO only, never COMRESET, so the abandoned command's late data completes the next request. -> D6
- **medium** `engine/recover.rs:27`: recovery never escalates when PxCMD.CR will not clear, and later commands are written to PxCI with ST off. -> wave 4
- **medium** `engine/issue.rs:36`: the pre-issue wait can spend the whole budget, after which the command is issued and abandoned; the worst case (about 39 s) overruns the kernel's 35 s reply wait. -> wave 4
- **low**, was medium (crosses no confidentiality boundary; the CLO path above is the realistic source) `engine/build.rs:70`: PRDBC is never checked, so a short transfer returns stale buffer contents. -> D6
- **medium** `clock/wait.rs:31`: every wait busy-spins a core for up to 30 s; the bound IRQ is never enabled. -> wave 4
- **refuted**, was medium (Bus Master is already on from the kernel's boot pass; see kernel-grants-and-ipc.md) `setup/open.rs:45`: Bus Master is enabled before the BIOS handoff and HBA reset. -> K6 fixes the boot pass
- **low**, was medium (every reader already holds StoreWrite, and the installer's erase confirmation uses it) `server/handlers/identify.rs:42`: the drive serial is sent over IPC. -> D6 keeps it
- **medium** `constants/regs/port_bits.rs:31`: no hotplug or surprise removal, and no re-IDENTIFY after a COMRESET, so a swapped disk is served with the old bounds. -> wave 4
- **medium** `identity/capacity.rs:48`: only 512-byte-sector LBA48 disks are served; 4Kn and LBA28 disks are refused. -> wave 4
- **medium** `constants/ata.rs:33`: one command at a time, 32 KiB per command, no NCQ, one disk. -> wave 4
- **medium** `constants/ata.rs:20`: no TRIM and no FUA. -> wave 4
- **medium** `main.rs:27`: the SDHCI/eMMC stack (about 6.5k lines) shares the SATA capsule's address space and grants. -> wave 4
- **medium** `server/handlers/read.rs:48`: raw-pointer copies and unchecked MMIO accessors, 52 unsafe sites with 5 SAFETY comments. -> wave 4
- **medium** `engine/init.rs:69`: bring-up can spend minutes busy-spinning when a disk fails. -> wave 4
- **medium** `userland/ahci_link_proofs/src/engine/mod.rs:3`: the issue and recovery sequence and the IPC handlers are untested. -> D6 in part
- low: `server/poll_irq.rs:25` Irq held but never delivers; `engine/completion.rs:96` four files over 75 lines; `userland/capsule_driver_ahci/README.md:177` README contradicts the code; `server/handlers/failure.rs:28` a port-state refusal reported as an ATA status; `engine/region.rs:38` broker DMA addresses not checked for alignment or DMA32; `discover/rule.rs:54` any SATA prog-if treated as AHCI; `choose/pick.rs:28` RST RAID members served as raw disks; `engine/program.rs:43` link power management forced off, no security freeze; `server/handlers/port_list.rs:34` port list is a bring-up snapshot.

## capsule_driver_virtio_blk: partial

virtio-blk capsule, legacy and modern transports, one request in flight. Paths under userland/capsule_driver_virtio_blk/src/ unless they start with userland/. the claim check also covers this driver under a vIOMMU (media verifier).

- **low**, was medium (no wedge on a late completion; the device is the hypervisor) `io/settle.rs:33`: a request that never completes is never recovered by a reset; each later request waits 5 s and fails. -> wave 4
- **medium** `io/wait_used.rs:60`: the request wait is bounded by pass counts, not wall time (8 minutes or a few ms). -> wave 4
- **medium** `discover.rs:44`: functions without an INTx pin are skipped, and INTx is preferred over MSI-X. -> wave 4
- **medium** `queue/post/publish.rs:25`: no memory barriers between ring writes, avail.idx and notify; correct on x86 only, and it ships for aarch64. -> wave 4
- **medium** `queue/post/descriptors.rs:31`: queue depth 1, no DISCARD, WRITE_ZEROES or resize handling. -> wave 4
- **medium** `userland/virtio_blk_proofs/src/lib.rs:2`: wait loops, the descriptor chain and feature negotiation are untested. -> wave 4
- low: `queue/used.rs:32` used element ignored and data buffer never cleared; `setup/dma/rollback.rs:61` legacy rollback unmaps the ring before a reset; `init.rs:52` legacy rings over 256 refused, size not checked for a power of two; `regs/state/r16.rs:23` unsafe accessors without bounds; `setup/sequence.rs:41` rule violations, inaccurate manifest comments.

## capsule_driver_usb_msc: partial

USB Bulk-Only mass storage class driver over driver.xhci0, holding no hardware capabilities. Paths under userland/capsule_driver_usb_msc/src/ unless they start with userland/.

- **high** `disk/ready.rs:73`: probing runs inline in the server loop, bounded by tries not time; an empty card reader blocks the driver about 35 s per probe, again every 5 s, and at boot stalls the kernel's block-device selection for the internal disk. -> D12
- **medium** `server/dispatch.rs:36`: codec and diagnostic ops are served to any sender, and OP_PROBE_CONFIG clears ignore_residue on the live device; reachable today only through the kernel reply route. -> D12, K7
- **medium** `server/runner.rs:66`: no hot-unplug, re-plug or medium-change handling. -> wave 4
- **medium** `xhci/call.rs:25`: no overall deadline for one block request, which can hold the server for minutes. -> wave 4
- **medium** `userland/usb_msc_proofs/src/lib.rs:19`: xHCI reply parsing and probe timing are tested only against a mock. -> D12 in part
- low: `scan/probe.rs:66` configuration capped at 255 bytes, a parse failure blacklists the port; `scsi/sense_response.rs:39` descriptor-format sense not decoded; `descriptors/visitor.rs:98` endpoint descriptors not validated; `scan/report.rs:44` stage doc contradicts the probe budget; `span/mod.rs:29` rule violations.

## nonos_blk_client: near

Library the installer and tools use to reach driver.nvme*, driver.ahci* and driver.virtio_blk0. Paths under userland/nonos_blk_client/src/.

- **medium** `driver/table.rs:93`: the reply decoder and the hand-copied per-driver opcode table are untested and not cross-checked with the drivers. -> wave 4
- **medium** `disks/describe.rs:54`: 4Kn disks are refused as install targets, and USB disks never appear. -> wave 4
- low: `device/span.rs:136` partial-block read-modify-write not atomic across clients; `device/identity_parse.rs:44` full serial printed by tool_install (refuted: shown only to the operator about to erase that disk); `device/span.rs:94` NVMe transfer limit re-derived in the client; `disks/scan.rs:91` rule violations.
