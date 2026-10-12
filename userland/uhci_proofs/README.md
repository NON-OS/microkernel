# uhci_proofs

Host proofs for the UHCI host capsule (`capsule_driver_uhci`). The crate
mounts the shipping fact sheet, schedule, transfers, slot table, wire and
handlers with `#[path]` and runs them against a model controller: its I/O
registers and root ports, a frame runner that walks the frame list, queue
heads and TDs as the controller does (depth first, NAK retried next frame,
a STALL, error or short packet with SPD halting the queue), and full- and
low-speed devices that answer each packet and count every wrong data
toggle.

## What it proves

33 `#[test]` functions:

- start: the controller runs, every frame on the first interrupt queue, a
  host reset that never clears fails within its bound (`start_tests`);
- ports in xHCI form, slots as USB addresses, refusals (`port_tests`);
- descriptors at 8- and 64-byte packets, a short data stage, OUT and IN
  requests (`control_tests`); a STALL, a device that only NAKs, and
  requests refused before the bus (`ep0_fault_tests`);
- interrupt reports in order with toggles kept, queue limits, a slot given
  back (`interrupt_tests`);
- bulk OUT and IN with toggles carried across transfers, a short packet, a
  STALL and reset (`bulk_tests`); the polled IN (`poll_tests`);
- TD words, the control plan and bulk packets (`td_tests`); the wire
  (`wire_tests`).

Run with `cargo test` from this directory.
