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
