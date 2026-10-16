# capsule_driver_spi

Status: wired, not yet signed, and held to no one. The kernel mirror
(`src/hardware/spi_capsule`) embeds and spawns it under
`nonos-capsule-driver-spi`, after the I2C host. It has no owner certificate
yet, so no image carries it until the owner signs it. Its endpoint is held
`KERNEL_ONLY` until a client exists (driver.spi_hid0, next).

## Role

`driver.spi0` drives an Intel LPSS SPI host: the SSP-type SPI core beside
the LPSS I2C functions in the same blocks. It serves raw, full-duplex
transfers to one client. It is never the firmware's flash controller.

```text
client --NSPI--> driver.spi0 --SSCR0/SSCR1/SSDR, chip select--> LPSS SPI core --> device
broker: claim of 00:1f.5, or of a window in the flash's --> refused (EPERM)
```

## Microkernel contract

- `MkDeviceList` lists the candidates; `MkDeviceClaim` and `MkMmioMap`
  take one function and its BAR 0.
- `MkIpcRecvFrom` serves `service:4528:driver.spi0`; `MkIpcReply` answers.
- `MkPidAlive` asks whether the holder is still there.

## Interface contract

NSPI version 1: the host wire's 20-byte header with magic `NSPI`; a
reply's payload starts with a 4-byte status. TRANSFER and CLOSE are the
holder's alone (-1 otherwise).

- `HEALTHCHECK` (1): 0.
- `OPEN` (2): `device u16, 0, 0, input_clock u32, key u64`, the key being
  the claimed function's broker device id with bit 63 set: the key the
  firmware's serial-bus record and driver.gpio0 name this host by. -16
  while a live client holds it.
- `CLOSE` (3): 0.
- `TRANSFER` (4): `mode u8, 0, 0, 0, hz u32`, then 1 to 4000 bytes out:
  as many bytes in. Mode 0 to 3 (bit 1 CPOL, bit 0 CPHA). The bit clock is
  the fastest not above `hz`; a `hz` slower than the divider reaches is
  refused (-22), never run faster. -110 when the core stops answering,
  -5 on an overrun or a chip select that would not move.
- `DOORBELL` (5): `uid u16, pin u16`, the holder's device's GpioInt line:
  `present u32, rang u32`. driver.gpio0 reports a line only to the
  claimant of its bus controller, so this driver drains gpio0's EVENTS
  (op 1) under its own key for the holder (the gpio0 client is
  driver.i2c_pci0's, mounted). `present` is 0 while gpio0 is absent or
  refuses; the holder then reads on its timer.

## Wire format

As above; every field little-endian. To the hardware (as recalled,
unverified; see Release evidence): SSCR0 0x00, SSCR1 0x04, SSSR 0x08,
SSDR 0x10; the LPSS private block at 0x200 with RESETS +0x04, chip-select
control +0x24 and capabilities +0xFC.

## Authority

`CAPSULE_REQUIRED_CAPS = 0x38018`: IPC, Memory, DeviceEnum, Driver, Mmio.
`CAPSULE_OPTIONAL_CAPS = 0x100`: Debug, in a serial-debug build only. No Irq (polled), Dma or Pio. The broker refuses every capsule the PCH's
flash controller (bus 0, device 31, function 5, whatever its id), any
device with a window in the flash's decode window (the top 16 MiB below
4 GiB), and any map of that window (broker/firmware_flash.rs).

## Privacy and persistence

Bytes pass through unread and are kept only for the transfer they ride in.
Nothing is logged but bring-up lines, and nothing is written anywhere.

## Runtime lifecycle

Spawned once, after the I2C host. It lists Intel functions of the LPSS
classes, skipping those driver.i2c_pci0's table names as I2C; claims each
in turn, maps BAR 0 and reads its LPSS capabilities, keeping the first that
says SPI and giving the rest back. A candidate another driver holds is
tried again, three times a second apart. With none, it says so and leaves.

## Failure model

A function whose capabilities do not say SPI, or whose chip select will
not hold, is given back. A transfer that stalls ends with -110; chip
select is deasserted and the core disabled after every transfer whatever
happened. A holder the kernel says is gone is let go.

## State ownership

The capsule owns its function's claim and BAR mapping and the holder.

## Operating rules

The core is enabled only during a transfer. Chip select is in software
control, deasserted between transfers. Frames move in lock step, one out
for one in, so no FIFO can overrun.

## Current implemented surface

One SPI host, chip select 0, 8-bit Motorola frames, modes 0 to 3, up to
4000 bytes a transfer, polled.

## Release target

The SPI bus under SPI-HID keyboards and touchpads (driver.spi_hid0).

## Release evidence

`spi_proofs` covers the clock divider (never above what was asked, refused
below what it reaches), the modes, the lock-step exchange against a
modelled core (order, stale frames, a silent or dead core), and the SPI
proof (a non-SPI, absent or dead capabilities register refused with no
write; chip select read back). `kernel_proofs` covers the flash guard and
the held table. Verification rows: every register offset and bit above,
the capabilities type values, and the 133 MHz input clock assumed are
recalled from the PCH datasheets' LPSS chapter, unverified.

## Release checklist

- A client (driver.spi_hid0) and the held table opened to it.
- Signed by the owner and added to an image's features.
- The register rows verified against a published Intel datasheet.

## Explicit non-goals today

No DMA, no interrupts, no chip selects but 0, no frames but 8 bits, no
more than one host, no AMD SPI (its FCH controller is the flash), and
never the firmware's flash.

## Verification

Static gate: `nonos-ci/run-static-checks.sh`.
Proofs: `(cd userland/spi_proofs && cargo test --release)` and
`(cd userland/kernel_proofs && cargo test --release)`.
