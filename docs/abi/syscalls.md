# Syscalls

Every native syscall NONOS 0.9.2 accepts, with its number, the [capability](../overview/glossary.md#capability) that admits it, and what it does.

## Numbers

A syscall number is a [syscall tag](../overview/glossary.md#syscall-tag): four ASCII letters packed little-endian into a `u64` by `tag4`, first letter in the lowest byte (`src/syscall/abi/tag.rs:17-22`). `MISD` is the bytes `M`, `I`, `S`, `D`, so its number is `0x4453494D`. Any number not in `REGISTRY` is answered with `ENOSYS`, -38, except from a [foreign process](../overview/glossary.md#foreign-process) on x86_64, whose supervisor answers it; see [The NONOS ABI](README.md#calling-convention).

`REGISTRY` holds one table per family, 130 calls in all (`src/syscall/abi/registry/mod.rs:27-28`):

| Family | Registry file | Calls |
|---|---|---|
| `mk` | `src/syscall/abi/registry/mk.rs` | 114 |
| `crypto` | `src/syscall/abi/registry/crypto.rs` | 12 |
| `admin` | `src/syscall/abi/registry/admin.rs` | 3 |
| `graphics` | `src/syscall/abi/registry/graphics.rs` | 1 |

The first letter of a tag names the family: `M` microkernel, `C` crypto, `A` admin, `G` graphics.
