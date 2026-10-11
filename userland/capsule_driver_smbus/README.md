# capsule_driver_smbus

Status: wired, not yet signed. The kernel mirror (`src/hardware/smbus_capsule`)
embeds and spawns it under `nonos-capsule-driver-smbus` (the
`microkernel-driver-smbus` build) when the inventory sees `SerialSmbus`, an
Intel function of class 0Ch subclass 05h. `driver.smbus0` is held to the
kernel alone until a driver above it needs the bus. It has no owner
certificate yet, so no image carries it until the owner signs it.

## Role

`capsule_driver_smbus` drives the SMBus host of Intel's ICH and PCH chipsets
(the controller Linux's `i2c-i801` drives) and serves SMBus transfers on
`driver.smbus0`. It owns the host, not what sits on the bus: SPD EEPROMs,
sensors and SMBus touchpads are for drivers above it.

```text
a driver above --NSMB--> driver.smbus0 --PIO grant--> SMBus host --> devices
```

## Microkernel contract

- `MkDeviceList` finds an Intel function of class 0Ch subclass 05h whose
  BAR4 (SMBA) is an I/O range of at least 16 ports. The iSMT hosts of Atom
  servers share the class but have no I/O BAR, so they are left out.
- `MkDeviceClaim` claims it. `MkPciConfigRead` checks that firmware left I/O
  decode on (the broker writes no Command bit for ports) and reads HOSTC.
- `MkPciConfigWrite` sets HOSTC's host enable and clears its I2C mode, the
  two bits `src/hardware/broker/pci/quirk_bits.rs` allows.
- `MkPioGrant` grants BAR4; every `in` and `out` is `MkPioRead` or
  `MkPioWrite`.
- `MkIpcRecvFrom` serves `service:4244:driver.smbus0`; `MkIpcReply` answers.

## Interface contract

NSMB, version 1: driver.hda0's 20-byte header layout (its codec files are
mounted) under the magic `NSMB`.

- `HEALTHCHECK` (1): 0.
- `INFO` (2): u16 vendor, u16 device, u16 ports, u16 reserved.
- `TRANSFER` (3): u8 address (7-bit), u8 flags (bit 0 read), u8 kind
  (0 quick, 1 byte, 2 byte data, 3 word data, 4 block), u8 command, u8
  length, then the bytes a write carries (1 for byte data, 2 low first for
  word data, 1 to 32 for a block). The reply's data is a length byte and the
  bytes a read returned.

Errors, as Linux's i801 returns them: -6 no device acknowledged, -5 the
host failed, -11 arbitration lost, -16 host busy, -71 a block length outside
1 to 32, -95 block transfers on a host older than ICH4, -110 timed out (the
transaction is killed), -22 a malformed request.

## Authority

`CAPSULE_REQUIRED_CAPS = 0x118018`: IPC, Memory, DeviceEnum, Driver, Pio.
`CAPSULE_OPTIONAL_CAPS = 0x100`: Debug, in a serial-debug build only. No
Mmio, Irq or Dma: the host is polled and moves every byte through its
registers.

## Current implemented surface

- Quick, byte, byte data, word data and block (through the 32-byte buffer)
  transfers, as `i801_access` runs them: a busy host refused, the flags a
  transfer left cleared first, a 200 ms bounded wait polled 1 ms apart, a
  kill on timeout, and the INUSE semaphore given back to firmware and ACPI
  after every transfer.
- At start, a read of byte 2 (the DRAM type) of each SPD EEPROM at 0x50 to
  0x57, logged.

## Wire format

A 20-byte header (magic, version, op, flags, reserved, request id, payload
length), then a 4-byte status and the op's payload.

## State ownership

The capsule owns the claim and the port grant. Devices on the bus own their
own state; the driver keeps none of it.

## Operating rules

No port outside the granted BAR. The semaphore is released after every
transfer, failed ones included, so firmware's SMI handlers can use the host.

## Privacy and persistence

Transfers pass through without being kept or logged; the start's SPD line
logs only DRAM types.

## Runtime lifecycle

Spawned once, where the inventory saw the host. With no host it logs one line and exits `EXIT_ABSENT` (2) before
claiming anything; a failed bring-up is retried on the shared bounded
schedule and ends in `EXIT_GAVE_UP` (6). Once up it waits in the kernel for
each request.

## Failure model

A transfer's failure is its reply; the host stays served. I/O decode left off
by firmware, a refused HOSTC write or a refused grant fails the bring-up
attempt, which releases the claim.

## Release target

The SMBus host on Intel machines, for SPD, sensors and SMBus touchpads.

## Release evidence

`smbus_proofs` on the host. No boot yet.

## Release checklist

- Signed by the owner and added to the image's features.
- The held table opened to the first driver above it.
- A QEMU q35 boot (ICH9 SMBus, 8086:2930, with SPD EEPROMs at 0x50 to 0x57)
  logging the SPD line.

## Real hardware bring-up checklist

- An Intel laptop or desktop logs its DIMMs' DRAM type (0x0b DDR3, 0x0c DDR4,
  0x12 DDR5).
- A transfer to an empty address answers -6, not a timeout.

## Explicit non-goals today

No interrupts, no PEC, no I2C block read mode, no byte-by-byte block mode for
pre-ICH4 hosts, no SMBus host notify, no slave mode, no AMD FCH SMBus.

## Verification

Static gate: `nonos-ci/run-static-checks.sh`.
Proofs: `(cd userland/smbus_proofs && cargo test --release)`.
