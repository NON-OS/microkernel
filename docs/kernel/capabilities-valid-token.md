# Capabilities: What each bit admits, continued

Part of [Capabilities](capabilities.md).

## What each bit admits, continued

Some calls need only a valid token: `MkExit`, `MkPidAlive`, `MkYield`, the futex
and time reads, `MkBatteryStatus`, `MkProcStat`, `MkAttestStatus`,
`MkAttestPolicy`, `MkBootAttest` and `MkCapCheck`, listed together under
`is_valid` (`src/syscall/contract/cap_table/mk.rs:25-39`). A valid token is one
that has not expired and grants at least one capability, as `is_valid` reads
(`src/capabilities/token/types/query.rs:44-47`). `MkSetTls` needs the same,
since it changes nothing outside the caller
(`src/syscall/contract/cap_table/mk_ipc.rs:27-31`). `MkTtyQuery` admits every
caller whose token passed the resolver
(`src/syscall/contract/cap_table/mk_surface.rs:51`). `MkProcStat` shows a caller
without `AttestRead` or `ProcessControl` only the identity and state of other
processes, as `sees_all` decides
(`src/syscall/microkernel/procstat_redact.rs:29-34`).

Two refusals are written into the predicates themselves. `can_attest_doc`
refuses a caller that holds `Network`, because a TPM quote names the machine for
good (`src/syscall/caps/checks/system.rs:42-47`). `can_input_consumer` takes
`InputConsumer` alone, so neither a device driver, which holds `Irq`, nor a
holder of `Admin` reads the keystroke stream
(`src/capabilities/token/types/authority_broker.rs:64-66`).
