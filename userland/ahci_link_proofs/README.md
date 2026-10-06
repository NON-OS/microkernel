# ahci_link_proofs

Host proofs for the AHCI driver (`capsule_driver_ahci`). The crate includes the
shipping driver source with `#[path]`, laid out in a directory tree that
mirrors the driver's module paths, so `cargo test` runs the code that boots.

## What it proves

93 `#[test]` functions over the driver's pure decisions:

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
  hostile one, and the recovery after a failed command (`completion_tests`,
  `completion_hostile_tests`);
- which ports the mapped ABAR window reaches, which ports a sparse PI names,
  which hold a talking device in any link power state, the spin-up bits, and
  what the HBA reset must give back of CAP and PI (`window_tests`,
  `ports_tests`);
- which PCI functions are taken for an HBA (SATA of any prog-if, Intel RAID
  mode, never an Intel VMD, whose list must match the kernel's) and the
  smallest ABAR taken (`discover_tests`);
- the link bring-up after COMRESET: up, negotiating, or an empty port within
  its debounce; the CLO and COMRESET kick of a port stuck busy, and the
  engine stop (`bring_up_tests`, `engine::recover_tests`);
- the model and serial read from IDENTIFY, and the OP_IDENTIFY reply layout
  (`names_tests`, `identify_reply_tests`);
- the medium rule: only the kernel or a `StoreWrite` holder reaches the disk
  (`medium_tests`);
- the failure reason codes (`reason_tests`).

## What it does not prove

The MMIO and DMA themselves, and the order of broker calls in setup. Those
need a boot.

## Run

```sh
cd userland/ahci_link_proofs && cargo test --release
```

See [drivers](../../docs/handbook/drivers.md) and
[proofs](../../docs/handbook/verification/proofs.md).
