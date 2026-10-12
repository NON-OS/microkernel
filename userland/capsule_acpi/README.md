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
   MkAcpiPower, MkAcpiEvent  <---      batteries, buttons, lid, thermal
   MkAcpiSleep               <---      \_S5 (start, and PREPARE_SLEEP)
   MkAcpiCpu                 <---      _PSS _PCT _PPC _CST _CPC, per processor
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

A hardware-reduced platform (the FADT's HW_REDUCED_ACPI flag; every Arm
one) has no SCI, PM1 or GPE blocks. There the capsule finds its Generic
Event Devices (ACPI0013) and logs each with its _CRS interrupts (trigger,
polarity, sharing, wake) and whether it has _EVT. They are not served
until the kernel can grant a GED interrupt to the capsule.

The embedded controller is found first, from the ECDT or the PNP0C09
device's _CRS, so its fields work during _REG and _INI; EmbeddedControl is
connected only once it is. Its accesses take the Global Lock when its _GLK
says so. Its GPE is served by the capsule: while SCI_EVT is set, QR_EC and
the matching _Qxx, at most 32 at a time.

Batteries (PNP0C0A) and AC adapters (ACPI0003) are read at start, every
minute, and on their Notify (0x80 state, 0x81 information): _STA's
battery bit, _BIX or else _BIF, _BST, _PSR. The charge over all batteries,
whether any is charging or discharging, and whether AC is online go to
the kernel through `MkAcpiPower`, which MkBatteryStatus then answers from
while the report is under three minutes old.

Power buttons (PNP0C0C), sleep buttons (PNP0C0E) and lids (PNP0C0D) that
report through AML are served on their Notify: 0x80 on a power button goes
to the kernel through `MkAcpiEvent`, which posts the desktop's Power key as
it does for the fixed button; a sleep button press is logged, as NONOS has
no sleep state yet; a lid's _LID is read at start and on 0x80, and the
kernel told when it changes. The fixed PM1 buttons stay the kernel's.

Thermal zones are read at their _TZP interval (every 30 seconds when they
give none), and on Notify 0x80, 0x81 (trip points changed) and 0x82 (device
lists changed). Each _ACx level runs the fans its _ALx names, by _FSL when
the fan has one, else by D0 and D3 through its _PR0 power resources,
counted across fans that share them; a level stays on until the zone is two
degrees below it, and a fan over a zone that cannot be read runs. The
hottest temperature and the trip points reached go to the kernel through
`MkAcpiEvent`. _CRT is believed on two readings a second apart; the kernel
then asks the desktop to shut down, and powers off itself if the zone is
still critical 30 seconds later. Passive cooling (_PSV) is reported, and
waits on processor performance control.

Every processor, a Processor object (by its ProcID) or an ACPI0007 device
(by its _UID), has its _PSS, _PCT, _PPC, _CST and _CPC read at start and
handed to the kernel through `MkAcpiCpu`, one record per processor, for
the kernel's frequency and idle control; a part whose register lies
outside memory, I/O, PCC or fixed hardware is left out. They are read
again on the processor's Notify 0x80, 0x81, 0x83 and 0x85. One `[ACPI]
cpu` line is logged for each run of processors with the same tables. Each
processor's `_OSC` (Intel's processor UUID) is evaluated first, since firmware
often loads those tables from it, declaring only the capabilities listed in
`cpu/caps.rs`, none until the kernel has the control code; firmware's answer is
logged on one `cpu _OSC` line. `_PDC` is not evaluated. Nothing is entered from
here.

\_S5 is evaluated at start too, with nothing run, and its SLP_TYP values
handed to the kernel through `MkAcpiSleep`, so soft-off works on a laptop
whose \_S5 is a method even when the shutdown does not come through
`capsule_power`.

A battery's _BTP is set to its warning capacity, so crossing into low
notifies. A discharging battery at or under its warning capacity is told
to the kernel once as low; one at its low capacity, or with _BST's
critical bit, is told as critical at every reading (every 10 s while it
lasts), and the kernel asks the desktop to shut down, then powers off 30 s
later if it still is. Each zone gets _SCP(0), active cooling first.

Display adapters above backlit outputs get _DOS(0): firmware keeps output
switching and the AC/battery brightness change. Their _DOD IDs are logged.

Hardware errors (ACPI 6.5, chapter 18): BERT is read once at start and the
fatal error the last boot ended in, if any, logged. HEST's enabled GHES and
GHESv2 sources are kept: polled ones are read at their interval (never
faster than 100 ms), SCI ones when an Error Device (PNP0C33) is notified.
Each record is logged on one `[ACPI] hw error` line by section type
(processor, platform memory, PCI Express) and counted by severity on the
kernel's posture line through `MkAcpiHwError`; the block is then given back
to firmware (status cleared, GHESv2's Read Ack written). Other source types
(machine check, AER, NMI) are listed, not served.

Hotkey sources are logged by the name firmware gives each event, never a
guessed meaning: WMI mappers (PNP0C14) with their _WDG event GUIDs, and on
Notify the GUID and the data _WED returns; Intel's HID event filter
(INT33D5) with HDEM's code; Intel's virtual buttons (INT33D6) with the
Notify value. The Intel codes are marked unverified until a laptop shows
them.

Backlit outputs are the devices defining both _BCL and _BCM (appendix B).
Their levels come from _BCL and their level now from _BQC; brightness is
not changed at start. The brightness keys arrive as Notify on the output,
or on the graphics adapter above it: 0x86 and 0x87 step up and down
through _BCL's levels and stop at the ends, 0x85 cycles, 0x88 goes to the
lowest, and each is set with _BCM. `_DOS` is not called, so firmware keeps
whatever switching and brightness policy it booted with.

Platform controllers with no PCI function (I2C, SD/eMMC and GPIO hosts
such as AMD's and Intel's LPSS parts) are published to the broker: each
present and enabled device whose _HID, or else a _CID, is on
`data/platform_ids.tsv` and whose evaluated _CRS names a memory window
goes to the kernel through `MkAcpiPlatformDevice` with its windows, its
first interrupt and its path (encoded by the kernel's own codec, mounted
from `src/hardware/broker/acpi/`), and becomes a claimable device the kernel
holds to its region rule. The list names where each id is documented;
every id in it is marked unverified. One `[ACPI] platform` line per
listed device: its broker id, the kernel's refusal, or why it was passed
over. No IOMMU domain confines them yet (no ANDD or IVRS ACPI-HID parse).

USB Type-C connectors are read through UCSI when firmware declares the
PPM (USBC000, or PNP0CA0 by _CID): the mailbox at its _CRS memory window,
the _DSM's write and read functions to signal it, and `nonos_ucsi` for
the protocol (PPM_RESET, notifications on, GET_CAPABILITY, each
connector's capability and status). A Notify on the device reads the
CCI; the connectors it names are read a few per pass and their changes
acknowledged. Read-only: no role swap, no alternate mode. A PPM that
stops answering costs at most 100 polls of 1 ms per command; it is reset
and walked again after 0, 5 and 10 seconds, then left alone. More than
32 notifications in a second are set aside for 2 seconds and every port
read after. One `[ACPI] ucsi` line per port that changes, and one per
change in the session's state. The UCSI constants are recalled, not
read from the published specification (unreachable when written).

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
it; Admin (0x200) for the ACPI calls and MkSmbiosTable, which the kernel gates on Admin
until a Firmware bit of their own is decided. Debug (0x100) is optional, for
its serial lines. No Mmio, Dma or Pio.

## Interface contract

Service `service:4530:acpi`, reply `reply:4531:endpoint.acpi.reply`, held
to `power` and `desktop_shell` (`src/services/registry/held_table.rs`). Requests go through
`MkIpcRecvFrom` and replies through `MkIpcReply`; the wire format is in
`src/server/protocol.rs`: an op byte, then a status word and the body.

Each op answers only the services named for it (`src/server/policy.rs`,
registry at each request; anyone else gets EPERM. BRIGHTNESS,
SET_BRIGHTNESS, POWER and THERMAL: `power` and `desktop_shell`.
PREPARE_SLEEP: `power`. SMBIOS: `desktop_shell`. SERIAL: the I2C
capsules. DEVICE_POWER: any sender the endpoint admits, moves only for
the device's holder. LEGACY: `driver.ps2_kbd0`. STATUS, DEVICE and any
other op: no one.

- STATUS (1): devices, routes, SCIs served, GPE methods run, refused region
  accesses, armed GPEs, notifications dropped.
- DEVICE (2, index): _STA, _ADR, _UID, _HID and the device's path.
- BRIGHTNESS (3), the brightness wire, which the graphics drivers' native
  backlight is to serve too: the number of panels (at most 4), then for each
  its identity (u32, the output's _ADR, matching the adapter's _DOD), its
  level now (0xFF unknown), and its levels: a count and the percentages.
  ENOENT with no backlit output.
- SET_BRIGHTNESS (4, percent[, panel u32]): one panel, or every panel when
  none is named, to its level nearest the percentage; EINVAL above 100,
  ENOENT for an unknown panel, EIO when a _BCM failed.
- PREPARE_SLEEP (5, state): a shutdown is coming. \_TTS(5) and \_PTS(5)
  run, \_S5 is evaluated again, and its SLP_TYP values go to the kernel
  through `MkAcpiSleep`. EINVAL for any state but 5; ENOENT when \_S5 is
  unusable; EPERM from anyone but `capsule_power`, which calls it before
  `AdminShutdown`.
  States 3 (S3) and 0 (suspend-to-idle) are coded but behind one switch,
  (Decision 6); while it is off they answer EINVAL as before. With it on,
  S3 runs \_TTS(3), arms only the wake devices `WAKE_DEVICES` names,
  runs \_PTS(3) and \_SI._SST(3), and is undone (\_WAK(3), disarm,
  \_TTS(0)) when the kernel's half answers NotWired, which it does until
  that half exists; suspend-to-idle calls the Low Power S0 Idle _DSM
  entry and exit notifications.
- POWER (6), the power wire: batteries and present ones; state bits (1
  charging, 2 discharging, 4 critical); the charge (0xFF unknown); the
  summed rate and the minutes left, to empty or full (u32, 0xFFFFFFFF
  unknown); AC (0, 1, 0xFF unknown); the unit (1 mA, 0 mW). Then the
  Type-C ports: a count (0 without UCSI, at most 8) and four bytes each,
  flags (attached, sourcing, a charger, can source, can sink, status
  unknown, session stopped), power operation mode, partner type and
  battery charging status. Layout in `src/server/typec_wire.rs`.
- THERMAL (7): the number of zones (at most 64), then per zone its
  temperature and _CRT in tenths of a kelvin (0 unknown or absent) and
  state bits (1 passive, 2 hot, 4 critical, 8 a fan it lists running).
- SERIAL (9, index): one device on an I2C, SPI or UART bus, from the
  evaluated tables (_STA, then _CRS, _DSM and _DSD, after _INI): bus kind,
  address or chip select, speed, the HID-over-I2C descriptor register from
  _DSM function 1, the GpioInt pin or GSI with its trigger, polarity and
  wake, the bus and GPIO controllers' _ADR, _UID and paths, and the count
  of such devices. Layout in `src/serial/wire.rs`. Held to `driver.i2c_pci0`
  and `driver.i2c_hid0` besides `power` and the shell. Each device is
  logged at start on one `[ACPI] serial` line.
- DEVICE_POWER (10, verb, state, device id): a device's D-state (ACPI
  6.5, chapter 7), a PCI function or an ACPI-bus device. Any caller the
  port admits may read it; only the pid holding the device's claim (asked
  of the kernel through `MkAcpiFunctionOf`, or `MkAcpiDeviceName` for an
  ACPI-bus device, found by its path or its `_HID` and `_UID`) may move it. A move turns the new state's power
  resources on in ResourceOrder, runs _PSx, then gives the old state's
  back; a fault undoes the move. D1 and D2 need _PSx or _PRx; a device in
  a low-power state goes back to D0 first. The fans and the devices
  share one power-resource count. A driver sets D0 after its claim, which
  powers up a device firmware left off. Layout in `src/dpower/wire.rs`.
  At boot, a device the tables describe under a PCI root (present and
  enabled by _STA, with _PS0 or _PR0, its _ADR reached only through real
  PCI-to-PCI bridges) whose function reads all ones gets its D0 and one
  `MkPciProbeFunction`: the kernel reads that function and adds it to the
  device table if it answers. One that does not is put back to D3cold at
  once; one no driver has claimed a minute after start is too.
- LEGACY (11, index): the legacy devices the namespace declares
  (PNP0303 keyboard, PNP0F13/PNP0F03 mouse, PNP0501/PNP0500 UART, PNP0B00
  RTC, PNP0103 HPET, PNP0100 PIT; by _HID or _CID, present by _STA) with
  their evaluated I/O, memory and interrupt resources. Every reply starts
  with the count, the 8042 verdict and the FADT's IAPC_BOOT_ARCH flags:
  an 8042 is there when the namespace declares a PS/2 port or the FADT
  says so, and otherwise the PS/2 driver leaves 0x60 and 0x64 alone.
  Layout in `src/legacy/wire.rs`. Logged at start, one `[ACPI] legacy`
  line each and one `[ACPI] 8042:` line.
- PROPERTIES (12, device id u64, offset u32): the device's _DSD
  properties (device-properties UUID; integers, strings, references as
  namespace paths, packages nested four deep) and the data-only subnodes
  the hierarchical data extension hangs below it (four levels, 32 nodes,
  a loop read once), read by `nonos_aml::props`. Only the pid holding the
  device's claim may ask, through the same kernel claim test as
  DEVICE_POWER. The reply is the whole length u32 and up to 504 bytes
  from `offset`; a driver asks again from where the last page ended.
  Layout in `src/props/wire.rs`; a driver mounts `wire*.rs` to read it.
  EIO for a tree past 16 KiB.
- SMBIOS (8): the machine's identity from SMBIOS (DMTF DSP0134), read
  once at start through `MkSmbiosTable` and walked by `nonos_smbios`: vendor,
  product, version, board vendor, board, BIOS vendor, version and date, and
  chassis, each a length byte and its text. Never a serial number or the
  UUID. ENOENT without SMBIOS. Logged at start on one `[SMBIOS]` line.

- Port facts, published once at start, not served on the port. Each USB
  host's ports (ACPI 6.5, 9.14: a container with _UPC or _PLD, three
  levels below the host) go to the kernel through `MkAcpiUsbPorts`, eight
  bytes a port: hub port above (0 for the root hub), port number, the
  _UPC facts (connectable, connector type), the _PLD facts (user-visible,
  lid, dock, panel) and a verdict, internal (not connectable, proprietary,
  hidden or in the lid), external (connectable and visible) or unknown. A
  host driver or hub walker reads them back by device id with
  `MkUsbPortFacts`. A PCIe port whose _DSD says ExternalFacingPort (and,
  beside it, DmaProperty) goes through `MkAcpiExternalPort`; the kernel
  reads the buses behind it from the port's own bridge registers and sets
  `DEVICE_FLAG_EXTERNAL` on the port and every function there, later ones
  included. Layout in `src/hardware/broker/acpi/usb_port_wire.rs`, mounted
  here by path. One `[ACPI] usb ports` or `[ACPI] external port` line each.

- Firmware-reserved ranges, published once at start before any device is
  woken. Every present PNP0C01 and PNP0C02 device's evaluated _CRS (I/O
  Port, Fixed I/O, 24-bit, 32-bit and 32-bit fixed memory, and the Word,
  DWord, QWord and Extended address space descriptors of the memory and
  I/O types; a relocatable range keeps its whole span) goes to the kernel
  in one list through `MkAcpiReserved`, 24 bytes a range. The broker keeps
  them, only ever adding, and refuses from then on any MMIO, PIO or DMA
  grant that shares a byte with one; a device's own BAR wholly inside a
  reservation passes. It checks MCFG's ECAM windows against them and adds
  `ECAM not reserved` to the posture line when one is not covered. Layout
  in `src/hardware/broker/reserved/wire.rs`, mounted here by path. One
  `[ACPI] reserved:` line.

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
