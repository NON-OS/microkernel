# capsule_power

## Role

`capsule_power` is the userland power-management service. It exposes
reboot and shutdown to other capsules via IPC. Reboot lands real on
every x86 box (ACPI reset register if the FADT offers one, 8042
pulse, both again, port 0xCF9, triple fault as last resort). Shutdown
enters ACPI S5 with the SLP_TYPa/b read from the constant `\_S5`
package in the DSDT or an SSDT (PM1a/PM1b control, or the sleep control
register on hardware-reduced platforms). A firmware whose `\_S5` is not
a constant package (it would need an AML interpreter to evaluate) or has
no PM1 control block cannot be powered off; the kernel then parks the
CPU. The handbook page is
[System apps and services](../../docs/handbook/apps/system-apps.md).

The kernel spawns it before the drivers (`src/userspace/capsule_power`,
`spawn_plan/platform_acpi.rs`) and registers its second name, `power.off`,
beside `power`: the desktop shell's request for the wiped power-off. It is

```text
desktop shell (alone: HELD, and the sender checked here)
    |
    | power.off POFF, or OP_REBOOT / OP_SHUTDOWN on power
    v
capsule_power -> mk_admin_reboot / mk_admin_shutdown
    |
    v
kernel admin_ops dispatcher
    |
    v
arch::x86_64::acpi::power_reboot::reboot()  (ACPI, 8042, ACPI, 8042, 0xCF9, triple fault)
arch::x86_64::acpi::power_sleep::shutdown() (S5 from the \_S5 package)
```

## Microkernel contract

- `MkIpcRecvFrom` on port `4448` reads `power` and `power.off` (port `4446`)
  requests, one inbox, and the sender's pid; `MkServiceLookup` and
  `MkThreadGroup` say whether that pid's process is `desktop_shell`.
- `MkIpcReply` returns each status.
- `AdminReboot` syscall invokes the kernel reboot path.
- `MkIpcCall` to `capsule_acpi` on port `4530`, PREPARE_SLEEP (5, state 5),
  before every shutdown, bounded at 3 s: the firmware's `_TTS(5)` and
  `_PTS(5)` run and `\_S5` is evaluated by the AML interpreter, and the
  kernel takes its SLP_TYP values. On a laptop whose `\_S5` is a method,
  which the kernel's constant reader cannot read, this is what lets it
  power off. When `capsule_acpi` is not running the call times out and the
  constant reader's values stand.
- `AdminShutdown` syscall invokes the kernel shutdown path. It does not
  return: the machine powers off, or parks when S5 is unavailable.
- `MkTimeMillis` reads the wall clock to track last-request timestamps.

## Interface contract

| Op | Value | Purpose |
|---|---|---|
| `OP_HEALTHCHECK` | 0x0001 | liveness ping |
| `OP_REBOOT` | 0x0002 | invoke `AdminReboot`; the system reboots |
| `OP_SHUTDOWN` | 0x0003 | prepare firmware through `capsule_acpi`, then invoke `AdminShutdown`; the system powers off |

Reboot, shutdown and `power.off` run for the desktop shell's process alone
(`off/rule.rs`); anyone else gets EPERM, a malformed `power.off` EINVAL.

The rest is in two parts: [authority and lifecycle](docs/authority-and-lifecycle.md)
and [wire, state and release](docs/wire-and-release.md).
