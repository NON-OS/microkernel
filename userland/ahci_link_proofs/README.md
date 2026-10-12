# ahci_link_proofs

Host proofs for the AHCI driver (`capsule_driver_ahci`). The crate includes the
shipping driver source with `#[path]`, laid out in a directory tree that
mirrors the driver's module paths, so `cargo test` runs the code that boots.

## What it proves

143 `#[test]` functions over the driver's decisions and its command path:

- the SATA link-up predicates and the signature that marks an ATA disk
  (`sig_tests`, `tests`);
- which disk is served when several carry the store or plan magic, or none does
  (`choose_tests`);
- the rules an IDENTIFY block is held to: 48-bit addressing supported and
  enabled, a 512-byte logical sector, a count below 2^48, with a fuzz over
  hostile blocks (`identity_tests`, `identity_fuzz_tests`);
- the request parser, the span check and the PRD byte count a command may
  carry (`request_tests`, `span_tests`);
- the wait on a command against a port that answers honestly and against a
  hostile one, an ERR in PxTFD judged only once slot 0 is clear, and the
  wait before issue (`engine::completion_tests`);
- the command path end to end, issue to recovery, on a model HBA port and
  drive behind the host MMIO accessor (`model`, on model time): slot 0's
  header, FIS and PRD byte by byte, a device error and the next command, a
  drive still running a command the driver gave up on (no late data or late
  write lands in the next request), a short PRDBC on a read or a write, a CR
  that will not clear, an HBA error while the drive goes on, a drive slow out
  of its reset (never issued to, never reset again inside 31 s), a pulled
  disk, a device whose link is down (reset once a window), a drive that
  resets itself mid-command, slots that outlive recovery, the reply each
  failure gets (the ATA status only for the disk's own error), the time a
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
- the model and serial read from IDENTIFY, and the OP_IDENTIFY reply layout
  (`names_tests`, `identify_reply_tests`);
- the medium rule: only the kernel or a `StoreWrite` holder reaches the disk
  (`medium_tests`);
- the failure reason codes (`reason_tests`).

## What it does not prove

Real silicon: the model HBA and drive follow the spec and QEMU, not any one
part. The order of broker calls in setup. Those need a boot.

## Run

```sh
cd userland/ahci_link_proofs && cargo test --release
```

See [drivers](../../docs/handbook/drivers.md) and
[proofs](../../docs/handbook/verification/proofs.md).
