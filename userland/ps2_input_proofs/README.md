# ps2_input_proofs

Host proofs for the PS/2 input driver's bring-up and decode
(`capsule_driver_ps2_input`). The crate includes the shipping discovery, init,
setup, the drain with its keyboard framing and decode, the mouse packet path,
and the serve loop's state, pump and upkeep with `#[path]`; the active
keyboard layout alone is a stand-in (`src/keymap/active.rs`), as the shipping
one asks the policy store. The driver reaches the i8042 only through port I/O
calls into `nonos_libc`, so the shim (`libc_shim/`) is the seam: an i8042
modelled from the datasheet answers every read and takes every write in the
test's own thread, with nothing left to timing, and every event the driver
posts and every console line it writes is kept for the test.

## What it proves

59 `#[test]` functions: whether a controller is there at all (the broker lists
the keyboard record on every machine, so the controller answering is what
counts) and what a failed bring-up gives back (`presence_tests`); the setup
sequence and its fallbacks (`sequence_tests`, `sequence_fallback_tests`);
every step of the keyboard and mouse bring-up, with every byte the driver
sends recorded in order (`keyboard_tests`, `mouse_tests`); waits that sleep
once the first polls go unanswered (`pause_tests`); the keyboard's prefix
framing, Pause, Print Screen and Break, and command replies (`frame_tests`,
`frame_state_tests`); the lock keys and the LED exchange (`lock_tests`,
`led_tests`), and the first LED sync lighting Num Lock (`sync_tests`); the
mouse packet decode, its timed resync and reset detection, and its queue
(`packet_tests`, `mouse_sync_tests`, `mouse_queue_tests`); a mouse that reset
brought up again with a key let go meanwhile still posted (`reinit_tests`),
and one that keeps resetting turned off after three bring-ups in a minute
(`flap_tests`). The cases come from real machines: firmware
that hands the port off disabled, a keyboard acknowledgement that lands late,
and an aux port with nothing behind it that echoes forever.

Scancode and packet decoding against hostile bytes is also in
`userland/input_proofs`.

## What it does not prove

Interrupt delivery on IRQ1 and IRQ12, the kernel's input ring behind the
post, the IPC receive and the op handlers around the serve loop, and the
policy store's choice of layout, which reach the kernel.

## Run

```sh
cd userland/ps2_input_proofs && cargo test --release
```

See [drivers](../../docs/handbook/drivers.md) and
[proofs](../../docs/handbook/verification/proofs.md).
