# i2c_hid_proofs

Host-runnable proofs for the HID-over-I2C touchpad capsule, from service
lookup to posted pointer events. The shipping driver source is included
through `#[path]`. Its calls into `nonos_libc` land in a shim shared with
`i2c_transfer_proofs`: the service lookup finds a controller, the IPC call
reaches the shipping i2c_pci OP_TRANSFER handler, and every input event the
driver posts is kept for the test. Below the handler is the modelled
DesignWare core with a modelled precision touchpad on its bus. Both drivers'
real wire code meets in one process, and nothing between the HID driver's
request encoder and the pad's register file is a stand-in.

## What is proved

Setup binds only the address the controller names: the pad the firmware
declared, or the one its read-only sweep answered; no other address is asked
for, a peer's address past 7 bits is refused, register 0 is never written,
and a bus with no pad is given up 30 s after the first probe, however often
it asks, saying why (`no_scan_tests`, `acpi_tests`, `window_tests`). A
keyboard is put back to sleep; a touch screen is driven as a screen, never
through the touchpad's layout; a mouse, or the mouse of a keyboard that has
one, is driven, its key reports dropped unread (`keyboard_tests`,
`bind_tests`, `mouse_app_tests`, `composite_tests`). Every finger of a screen
or a parallel-mode pad is found, hybrid frames are put together, and a
screen's first finger presses, drags and releases (`multitouch_tests`,
`screen_tests`, `hostile_tests`); the feature report logic runs over any
transport's port (`feature_tests`).

The descriptor walk lays out what HID 1.11 says: reports continued after
another's items, Push and Pop, usage ranges, four-byte usages, the logical
minimum and its sign (`walker_tests`, `range_tests`); each bound at and past
it, a vendor report too long to read costing only itself (`bounds_*`,
`long_tests`, `vendor_tests`); 200 000 hostile descriptors and 300 000
hand-built layouts, no panic with overflow checks on (`*fuzz_tests`).

A pad that resets itself is set up again, a bounded number of times, and
lets go of what it held, on every path a reset shows itself: the ring, a
mode read back lost, and on a pad that cannot read back, a run of mouse
reports with no touch report. A stick, a still finger, a NACKed or slow
write and the driver's own stall let go of nothing (`recovery/`, `reset_*`,
`recover_tests`, `stick_tests`, `doorbell_tests`, `line_tests`). Reads back
off to 16 ms on an idle pad (`pace_*`); a transfer is waited for as long as
the shipping controller's longest path takes (`budget_tests`). Frames post
input and nothing else, repeats are paced, and nothing is left behind
(`quiet_tests`, `repeat_tests`, `retain_tests`, `torn_tests`).

## Run

```sh
cd userland/i2c_hid_proofs
cargo test
```

A debug build, so the fuzz runs with overflow checks on.

See [drivers](../../docs/handbook/drivers.md) and
[proofs](../../docs/handbook/verification/proofs.md).
