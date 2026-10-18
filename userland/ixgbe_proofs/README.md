# ixgbe_proofs

Host proofs for the ixgbe (82599, X540, X550) driver capsule
(`capsule_driver_ixgbe`). The driver's files are compiled here through
`#[path]`, unmodified, over a small libc shim, and run against a register
window in host memory (`nonos_devmodel`), with a modelled part on a second
thread that finishes its resets. QEMU has no ixgbe device, so this is the
only run the driver gets before hardware.

## What it proves

44 `#[test]` functions:

- the id table against Linux's names and order, and the parts left out
  (82598, X550EM, VFs, igb and igc) (`id_tests`, `part_tests`);
- the reset: quiesce, master stop before any reset, the double reset, a
  reset that never clears, a late auto-read, and the ordering
  (`reset_tests`, `reset_wait_tests`, `reset_order_tests`);
- the 82599's AUTOC put back and HLREG0, RDRXCTL and FCTRL
  (`control_tests`);
- both queues (`tx_queue_tests`, `rx_queue_tests`);
- a whole bring-up and its line (`bring_up_tests`);
- a failed bring-up standing the part down and giving every grant back
  (`finish_tests`);
- LINKS decoding, NBASE-T included (`link_tests`);
- the descriptors, rings, request decode, log line and waits, shared with
  igb's proofs (`tx_encode_tests`, `rx_wb_tests`, `rx_fuzz`, `tx_ring_tests`,
  `tx_wrap_tests`, `decode_fuzz`, `log_tests`, `wait_tests`,
  `pci_command_tests`).

## Run

```sh
cd userland/ixgbe_proofs && cargo test --release
```
