# capsule_driver_vmware_svga

## Role

`capsule_driver_vmware_svga` drives the SVGA II display adapter (PCI
15ad:0405): VMware Workstation's, Fusion's and ESXi's, VirtualBox's VMSVGA,
and QEMU's `vmware-svga`; and VirtualBox's VBoxSVGA, the same registers
under 80ee:beef with its BARs in VirtualBox's order. It sets the mode, keeps the compositor's surface,
copies what the compositor drew into the framebuffer and asks the host to
redraw it. The compositor reaches it exactly as it reaches virtio-gpu.

```text
compositor --NVGP (get primary, transfer, set scanout, flush)--> driver.vmware_svga0
    |                                                                |
    | draws into the shared surface                                  | copy into VRAM (BAR1)
    v                                                                v
kernel surface registry                                SVGA_CMD_UPDATE in the FIFO (BAR2)
```

## Microkernel contract

- `MkDeviceList` finds the adapter: 15ad:0405 with an I/O BAR0 and memory
  BARs 1 (framebuffer) and 2 (FIFO), or 80ee:beef with the framebuffer in
  BAR0 and the ports in BAR1 (VBoxSVGA; VBoxVGA, with no port window, is
  left alone).
- `MkDeviceClaim`, then `MkPioGrant` on the port BAR for the index and value
  ports.
- `MkMmioMap` maps the FIFO (the size SVGA_REG_MEM_SIZE gives) and the
  visible framebuffer only (from FB_OFFSET for `pitch * height`).
- `MkMmap` and `MkSurfaceRegister`/`MkSurfaceShare` make the primary surface
  in the capsule's own memory; the framebuffer is never handed to another
  process.
- No `MkDmaMap`, no `MkIrqBind`.
- `MkIpcRecv` serves `service:4280:driver.vmware_svga0`; the kernel holds the
  endpoint to the compositor.

## Interface contract

| Op | Value | Meaning | Reply |
|---|---|---|---|
| `OP_HEALTHCHECK` | 0x0001 | still in SVGA mode with the FIFO configured | status |
| `OP_TRANSFER_TO_HOST` | 0x0008 | copy a rectangle of the surface into VRAM | status |
| `OP_SET_SCANOUT` | 0x0009 | the primary resource on scanout 0 (already showing) | status |
| `OP_FLUSH` | 0x000A | SVGA_CMD_UPDATE for a rectangle | status |
| `OP_GET_PRIMARY_SURFACE` | 0x000C | surface handle, resource 1, width, height, stride, B8G8R8A8 | 32 bytes |

The first sender of `OP_GET_PRIMARY_SURFACE` owns the surface; any other
sender's draw is refused with `-16`. Other errors: `-22` bad request, `-38`
unknown op, `-110` the device (a FIFO that stayed full, a lost SVGA mode).

## Authority

`IPC`, `Memory`, `GraphicsSurfaceCreate`, `Driver`, `DeviceEnum`, `Mmio`,
`Pio` (`CAPSULE_REQUIRED_CAPS = 0x139018`); `Debug` optional
(`CAPSULE_OPTIONAL_CAPS = 0x100`). No `Dma`, `Irq` or `Network`.

```text
allowed:   SVGA II claim, its ports, FIFO and visible framebuffer, one shared surface
forbidden: device memory in another process, DMA, input, network
```

## Privacy and persistence

Pixels live in the surface and VRAM only. Nothing is captured or kept.

## Runtime lifecycle

Claim, port grant; SVGA_REG_ID 2 agreed; the firmware's mode registers saved;
the FIFO mapped and laid out as Linux's vmw_fifo_create does (CONFIG_DONE
dropped, MIN at least a page in, MAX, NEXT_CMD and STOP at MIN, BUSY cleared
on an extended FIFO, CONFIG_DONE raised); the mode the firmware left kept
when it is 32-bit and fits, else 1024x768, else a smaller one VRAM holds;
WIDTH, HEIGHT, BITS_PER_PIXEL 32, ENABLE; BYTES_PER_LINE and FB_OFFSET read;
the visible framebuffer mapped; the surface made; serve.

## Failure model

Every step names its failure. One after the FIFO is touched writes the
firmware's mode registers back (the compositor may still be drawing into the
firmware's framebuffer), then gives back every map, the port grant and the
claim. A full FIFO is waited on for 100 ms, pinging the host with SYNC, and
the update dropped past that; the next frame redraws the area. Bring-up is
tried a bounded number of times (`nonos_libc::start_driver`).

## Current implemented surface

- One screen, 32 bits per pixel, the firmware's resolution or 1024x768.
- Software copy into VRAM; SVGA_CMD_UPDATE written a word at a time with
  NEXT_CMD moved after each, which every SVGA II host reads.

## Wire format

`NVGP` header (magic 0x4E564750, version 1, 20 bytes), payload length exactly
matching the message, then a status word and the op's body.

## State ownership

The capsule owns the FIFO, the framebuffer mapping and the surface. The
compositor owns what is drawn.

## Operating rules

- Never hand device memory to another process.
- A transfer's offset must be where its rectangle starts in the surface.
- Put the firmware's mode back on any failed attempt.

## Release target

The desktop at the resolution the firmware chose, in VMware, VirtualBox
(VMSVGA and VBoxSVGA) and QEMU (`vmware-svga`).

## Release evidence

`vmware_svga_proofs` runs the whole bring-up against a model of the adapter's
registers and FIFO with the broker on host memory: the FIFO layout, the mode
choice, the pitch, every refusal and what it gives back, transfers, flushes
across thousands of commands and the ring's wrap, and a host that stops
reading. A boot is the owner's.

## Release checklist

- Signed manifest and kernel mirror present (`driver_vmware_svga` keys).
- `vmware_svga_proofs` passes.
- QEMU `-vga none -device vmware-svga`, VMware, and VirtualBox with VMSVGA
  or VBoxSVGA show the desktop and redraw as it changes.

## Explicit non-goals today

No resize from host hints yet (unit D 10 brings the hints), no hardware
cursor, no SVGA3D, no GMR or screen objects, no SVGA3 (15ad:0406).

## Verification

- Build: `make -B nonos-mk-driver-vmware-svga`
- Proofs: `cd userland/vmware_svga_proofs && cargo test --release`
- Static gate: `bash nonos-ci/run-static-checks.sh`
