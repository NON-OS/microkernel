# capsule_setup_wizard

## Role

`capsule_setup_wizard` owns first-boot setup. It discovers the compositor,
input-router and policy services, takes the whole screen with a surface and
the keyboard, walks the person through thirteen steps (keyboard, name, time
zone, mode, network, network route, privacy, appearance, Qwen model, apps,
installed software, computer name, review) and submits the choices to the
policy service. Its exit status decides what the kernel starts next. The
handbook page is
[docs/handbook/apps/installer.md](../../docs/handbook/apps/installer.md).

```text
setup_wizard -> compositor
      |       -> input_router
      `       -> policy capsule, vfs
```

## Microkernel contract

- `MkIpcCall` talks to compositor, input-router, policy and vfs services.
- `MkIpcRecv` receives setup input events.
- `MkSurfaceRegister` and `MkSurfaceShare` own the setup surface.
- `MkExit` ends the wizard. The low byte is 0 to start the desktop or 3 to
  hand the whole screen to the installer with no desktop behind it; the byte
  above carries the apps the person turned off. 2 means setup could not draw,
  and the desktop starts without it.
- `MkProcStat` reads this machine's memory, which the Qwen step fits tiers to,
  and the install flag the boot menu set. For an install the Qwen step starts
  on the largest Qwen3 that fits; under QEMU's software CPU (CPUID's
  hypervisor vendor `TCGTCGTCGTCG`, read as the Linux personality reads it)
  on the smallest. On an amnesic stick (a NONOS disk loaded; setup runs only
  where no answers were kept, and an installed disk is written with them) the
  kernel holds the data volume in memory for the session, so the step says a
  model is downloaded into memory and gone at power off, offers only tiers
  whose download and run fit in memory less the larger of 1 GiB and a quarter
  (the kernel's keep, with the fetcher's `need.rs`), and starts on none. With
  no NONOS disk nothing is offered (`src/qwen/default.rs`, held by
  `setup_layout_proofs`).
- Service `service:4794:app.setup_wizard`, reply
  `reply:4795:endpoint.app.setup_wizard.reply`.

## Authority

`CAPSULE_REQUIRED_CAPS := 0x8001879`: CoreExec, IPC, Memory, Crypto, FileSystem,
GraphicsDisplayQuery, GraphicsSurfaceCreate, and EnrolDevRoot. Debug is
optional (`0x100`): only a `capsule-serial-debug` build grants it. Crypto derives
the TPM key that seals a Wi-Fi network setup is asked to remember. FileSystem
keeps what setup decides (the answers, the consent, the remembered network)
through vfs, which serves only a holder of it. EnrolDevRoot backs
`mk_local_consent_grant` and `mk_local_consent_revoke`, the consent that lets
this machine run what it installs. It has no network, store-write,
hardware, DMA, PIO, or IRQ authority.

The capsule is turned on by `nonos-capsule-setup-wizard` in the
`microkernel-setup-wizard` profile: the offline desktop, the tool registry and
setup, with no network or serial console. `microkernel-full-gui` includes that
profile, and `nonos-mk-desktop-gui-prod` builds it beside
`microkernel-desktop-gui`.

## Persistence

In amnesic mode nothing is kept. In install mode the answers record goes to
`/nonos/setup/answers` and the marker `NSD1` to `/nonos/setup/done` through vfs,
answers first, and is persisted to the store. The record (version 5) holds the
keyboard, time zone, wallpaper, name, Qwen tier, the apps turned off, the
computer's name and the network route. On a later boot setup finds both,
restores the consent, waits for the policy service and exits without drawing:
with 3 on an install boot, 0 otherwise. The installer carries the record to
the disk it writes, so setup does not run there either.

A Wi-Fi network joined on the network step is remembered only when asked and
only in install mode, sealed under a TPM-derived key in `/nonos/wifi/saved`.

## Tests

`userland/setup_layout_proofs` checks the layout on every supported canvas:
regions on the canvas and in order, every screen's content above the keys,
and every text literal fitting its box.
