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
- `MkMmap`, `MkSurfaceRegister`, `MkSurfaceShare`, `MkSurfaceRelease` and
  `MkMunmap` own the temporary splash surface; the compositor maps and
  presents it.
- `MkServiceLookup` finds the compositor, the input router and the shell.
- `MkAttestStatus` reads the boot attestation status.
- `MkTimeMonotonic` (`mk_uptime_ms`), `MkYield`, and `MkExit` bound runtime and exit.

## Nothing of the firmware's

The splash paints its whole canvas: a wash over every pixel first, then
NONOS's own title, the attestation panel, the warning's row and the status
line, laid out for the canvas (`src/canvas.rs`, `src/layout_fit.rs`). No
firmware or vendor logo is kept or drawn, at the owner's word. The
compositor holds its first frame until this surface is up, so the boot
shows no flash of the firmware's screen.

## The attestation panel

Each line says what this boot recorded, from MkAttestStatus, and nothing
else (`src/attest_lines.rs`):

| line | from | says |
|---|---|---|
| bootloader | `secure_boot` | `secure boot verified`, or `secure boot off` |
| kernel | `kernel_sig_ok`, the loader's ed25519 check | `ed25519 verified`, or `signature not verified` |
| capsules | `zk_verified` | `zk attested` (STARK), `path only (development)` (Merkle path alone), or `not attested` |

The verdict under them is `ATTESTED` only when the kernel's signature and
the STARK both held, and `UNVERIFIED` otherwise; secure boot has its own
line, since a test machine may run without it. With no record every line
reads `not reported` and the verdict `NO RECORD`. A development image shows
`path only (development)` and `UNVERIFIED`.

## Without an IOMMU

When the kernel marks `BOOT_DMA_UNCONFINED` in MkProcStat's header (the
no-IOMMU posture), the splash draws `device DMA not confined by hardware`
in red, in a row the layout keeps between the panel and the status line
whether or not it is drawn (`src/dma.rs`, `src/warning.rs`, `src/layout.rs`).
MkProcStat needs no capability.

## A canvas that changes size

The compositor can move from the firmware framebuffer to a GPU driver at
another size while the splash is up (on QEMU, 1280x800 to 1920x1080). The
splash asks DISPLAY_INFO again every 250 ms, and on a new size draws itself
afresh at that size, its layout placed for it, and submits it at the
same depth, so the compositor swaps it in before the old surface is given
back (`src/follow.rs`, `src/show.rs`). It redraws at most four times, and its
time on screen carries across a redraw.
Proofs: [boot_splash_proofs](../boot_splash_proofs/README.md).

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
