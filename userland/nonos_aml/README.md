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
