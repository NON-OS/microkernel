# ehci_proofs

Host proofs for the EHCI host capsule (`capsule_driver_ehci`). The crate
mounts the shipping fact sheet, schedule, transfers, slot table, wire and
handlers with `#[path]` and runs them against a model controller: its
capability and operational registers, root ports that know each device's
speed (a low-speed line in K-state, a full-speed port left disabled after
reset, the port owner handing either to the companion), a frame runner
that walks the periodic list and the async ring the driver built,
fetching qTDs through each QH's overlay and running them in packets of
the QH's size with the qTD's or the QH's toggle, and an async advance
doorbell answered at the next frame. The devices are `uhci_proofs`'
(mounted), which count every wrong data toggle.

## What it proves

33 `#[test]` functions:

- start: halted, reset, both schedules on, every port routed here and
  powered, the async ring closed on its one head, no interrupt QH running
  until armed; a reset that sticks fails within its bound (`start_tests`);
- routing: a high-speed device stays at speed 3; full speed is reset and
  handed on, low speed handed on unreset; each decided once; a handed
  port refused; slots as USB addresses (`port_tests`);
- descriptors, a short data stage taking its alternate link to STATUS,
  OUT and IN requests (`control_tests`); a STALL, a device that only NAKs,
  and requests refused before the bus (`ep0_fault_tests`);
- interrupt reports in order with toggles kept, QH limits, a slot given
  back (`interrupt_tests`);
- bulk OUT and IN at 512-byte packets with toggles carried, a short
  packet, a STALL and reset (`bulk_tests`); the polled IN, two adapters
  side by side (`poll_tests`, `poll_share_tests`);
- a timed-out IN taken back through the doorbell with the ring whole
  again, a full 4 KiB IN, a slot's QHs taken back and reused at DATA0
  (`recover_tests`); the wire (`wire_tests`).

Run with `cargo test` from this directory.
