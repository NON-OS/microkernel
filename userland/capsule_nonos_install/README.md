# capsule_nonos_install

`capsule_nonos_install` is a console installer meant to run from the live USB. It walks a
seven step sequence (survey, compose, prove, mint root, enroll, write boot partition, print
receipt) and stops on the first step that refuses. The program is x86_64 assembly from
`_start` to the exit syscall: `src/main.rs` only pulls the files in `src/asm/` into one
`global_asm!` block and supplies the panic handler. It has no crate dependencies.

No kernel profile turns on `nonos-capsule-nonos-install`: the capsule is built, signed and
enrolled, but no image carries it. The disk installers that ship are `capsule_install` and
`install-cli`, described in [docs/handbook/apps/installer.md](../../docs/handbook/apps/installer.md).

## Role

`src/asm/entry.S` prints a banner, then for each entry of the `steps` table prints
`[k/7] <name>`, calls the handler, and exits with the 1-based step number on refusal or 0
when all seven ran. What each step does today:

1. `step_discover` (`discover.S`): lists devices through `SYS_DEVICE_LIST`, class 0, up to 16
   records.
2. `step_compose` (`compose.S`): reads `/install/set` from the ramfs service over IPC and
   echoes it; a medium without the file is refused.
3. `step_prove` (`proveset_step.S`, `proveset_one.S`, `attest.S`): prints the
   `SYS_ATTEST_STATUS` record, then for each name reads `/install/<name>.elf`, `.cert`,
   `.manifest` and `.trailer` and passes them to `SYS_CAPSULE_VERIFY`, printing the caps the
   summary reports. One refusal ends the run.
4. to 7. `step_mint_root`, `step_enroll`, `step_write_esp`, `step_receipt` (`steps.S`): print
   "pending" and return success.

## Capabilities

From `Capsule.mk`:

```make
# CoreExec | IPC | Memory | DeviceEnum = 0x8019: exactly what the landed
# steps call (src/asm/tags.S lists every syscall the program makes).
# The capset grows only when a step that needs more lands.
CAPSULE_REQUIRED_CAPS    := 0x00008019
```

Decoded against `src/capabilities/types/defs.rs`, 0x8019 is CoreExec, IPC, Memory and
DeviceEnum. IO and StoreWrite were held until 0.9.2 and admitted nothing the program
calls: IO gates no syscall, and no step writes the store yet.

## Interface

- Service endpoint `service:4956:app.nonos_install`, reply endpoint
  `reply:4957:endpoint.app.nonos_install.reply`, slug `nonos-install`.
- Output only: console writes via `SYS_STDOUT`, falling back to `SYS_DEBUG` when the console
  write is refused (`src/asm/console.S`). It reads no input.
- Syscall tags are defined in `src/asm/tags.S` and mirror the kernel's numbers.
- File reads use the ramfs wire protocol in `fs_locate.S`, `fs_open.S` and `fs_read.S`.

## State and privacy

All buffers are static `.bss` arenas, for example a 6 MiB ELF arena and a 512 KiB trailer
arena in `src/asm/prove.S`. The capsule writes nothing to any disk in its current form.

## Build and test

- Build: `make nonos-mk-nonos-install`, with `-sign` and `-verify` variants
  (`nonos-mk/capsule.mk`).
- There is no host test crate, and no CI job runs it.
- Design notes for the unfinished steps are in `design/write-authority.md` and
  `design/driver-instancing.md`.

## Not done yet

- Steps 4 to 7 are stubs (`src/asm/steps.S`), so the final "all steps ran" line does not
  mean anything was written.
- `design/write-authority.md` records that block drivers answered writes only from the
  kernel-internal client. They now also serve a sender that holds StoreWrite, which this
  capsule dropped from its word, so it cannot write a disk.
- The `SYS_DEBUG` fallback in `console.S` needs the Debug capability per
  `src/syscall/microkernel/debug.rs`, which `CAPSULE_REQUIRED_CAPS` does not include.
