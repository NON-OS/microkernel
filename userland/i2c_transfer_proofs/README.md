# i2c_transfer_proofs

Host-runnable proofs for the Intel LPSS I2C capsule's transfer engine, probe
and IPC handler. The shipping driver source is included through `#[path]`;
the register accessor alone is replaced with one that forwards to the
DesignWare core in `nonos_i2cmodel`, with a HID touchpad on the bus behind it.
Setup's settling of a bind and its sweep are included too; the claim, the
mapping and the bring-up of each controller go through the broker, and the
bench (`src/bench`) brings the modelled core up in their place. The defects
these proofs caught are in [FOUND.md](FOUND.md).

## What is proved

A write is one START, the bytes in order, one STOP, and leaves the core
disabled with no rule broken. A register read turns round with one repeated
START, returns the register, and NACKs exactly its last byte; without the
restart flag the core still repeats the START because bring-up sets
`RESTART_EN`. An address nobody answers and a device that NACKs its data are
both a NACK to the caller, with the abort acknowledged and the next transfer
going through. The target is written only while the core is disabled. The
OP_TRANSFER handler answers with the length, the abort source and the bytes
in the layout the HID driver decodes, refuses a body whose length field
disagrees with its size before touching the bus, and passes a NACK on as its
own errno. The HID probe recognises a device by its descriptor length and
answers no, not error, for an empty address.

OP_TRANSFER and OP_PROBE reach the bound device and nothing else on the bus,
refusing every other address before the bus (`reach_tests`). OP_CONTROLLER_INFO
answers its whole 52-byte record (`info_tests`); it once copied an eight-byte
value into a four-byte slot and took the driver down on every call. A transfer
on a stalled bus yields, then sleeps, and ends at its deadline, a healthy one
never sleeps, and an empty one is refused (`pace_tests`). On a machine where
every yield gives a time slice away, a wait still ends within one slice of its
deadline, and a transfer on a moving bus polls back to back and never yields
(`busy_tests`). Until a device has returned its HID descriptor, every pad
declared on the controller, or on none, stays in reach, and once one has it
alone does (`settle_tests`); the first time a client then reads a valid HID
descriptor through OP_TRANSFER at a pad's declared register, the reach narrows
to that pad, while a malformed descriptor or a read at another register does not
(`narrow_tests`); an address declared twice carries the bound declaration's
register, so a descriptor read there narrows and one at the other does not
(`twice_tests`). A declaration with no address finds the pad by reading 0x15,
0x2C and 0x10, one byte at a time, writing nothing, up to the first that
answers, and a pad anywhere else is not found (`sweep_tests`); a GPIO expander
at 0x20 and a thermal sensor at 0x4C are never addressed, with the pad at 0x10
or 0x2C or with none answering, and a client's transfer to either is refused
(`sweep_neighbour_tests`); nothing is probed without such a declaration on the
controller, and a faulting bus ends the sweep at its first address
(`sweep_scope_tests`). Two faults in a row reset the controller: a wedged core
comes back and the next transfer goes through, a bus no reset frees is named
`Stuck`, said once, and still answers every call in bounded time, and a NACK is
the device answering, not a fault (`recover_tests`).

## Run

```sh
cd userland/i2c_transfer_proofs && cargo test --release
```

See [drivers](../../docs/handbook/drivers.md) and
[proofs](../../docs/handbook/verification/proofs.md).
