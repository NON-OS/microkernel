# Syscalls: Numbers

Part of [Syscalls](syscalls.md).

## Numbers

A syscall number is a [syscall tag](../overview/glossary-stark-proof.md#syscall-tag): four
ASCII letters packed little-endian into a `u64` by `tag4`, first letter in the
lowest byte (`src/syscall/abi/tag.rs:17-22`). `MISD` is the bytes `M`, `I`, `S`,
`D`, so its number is `0x4453494D`. Any number not in `REGISTRY` is answered
with `ENOSYS`, -38, except from a [foreign
process](../overview/glossary-dma-pool.md#foreign-process) on x86_64, whose supervisor
answers it; see [The NONOS ABI](abi-calling-convention.md#calling-convention).

`REGISTRY` holds one table per family, the microkernel's in four parts, 179
calls in all (`src/syscall/abi/registry/mod.rs:30-38`):

| Family | Registry file | Calls |
|---|---|---|
| `mk` | `src/syscall/abi/registry/mk.rs`, `mk2.rs` to `mk4.rs` | 163 |
| `crypto` | `src/syscall/abi/registry/crypto.rs` | 12 |
| `admin` | `src/syscall/abi/registry/admin.rs` | 3 |
| `graphics` | `src/syscall/abi/registry/graphics.rs` | 1 |

The first letter of a tag names the family: `M` microkernel, `C` crypto, `A`
admin, `G` graphics.
