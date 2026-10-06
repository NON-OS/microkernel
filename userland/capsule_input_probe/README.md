# capsule_input_probe

## Role

`capsule_input_probe` is an input-stack test capsule. It subscribes to input
events, takes a keyboard grab from the input router (it is one of the five
services `GRABBERS` admits), renders observed events to a compositor surface,
and serves the input end-to-end boot tests. The input path is described in
[Compositor](../../docs/handbook/desktop/compositor.md).

```text
input drivers -> input_router -> input_probe -> compositor -> driver.virtio_gpu
```

## Microkernel contract

- `MkServiceLookup` finds the compositor and the input router.
- `MkIpcCall` talks to them: scene submit, subscribe, `OP_GRAB_REQUEST`.
- `MkIpcRecvFrom` receives routed input events.
- `MkMmap`, `MkSurfaceRegister`, `MkSurfaceShare` and `MkSurfaceRelease`
  make the probe surface; the compositor maps and presents it.
- `MkExit` terminates the capsule.

## Authority

`CAPSULE_REQUIRED_CAPS := 0x1018`: IPC, Memory and
GraphicsSurfaceCreate. The capsule is a test
consumer of input events and has no direct device, IRQ, PIO, DMA, filesystem,
network, crypto, admin, or debug authority.

## Persistence

No persistent state. Event history is in RAM only and is lost on exit or reboot.

## Evidence Status

Partially proven. Source-level contract exists; full proof requires the PS/2
and xHCI input boot tests to pass on the target.
