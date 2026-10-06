# Capabilities

How the NONOS kernel decides what a process may ask of it: the [capability](../overview/glossary.md#capability) bits, the token each process holds, the check on every system call, and how bits are granted and taken back.

## One definition

The kernel names every capability once, in the `capability_table` list, with the bit it occupies (`src/capabilities/types/defs.rs:17-83`). There are 36, on bits 0 to 35, and `count` is derived from the list so it cannot drift from it (`src/capabilities/types/table.rs:39-43`).

[abi/caps.toml](../../abi/caps.toml) publishes the same bits for toolchains and [capsule](../overview/glossary.md#capsule) authors. Its `[bits]` table, starting at `CORE_EXEC`, is generated from the kernel list by `scripts/gen_caps_abi.py` (`abi/caps.toml:3-6`). The kernel never reads the file; `scripts/check_caps_abi.py` compares the two:

```
$ python3 scripts/check_caps_abi.py
caps-abi: 36 published bits agree with the kernel
```
