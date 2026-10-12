# nonos_aml

NONOS's own ACPI Machine Language interpreter, written from the ACPI 6.5
specification (UEFI Forum) and nothing else. It runs inside `capsule_acpi`,
never in the kernel: on other systems AML runs in ring 0, where one firmware
bug or one hostile SSDT reaches all of memory. Here a table that loops, faults
or asks for a region it was not granted stops the capsule, not the machine.
`no_std` with `alloc`, no dependencies.

## What is here

| Module | What it holds | Specification |
|---|---|---|
| `block` | the definition block header, length and checksum | 5.2.6 |
| `grammar` | PkgLength, NameSeg and NameString, a bounded cursor | 20.2.2, 20.2.4 |
| `op` | every opcode value, as `Op` | 20.3 |
| `shape` | what follows each opcode, one row per operand list | 20.2.5 |
| `arena` | the node arena and its limits | |
| `parse` | the term list read into the arena, nothing executed | 20.2 |
| `namespace` | the object tree, predefined root objects, search rules | 5.3, 5.7 |
| `load` | tables loaded in order into the namespace; field unit layout | 5.2.11, 19.6.48 |
| `object` | values, references, the implicit and explicit conversions | 19.3.5, 19.6 |
| `exec` | the interpreter: methods, control flow, operators, mutexes, events, the budget | 19 |
| `region` | field access arithmetic: widths, units, UpdateRule, the Global Lock's steps | 5.2.10.1, 19.6.48 |
| `exec/field` | regions, field units, IndexField, BankField, buffer fields, _REG, Load | 5.5.2.4, 6.5.4, 19.6 |
| `events` | the SCI: FADT event blocks, fixed events, GPE blocks and methods, ACPI mode | 4.8, 5.2.9, 5.6.4 |

## Reading a table

`Block::new` refuses a table whose length field or checksum is wrong, or
that is larger than the limit. `parse::load` reads its term list into an
`Arena`: one node per object, with its operands in grammar order. A method
body is kept as bytes (a `DEFERRED` node) and read by `parse::body` once
every table is loaded, because the bytes alone never say how many arguments
a call passes: only the called method's definition, or an `External`, does.
The parser asks the namespace through the `Scopes` trait.

A name in a SuperName is never called, though the grammar allows a call
there: `CondRefOf(\_OSI)` names a method of one argument and passes none.

## Loading the namespace

`Aml::load_table` takes the DSDT first and only first, then SSDTs and PSDTs
in the order the XSDT lists them. Each table is parsed, kept with its arena,
and walked: every named object becomes an `Entry` of its `Kind`, with its
definition left in the arena for the interpreter. Field units are laid out
from their FieldList as they are made. An `If` at load time is evaluated by
the interpreter through `LoadExec` and only the branch it chooses is loaded;
any other load-time statement is handed to `LoadExec::run`.

The predefined objects (`\_GPE`, `\_PR`, `\_SB`, `\_SI`, `\_TZ`, `\_GL`,
`\_OS`, `\_OSI`, `\_REV`) exist before any table, so a table cannot define
its own in their place. A lone NameSeg is searched for in the current scope
and each scope above it; every other path is followed exactly; definitions
are never searched. A name defined twice keeps its first definition and is
noted; a container defined again over a container still has its TermList
loaded into the first, so nothing inside either is lost. Missing scopes,
parents and alias targets are noted, never guessed, and the notes are
bounded at 256.

## Running code

`Machine` is one evaluation. `evaluate(aml, id, args)` runs a method, reads
a Name, or returns a reference to any other object; `load_table` loads a
table running its load-time code. Everything an evaluation spends is
counted against `ExecLimits` (2 M steps, 24 nested calls, 8 MiB made, 10 s
of Sleep, Stall and timed waits by default): a method that loops, recurses,
allocates or waits without end stops with a `Fault` naming the table and
offset, and the mutexes it held are released.

- Integers are 64 bits, or 32 when the DSDT's revision is below 2.
- Arguments are copied in; a reference passed by RefOf is written through.
  A Store to a Name converts to the Name's type and a buffer keeps its
  length; CopyObject replaces. What a method defines is removed when it
  returns, and its slot's generation changes, so a reference kept past
  that is stale, not dangling.
- Mutexes belong to one evaluation, nest, and are taken only in rising sync
  level order; serialized methods take their own. A wait that nothing in
  the evaluation could end is a fault, not a hang.
- Notify is queued (64 deep, overflow counted) for the capsule; Sleep,
  Stall, Timer, Debug and Fatal go to the `Host`.
- \_OS, \_OSI and \_REV answer from a `Personality` (below).

## Regions and fields

Every register access goes through the `Host`: `region_read` and
`region_write` for SystemMemory, SystemIO, PCI_Config, EmbeddedControl and
SystemCMOS; `transaction` for SMBus, IPMI, GenericSerialBus and PCC fields,
whose data buffer goes in and whose response comes back in the source
buffer; `gpio` for pins named by a field's Connection; `table` for
DataTableRegion (read from the platform's copy, never written) and
LoadTable; `lock_word` and `lock_released` for the FACS Global Lock.

- A field takes one access per access-width unit it touches. AnyAcc takes
  the narrowest naturally aligned width up to eight bytes that holds the
  field in one access; embedded controller and CMOS fields are read by
  bytes. A partial unit is written by the field's UpdateRule; the written
  word is cut to the access width. A field wider than an integer reads as a
  buffer. An access past the region's end is a fault.
- IndexField writes the byte offset to its index field, then moves the data
  field. BankField writes its bank value before each access. A Lock field,
  and Acquire(\_GL), hold the Global Lock (pending marked, firmware waited
  for by polling under the time budget; the SCI's GBL_STS is cleared by the
  event layer, and `Pm1Cnt::release_lock` writes GBL_RLS).
- A PCI_Config region carries the _ADRs from its device up to the host
  bridge (PNP0A03/PNP0A08) and the bridge's _SEG and _BBN.
- A space's accesses fault until `Machine::connect(aml, space)` marks its
  handler ready; `connect` runs _REG(space, 1) in every scope that holds a
  region of the space, and reports the _REGs that faulted.
- A region or buffer field a method defines resolves then, in the method's
  frame, so its arguments are in scope; it is gone when the method returns.
- Load (from a region, a field or a buffer), LoadTable (by signature and
  OEM ids, under RootPathString, with ParameterData stored) and Unload; what
  a loaded table defines outlives the method that loaded it.

## _OSI, _OS and _REV

Firmware written for Windows asks `_OSI` for the version strings Microsoft
publishes ("How to Identify the Windows Version in ACPI by Using _OSI",
learn.microsoft.com) and turns devices and features on by the answers. The
default `Personality` answers each of them, "Windows 2000" to "Windows 2022"
(`exec::WINDOWS`, each line naming its release), as Windows does for its
own string and every earlier one; `Personality::windows_up_to` caps the
answers at a release for a machine that needs it. The feature group strings
of section 5.7.2 and vendor strings such as "Linux" are not answered: the
first promise support NONOS does not have yet, the second select paths
tested against another system. `_OS` is "Microsoft Windows NT"; `_REV` is 2.

## Events

`events::parse_fadt` reads the FADT the kernel handed over for the SCI,
SMI_CMD and ACPI_ENABLE, and the PM1 and GPE blocks, preferring an X_ field
that gives an address. Only System I/O blocks are reached; a block in memory
is refused. `enter_acpi_mode` writes ACPI_ENABLE to SMI_CMD and waits,
bounded, for SCI_EN. `handlers` finds \_GPE's _Lxx and _Exx methods and
`wake_gpes` the events devices' _PRW name.

`Events::new` arms every event that has a method, sits in a block and is no
device's wake event; wake events are never enabled. `start` turns every GPE
off and clears it before arming, and enables in PM1 only the Global Lock's
release, plus the power and sleep buttons if the `Policy` says so (off by
default: the kernel reads the power button itself). `on_sci` clears the
fixed events it serves, then for each pending armed GPE: disable, clear an
edge event's status, run its method through the caller's evaluator, clear a
level event's status, re-enable. A level event whose method fails stays
disabled, since its source is still asserting. Register access goes through
`EventIo`, in the capsule MkAcpiPort.

## Limits

`Limits::DEFAULT`: a 4 MiB table, 64 levels of nesting (the parser recurses
once per level, so this bounds its stack), 1 Mi nodes per arena. Anything past
a limit, a malformed PkgLength, a package reaching past its parent, a bad
NameSeg or a string byte outside 0x01-0x7F refuses the whole table with the
offset it was found at; nothing is guessed. A failed method body leaves the
arena as it was.

## Proofs

`userland/nonos_aml_proofs` mounts these sources by `#[path]` and builds its
fixtures with its own AML encoder.
