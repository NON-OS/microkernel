# Capabilities: Changing capabilities at run time

Part of [Capabilities](capabilities.md).

## Changing capabilities at run time

Three calls work on another process's mask. `sys_cap_grant` needs `Admin`, and
the caller must already hold every bit it grants, so a grant never creates
authority (`src/syscall/microkernel/capability/handlers.rs:33-52`).
`sys_cap_revoke` needs `Admin`
(`src/syscall/microkernel/capability/handlers.rs:54-66`). Both mint the target a
fresh token, and `revoke` also raises the target's revocation epoch first, so a
copy of its old token fails `check_revocation_epoch`
(`src/process/caps.rs:99-108`). `sys_cap_check` returns 1 when the target holds
every bit of the mask and 0 otherwise, for any caller with a valid token
(`src/syscall/microkernel/capability/handlers.rs:68-74`). `u32_arg` refuses the
pid argument, rather than truncating it, when it does not fit in 32 bits
(`src/syscall/microkernel/dispatch/capability.rs:30-31`).

## Groups and delegation

`abi/caps.toml` also publishes named groups and the rights each group may hand
to another. They are policy, written by hand. The file says each group is the
mask of a real capsule, `SERVICE` for one being the keyring's
(`abi/caps.toml:48-51`). None matches its capsule exactly at this commit: the
keyring, for one, declares `IPC`, `Memory` and `Crypto`, 0x38
(`CAPSULE_REQUIRED_CAPS`, `userland/capsule_keyring/Capsule.mk:17`). The kernel
does not read this file.

| Entry | Capabilities |
|---|---|
| `BASIC` | `CORE_EXEC`, `MEMORY` |
| `SERVICE` | `CORE_EXEC`, `MEMORY`, `IPC` |
| `OPER` | `CORE_EXEC`, `MEMORY`, `IPC`, `PROCESS_CONTROL` |
| `GRAPHICS_SERVICE` | `CORE_EXEC`, `MEMORY`, `IPC`, `GRAPHICS_DISPLAY_QUERY`, `GRAPHICS_SURFACE_CREATE`, `GRAPHICS_SURFACE_MAP`, `GRAPHICS_PRESENT` |
| `BASIC_to_BASIC` | `CORE_EXEC`, `MEMORY` |
| `OPER_to_SERVICE` | `CORE_EXEC`, `MEMORY`, `IPC` |
| `OPER_to_BASIC` | `CORE_EXEC`, `MEMORY` |
| `SERVICE_to_BASIC` | `CORE_EXEC`, `MEMORY` |

## Limits

- The `[token]` section of `abi/caps.toml` does not describe the kernel's token:
  `id_bytes` and `mac_bytes` say 32, while `CapabilityToken` holds a `u64`
  `token_id` and a 64-byte `signature` (`abi/caps.toml:58-65`,
  `src/capabilities/token/types/defs.rs:22-32`), which `sign_token` fills with
  `mac64` (`src/capabilities/token/sign.rs:24-31`). The `algo` the `[mac]`
  section publishes is `sha3-256` (`abi/caps.toml:67-70`). The `mac64` the
  kernel computes is two keyed BLAKE3 hashes
  (`src/capabilities/token/material.rs:41-51`).
- `check_caps_abi.py` checks bit values, and `read_abi` collects every name a
  group or delegation line uses, each of which must be a published bit
  (`scripts/check_caps_abi.py:71-81`). Which capabilities a group should hold is
  policy, and no script judges it.

## See also

- [The NONOS ABI](README.md)
- [Syscalls](syscalls.md)
- [Kernel capabilities](../kernel/capabilities.md)
- [Manifests and capabilities](../userland/manifests-and-capabilities.md)
- [Shipping an app: choosing the capability
  word](../userland/shipping-an-app.md#choosing-the-capability-word)
- [Capsule isolation](../security/capsule-isolation.md)
- [abi/caps.toml](../../abi/caps.toml)
