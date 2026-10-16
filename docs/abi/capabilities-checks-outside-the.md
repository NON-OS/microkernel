# Capabilities: Checks outside the syscall table

Part of [Capabilities](capabilities.md).

## Checks outside the syscall table

Several bits are checked somewhere other than the syscall table, or nowhere that
runs:

- `Network`: an [endpoint](../overview/glossary-dma-pool.md#endpoint) in
  `NETWORK_SERVICES` requires it on top of `IPC`, through `required_caps`
  (`src/services/registry/policy.rs:26-44`). See [IPC](ipc.md).
- `RegisterService`: `caller_has_register_right` accepts it, or `Admin`, before
  a capsule may claim a runtime-registrable name
  (`src/services/registry/auth/caller_has_register_right.rs:17-20`).
- `SpawnBroker`: `attest` honours a capsule load's `on_behalf_of` pid only for a
  caller holding it
  (`src/kernel_core/process_spawn/capsule_spawn/attested_parent.rs:36-45`).
- `ProcessControl`: `sys_kill` lets a holder, or an `Admin` holder, end a
  process it does not parent or supervise
  (`src/syscall/microkernel/kill.rs:45-54`), and `sees_all` shows it the full
  process table (`src/syscall/microkernel/procstat_redact.rs:29-34`).
- `Keyring`: `gate_caller` refuses every operation of the kernel's keyring
  client to a caller without it
  (`src/security/keyring_capsule/capability.rs:26-35`). The client's only users
  are `mount_volume` and `format_volume`, and nothing calls either
  (`src/fs/blockfs_volume/mount_volume.rs:23-24`,
  `src/fs/blockfs_volume/format_volume.rs:22-25`), so the check never runs in
  0.9.2.
- `Entropy`: `gate_read` asks for it before the kernel client's `get_stats` and
  `healthcheck` (`src/security/entropy_capsule/capability.rs:23-31`), and no
  kernel code calls either.
- Neither bit gates a capsule that sends to the `keyring` or `entropy_pool`
  endpoint itself. Both endpoints ask for `IPC` alone, as every service endpoint
  outside `NETWORK_SERVICES` does. The keyring capsule ties each request to the
  sender's pid with `resolve_caller`
  (`userland/capsule_keyring/src/server/caller.rs:22-29`). [Randomness and
  cryptography](../security/randomness-and-cryptography.md#who-may-call-them)
  describes the same direct path to the entropy and crypto services.
- `IO` and `Hardware` are marked "Enforces nothing" in the table itself
  (`src/capabilities/types/defs.rs:21-29`).

## Rules the predicates add

- `Admin` stands in for `DeviceEnum` when listing devices and for `SpawnWindow`,
  through `can_device_enum` and `can_spawn_window`
  (`src/capabilities/token/types/authority_admin.rs:30-42`), and for
  `ProcessControl` in `sys_kill`, which reads the caller's bits itself
  (`src/syscall/microkernel/kill.rs:45-51`). It stands in for no broker or input
  bit: each predicate from `can_driver` to `can_input_consumer` asks for its own
  (`src/capabilities/token/types/authority_broker.rs:29-66`).
- `Admin` does not imply `EnrolDevRoot`: `can_enrol_dev_root` asks for that bit
  alone (`src/capabilities/token/types/authority_admin.rs:43-52`). It does not
  stand in for `DeviceEnum` on `MISR`, since `can_install_source` asks for
  `DeviceEnum` alone (`src/syscall/caps/checks/hardware.rs:28-31`). It does not
  open the full process table, since `sees_all` asks for `AttestRead` or
  `ProcessControl` (`src/syscall/microkernel/procstat_redact.rs:29-33`).
  `sys_pci_config_read` and `sys_pci_config_write` ask for `Driver` once more
  themselves (`src/syscall/microkernel/pci.rs:28-35`).
- The input ring has two rights: `can_input_source` takes `InputSource` to post,
  and `can_input_consumer` takes `InputConsumer` to drain and wait; `Irq`, which
  every driver holds, opens neither
  (`src/capabilities/token/types/authority_broker.rs:52-66`).
- `can_attest_doc` refuses a caller that holds `Network`, whatever else it
  holds, because a TPM quote names the machine for good
  (`src/syscall/caps/checks/system.rs:42-52`).
