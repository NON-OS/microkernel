# Capsule isolation: The capability check on every NONOS system call, continued

Part of [Capsule isolation](capsule-isolation.md).

## The capability check on every NONOS system call, continued

A refusal is passed by `log_deny` to the kernel's structured log as `[CAP-DENY]
pid=<pid> syscall=<name>(<number>)` (`src/syscall/contract/dispatch.rs:43-46`).
That `log` writes only when a log manager is installed, and nothing in the
kernel calls its `init` (`src/log/manager/api.rs:26-53`), so in this release the
line is dropped. It reaches neither the screen nor the [serial
console](../overview/glossary.md#serial-console), and the Terminal's `log`
command does not show refusals.

Revoking a bit through `revoke` mints a new token and raises the epoch, so the
older token fails at the next call (`src/process/caps.rs:99-108`). A process
receives its [manifest](../overview/glossary.md#manifest)'s bits once:
`install_spawn` refuses a second install (`src/process/caps.rs:110-122`). After
that it gains bits only from an `Admin` holder: `sys_cap_grant` refuses a caller
without `Admin` and any bit the caller does not hold itself, while
`sys_cap_revoke` needs `Admin` alone
(`src/syscall/microkernel/capability/handlers.rs:33-66`).

Two limits apply to the check itself:

- The token is bound to a pid, an address space and a boot, not to a code
  measurement. `new_token` writes 32 zero bytes into `subject_measurement`
  (`src/process/caps.rs:52`).
- `Admin` stands in for no broker bit at the gate: `can_driver`, `can_mmio`,
  `can_irq`, `can_dma` and `can_pio` each ask for their own bit alone
  (`src/capabilities/token/types/authority_broker.rs:29-51`). A capsule holding
  `Admin` cannot claim a device, map its registers, take DMA buffers or bind its
  interrupts. Four `Capsule.mk` files in this tree request `Admin` in
  `CAPSULE_REQUIRED_CAPS`: `capsule_install`, `tool_install`, `capsule_policy`
  and `capsule_power` (`userland/capsule_install/Capsule.mk:25`,
  `userland/tool_install/Capsule.mk:23`,
  `userland/capsule_policy/Capsule.mk:19`,
  `userland/capsule_power/Capsule.mk:13`). `Admin` does not imply the right to
  enrol a signing root (`can_enrol_dev_root` in
  `src/capabilities/token/types/authority_admin.rs:43-52`).
