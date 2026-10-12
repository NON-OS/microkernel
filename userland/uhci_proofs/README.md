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

48 `#[test]` functions:

- start: the controller runs, every frame on the first interrupt queue, a
  host reset that never clears fails within its bound (`start_tests`);
- ports in xHCI form, slots as USB addresses, refusals (`port_tests`);
- descriptors at 8- and 64-byte packets, a short data stage, OUT and IN
  requests (`control_tests`); a STALL, a device that only NAKs, and
  requests refused before the bus (`ep0_fault_tests`);
- interrupt reports in order with toggles kept, queue limits, a slot given
  back (`interrupt_tests`);
- bulk OUT and IN with toggles carried across transfers, a short packet, a
  STALL and reset (`bulk_tests`); the polled IN, two adapters side by side
  (`poll_tests`, `poll_share_tests`), places held by devices: two
  adapters below one hub side by side, and a first poll waiting for a
  free place (`place_tests`); several controllers as one host: ports numbered on, slots bound to
  their controller, a failed address binding nothing (`many_tests`);
- TD words, the control plan and bulk packets (`td_tests`); the wire
  (`wire_tests`);
- hubs, through nonos_usb_hub's own walk over this driver's wire
  (`hub_lane.rs` and `hub_seen.rs`, shared with ohci_proofs and
  ehci_proofs): a low-speed keyboard below a full-speed hub reporting, and
  reaching nothing once its TDs lose the low-speed mark; a full-speed
  stick's bulk data, the hub's root port left up when it is given back;
  the routed op's refusals (`hub_tests`). The model's hub (`hub.rs`) and
  bus lookup (`bus.rs`) are shared by all three host proof crates;
- virtual ports through the host front: driver.usb_hub0's walk and
  root-port pass park a hub's children, which are listed, claimed as root
  ports (first claim wins, bulk data reaching the child), detached and
  given back as nonos_usb_virt's table says (`virt_tests`,
  `virt_root_tests`).

Run with `cargo test` from this directory.
