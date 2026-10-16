# ahci_link_proofs

Host proofs for the AHCI driver (`capsule_driver_ahci`). The crate includes the
shipping driver source with `#[path]`, laid out in a directory tree that
mirrors the driver's module paths, so `cargo test` runs the code that boots.

## What it proves

162 `#[test]` functions over the driver's decisions, its open path, its
command path and a port's hand-back:

- the SATA link-up predicates and the ATA disk signature (`sig_tests`, `tests`);
- which disk is served when several carry the store or plan magic, or none does
  (`choose_tests`);
- the rules an IDENTIFY block is held to: 48-bit addressing supported and
  enabled, a 512-byte logical sector, a count below 2^48, with a fuzz over
  hostile blocks (`identity_tests`, `identity_fuzz_tests`);
- the request parser, the span check and the PRD byte count a command may
  carry (`request_tests`, `span_tests`); the failure reasons (`reason_tests`);
- the wait on a command against a port that answers honestly and against a
  hostile one, an ERR in PxTFD judged only once slot 0 is clear, and the
  wait before issue (`engine::completion_tests`);
- the command path end to end, issue to recovery, on a model HBA port and
  drive behind the host MMIO accessor (`model`, on model time, reaching memory
  only in live DMA grants, by bus address, and holding PxCMD to 10.1.2's
  order): slot 0's header, FIS and PRD byte by byte, a device error and the
  next command, a drive still running a command the driver gave up on (no late
  data or late write lands in the next request), a short PRDBC on a read or a
  write, a CR that will not clear, an HBA error while the drive goes on, a
  drive slow out of its reset (never issued to, never reset again inside
  31 s), a pulled disk, a device whose link is down (reset once a window), a
  drive that resets itself mid-command, slots that outlive recovery, the reply
  each failure gets (the ATA status only for the disk's own error), the time a
  request may take against the kernel's 35 s wait, and recovery by cause
  (`model::tests`, `engine::cause_tests`, `engine::recover_cause_tests`,
  `engine::comreset_tests`);
- which ports the mapped ABAR window reaches, which ports a sparse PI names,
  which hold a talking device in any link power state, the spin-up bits, and
  what the HBA reset must give back of CAP and PI (`window_tests`,
  `ports_tests`);
- which PCI functions are taken for an HBA (SATA of any prog-if, Intel RAID
  mode, never an Intel VMD, whose list must match the kernel's) and the
  smallest ABAR taken (`discover_tests`);
- the link bring-up after COMRESET: up, negotiating, or an empty port within
  its debounce; the COMRESET kick of a port stuck busy, never CLO, and the
  engine stop (`bring_up_tests`, `engine::recover_tests`);
- the model and serial read from IDENTIFY, the OP_IDENTIFY reply layout, and
  the medium rule: only the kernel or a `StoreWrite` holder reaches the disk
  (`names_tests`, `identify_reply_tests`, `medium_tests`);
- the open path, on a model HBA a dead driver left armed (port 0, PxCLB and
  PxFB on its frames), over the shim (`libc_shim`) logging each broker call
  with the model's state: Memory Space at the claim, Bus Master only after the
  HBA reset, every port read stopped (a spared one stopped first), never for a
  refused controller (`setup::open_tests`, `spared_tests`,
  `refused_write_tests`, `controller::stopped_tests`);
- a port up through `init_port` on real DMA regions, handed back: with its
  controller, stopped, Bus Master off, the release, nothing unmapped by hand
  nor twice (`serve` on three model ports, `Driver` drop); parked on the kept
  controller, stopped first; PxFB before FRE; DMA32 asked exactly for an HBA
  without CAP.S64A, any other grant above 4 GiB (`setup::give_back_tests`,
  `serve_tests`, `driver_drop_tests`, `engine::program_tests`, `dma32_tests`).

## What it does not prove

Real silicon (the model follows the spec and QEMU, not any one part) and the
walk itself: those need a boot.

## Run

```sh
cd userland/ahci_link_proofs && cargo test --release
```

See [drivers](../../docs/handbook/drivers.md) and
[proofs](../../docs/handbook/verification/proofs.md).
