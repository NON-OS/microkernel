# ohci_proofs

Host proofs for the OHCI host capsule (`capsule_driver_ohci`). The crate
mounts the shipping fact sheet, schedule, transfers, slot table, wire and
handlers with `#[path]` and runs them against a model controller: its
registers, SMM owner flag and root hub of three ports; a frame runner that
walks the periodic list from the HCCA and the control and bulk lists while
their filled bits are set, running each ED's TDs from HeadP to TailP in
packets of the ED's size (a NAK leaves the TD where it got to, a STALL or
no answer halts the ED, a short IN packet ends a rounding TD), the toggle
carried in the ED; and the devices of `uhci_proofs` (mounted), which
answer each packet and count every wrong data toggle.

## What it proves

37 `#[test]` functions:

- start: SMM hand-over, legacy emulation off, reset, every list on, the
  HCCA table, ports powered; an SMM driver that keeps the controller and
  a reset that sticks fail within their bounds (`start_tests`);
- ports in xHCI form, slots as USB addresses, refusals (`port_tests`);
- descriptors at 8- and 64-byte packets, a short data stage, OUT and IN
  requests (`control_tests`); a STALL, a device that only NAKs, and
  requests refused before the bus (`ep0_fault_tests`);
- interrupt reports in order with toggles kept, ED limits, a slot given
  back (`interrupt_tests`);
- bulk OUT and IN with toggles carried across transfers, a short packet, a
  STALL and reset (`bulk_tests`); the polled IN, two adapters side by side
  (`poll_tests`, `poll_share_tests`);
- a timed-out IN taken back, a full 4 KiB IN, a slot's EDs retired and
  reused at DATA0 (`recover_tests`); ED and TD words and the lists
  (`sched_tests`); the wire (`wire_tests`).

Run with `cargo test` from this directory.
