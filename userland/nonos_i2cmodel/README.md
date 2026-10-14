# nonos_i2cmodel

An I2C controller, a bus and a device, modelled from their specifications so
a driver can be run against them on the host.

`nonos_devmodel` puts a register window in memory and lets a driver's own
accessor read and write it. That proves a bring-up sequence. It cannot prove
a transfer, because the DesignWare core's data register is a FIFO on both
sides: a write pushes a command, a read pops a byte, and reading the
abort-clear register is what clears the abort. Memory cannot see a read.

This crate is reached differently. A proof crate replaces the driver's
register accessor, one small file, with one that forwards every access to
[`Designware`], a core with each register's read and write semantics written
from the databook. Below it sits a [`Bus`] with [`Target`]s, and
[`HidOverI2c`] is a precision touchpad speaking HID over I2C on that bus.
[`Expander`] is a GPIO expander of the PCA9555 kind beside it and [`Sensor`]
a thermal sensor of the LM75 kind, devices the touchpad's driver must never
touch, which count every byte read from them and keep every byte written.

## What the model enforces

The core executes its command queue while the driver looks at a register,
the way a real part works while software polls, so a burst of commands pushed
without checking the FIFO depth overflows here as it would there. The FIFO
depths come from `Config` and are reported in `IC_COMP_PARAM_1`, with the
shallow variants the databook allows. Configuration registers take a value
only while the core is disabled. A direction change repeats the START only
when `IC_CON.RESTART_EN` is set; otherwise the core closes the transfer, and
the modelled device, which is strict, forgets its register on the STOP.

The LPSS reset register holds the core while its function bits (1:0) are
clear; bit 2, the integrated DMA, the core runs without. Holding the function
in reset returns every core register to its power-on value. A bus can be
stopped (`stall_bus`): the core takes commands and runs none of them, until
the next function reset or for good, as a wedged master and a device holding
SDA low behave.

Every act the databook forbids is recorded as a [`Violation`]: a core
register touched while the LPSS wrapper holds it in reset, a configuration
write while enabled, a TX or RX overflow, a data-register read with nothing
to pop, a direction change with no way to repeat the START, a ten-bit target.
Real silicon reports none of these; it drops the write or loses the byte and
the failure shows up somewhere else.

## What the model records

Register writes in order, so a bring-up can be checked as a sequence. The
bus trace: every START and repeated START with its acknowledgement, every
byte with its acknowledgement, every read with whether the master NACKed it,
every STOP. The device's view: which register was read, which command was
received with what report, what feature report the host set.

## What it does not prove

Timing. The model has no clock; SCL counts are checked arithmetically by the
proof crates against the specification's minimums, not observed on a wire.
Silicon errata. A part that departs from the databook departs from this too.

## Who uses it

`i2c_hid_proofs` and `i2c_transfer_proofs` depend on it by path. It is a
host-only library and ships in no image.

See [drivers](../../docs/handbook/drivers.md) and
[proofs](../../docs/handbook/verification/proofs.md).
