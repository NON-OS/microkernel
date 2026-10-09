# capsule_boot_splash

## Role

`capsule_boot_splash` is the first visual userland client after handoff. It
waits for the compositor, paints a fullscreen boot splash, optionally displays
attestation detail after keyboard input, waits for the desktop shell to
register plus a one-second settle, and exits so the desktop fleet can take
over; `MAX_DWELL_MS` caps the wait at 30 s. The handbook page is
[System apps and services](../../docs/handbook/apps/system-apps.md).

```text
boot_splash -> compositor -> driver.virtio_gpu
      |
      `-> input_router for optional detail toggle
```

## Microkernel contract

- `MkIpcCall` talks to compositor and input-router services.
- `MkIpcRecvFrom` receives bounded key events.
- `MkMmap`, `MkSurfaceRegister`, `MkSurfaceShare` and `MkSurfaceRelease`
  own the temporary splash surface; the compositor maps and presents it.
- `MkServiceLookup` finds the compositor, the input router and the shell.
- `MkAttestStatus` reads the boot attestation status.
- `MkTimeMonotonic` (`mk_uptime_ms`), `MkYield`, and `MkExit` bound runtime and exit.

## Authority

`CAPSULE_REQUIRED_CAPS := 0x1018`: IPC, Memory and
GraphicsSurfaceCreate. It does not request the display query, network,
filesystem, crypto, hardware broker, DMA, PIO, IRQ, admin, or debug authority.

## Persistence

The capsule writes no files and holds no persistent state. It paints only into
its own temporary surface and releases that surface before exit.

## Build

`make nonos-mk-boot-splash`; sign with `make nonos-mk-boot-splash-sign`.
The kernel mirror is `src/userspace/capsule_boot_splash`.

## Evidence Status

Partially proven. The capsule is signed and attested with the committed fleet;
full boot-splash visual correctness still depends on QEMU/runtime boot proof.
