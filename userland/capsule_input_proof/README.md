# capsule_input_proof

## Role

`capsule_input_proof` is the input end-to-end proof capsule. It is an app on
`nonos_app_skeleton`: it opens a plain window, subscribes to input through the
skeleton, and records the first key press, pointer motion and click it is
delivered, so a boot validation can assert that a hardware event observed by
a driver capsule reaches a focused window. It owns no hardware and paints only
a background; its product is a sequence of `MkDebug` markers ("surface
composited", "surface ready", "key down", "pointer motion", "focus routed")
ending in a PASS line (`src/proof/markers.rs`). The input path is described in
[Compositor](../../docs/handbook/desktop/compositor.md).

```text
driver.ps2_kbd0 / driver.usb_hid0
        |
        v
input_router -- OP_SUBSCRIBE / NINP delivery --> capsule_input_proof (window)
        |
        `-- MkDebug markers, then PASS
```

## Microkernel contract

```text
CAPSULE_REQUIRED_CAPS = 0x1819
CAPSULE_OPTIONAL_CAPS = 0x100
```

The optional bit is `Debug`. Only a `capsule-serial-debug` build grants it, so a kernel profile that
runs this proof must compile that feature for the markers to appear.

The required bits are CoreExec, IPC, Memory, GraphicsDisplayQuery and
GraphicsSurfaceCreate, for the window. `nonos_app_skeleton::run` does the
service lookups, the window, the subscription and the receive loop; the
capsule's own code adds only `MkDebug` for the markers.

## Interface contract

The capsule is a client, not a server. It exposes no operations of its own.
Endpoints: `service:4790:app.input_proof`, reply `4791`; the kernel mirror is
`src/userspace/capsule_input_proof`.

## Authority

The capsule may talk to the desktop services over IPC, draw its own window
and, on a serial-debug build, write to the debug surface. It has no PCI,
MMIO, IRQ, DMA, PIO, filesystem, network or focus-routing authority.

## Privacy and persistence

The capsule keeps no state across boots and writes no events to disk. The
markers it emits exist only on the ephemeral debug surface.
