# ps2_input_proofs

Host proofs for the PS/2 input driver's bring-up
(`capsule_driver_ps2_input`). The crate includes the shipping discovery, init
and setup source with `#[path]`. The driver reaches the i8042 only through
port I/O calls into `nonos_libc`, so the shim (`libc_shim/`) is the seam: an
i8042 modelled from the datasheet answers every read and takes every write in
the test's own thread, with nothing left to timing.

## What it proves

29 `#[test]` functions: whether a controller is there at all (the broker lists
the keyboard record on every machine, so the controller answering is what
counts) and what a failed bring-up gives back (`presence_tests`); the setup
sequence and its fallbacks (`sequence_tests`, `sequence_fallback_tests`);
every step of the keyboard and mouse bring-up, with every byte the driver
sends recorded in order (`keyboard_tests`, `mouse_tests`); and the mouse
packet decode (`packet_tests`). The cases come from real machines: firmware
that hands the port off disabled, a keyboard acknowledgement that lands late,
and an aux port with nothing behind it that echoes forever.

Scancode and packet decoding against hostile bytes is also in
`userland/input_proofs`.

## What it does not prove

Interrupt delivery on IRQ1 and IRQ12, and the posting of events to the kernel
input ring.

## Run

```sh
cd userland/ps2_input_proofs && cargo test --release
```

See [drivers](../../docs/handbook/drivers.md) and
[proofs](../../docs/handbook/verification/proofs.md).
