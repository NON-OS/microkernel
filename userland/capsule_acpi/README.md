# capsule_acpi

`acpi` runs the platform's firmware: the DSDT and SSDTs, in NONOS's own AML
interpreter (`userland/nonos_aml`), in a capsule where a broken or hostile
table can stop this capsule and nothing else. It brings the platform's
devices up, reports what firmware describes, and serves the SCI.

## Role

```text
   kernel                                   capsule_acpi
   ------                                   ------------
   MkAcpiTable  -- checked copies -->  FADT, DSDT, SSDTs --> nonos_aml namespace
   MkAcpiRegion -- one grant per  -->  SystemMemory windows (mapped)
                   OperationRegion     SystemIO windows
   MkAcpiPort   -- in / out       -->  SystemIO fields, PM1 and GPE registers
   MkAcpiPci    -- config access  -->  PCI_Config fields
   SCI record (PNP0C08)
     MkDeviceClaim, MkIrqBind  --->    SCI: fixed events, _Lxx/_Exx, Notify
                                       port 4530: STATUS, DEVICE
```

At start it loads the tables, connects SystemMemory, SystemIO and
PCI_Config (running each scope's _REG), calls `\_PIC(1)`, negotiates `_OSC`
(platform-wide with nothing claimed; each PCI host bridge supporting
extended configuration space and MSI and asking for no control), runs
`\_SB._INI` and every present device's `_INI`, reads the inventory and every
`_PRT`, enters ACPI mode, arms the runtime GPEs and binds the SCI.

## Microkernel contract

Every register it touches comes through the kernel's ACPI grant rule
(`src/hardware/broker/acpi/rule/`): SystemMemory only over firmware's own
memory or unclaimed device space, never the allocator's RAM or the
interrupt, timer and IOMMU hardware; SystemIO outside the legacy devices the
kernel runs; PCI_Config writes never to another capsule's function. Each
grant is one `[ACPI] region` line on serial and a count on the
`[ACPI] posture` line. It claims one device, the SCI record, and no PCI
function, and does no DMA.

## Authority

`CAPSULE_REQUIRED_CAPS := 0x58218`: IPC (0x8) to serve port 4530; Memory
(0x10) for the heap and the interpreter's objects; DeviceEnum (0x8000) to
find the SCI record; Driver (0x10000) to claim it; Irq (0x40000) to bind
it; Admin (0x200) for the four ACPI calls, which the kernel gates on Admin
until a Firmware bit of their own is decided. Debug (0x100) is optional, for
its serial lines. No Mmio, Dma or Pio.

## Interface contract

Service `service:4530:acpi`, reply `reply:4531:endpoint.acpi.reply`, held
to `power` (`src/services/registry/held_table.rs`). Requests go through
`MkIpcRecvFrom` and replies through `MkIpcReply`; the wire format is in
`src/server/protocol.rs`: an op byte, then a status word and the body.

- STATUS (1): devices, routes, SCIs served, GPE methods run, refused region
  accesses, armed GPEs, notifications dropped.
- DEVICE (2, index): _STA, _ADR, _UID, _HID and the device's path.

## Privacy and persistence

It keeps nothing across boots. The tables it reads are the ones AML needs;
the kernel does not hand it MSDM, SLIC or the TPM tables.

## Failure model

A table the interpreter refuses adds nothing and the rest load. A method
that faults (budget, type, a refused region) stops that evaluation only. A
level GPE whose method fails is left disabled. No FADT or DSDT: it exits 2.

## Verification

`userland/nonos_aml_proofs` proves the interpreter, the event layer and the
platform layer this capsule runs; `userland/kernel_proofs` proves the grant
rule and the table checks.
