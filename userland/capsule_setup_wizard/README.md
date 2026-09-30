# capsule_setup_wizard

## Role

`capsule_setup_wizard` owns first-boot setup UI. It discovers compositor,
input-router, and policy services, paints a guided setup surface, receives
input events, and submits policy choices through explicit capsule IPC.

```text
setup_wizard -> compositor
      |       -> input_router
      `       -> policy capsule
```

## Microkernel contract

- `MkIpcCall` talks to compositor, input-router, policy, and vfs services.
- `MkIpcRecv` receives setup input events.
- `MkSurfaceRegister` and `MkSurfaceShare` own the setup surface.
- `MkExit` ends the wizard: 3 asks the kernel to open the installer, 0 does not.

## Authority

`CAPSULE_REQUIRED_CAPS := 0x8001939`: CoreExec, IPC, Memory, Crypto, Debug,
GraphicsDisplayQuery, GraphicsSurfaceCreate, and EnrolDevRoot. Crypto derives
the TPM key that seals a Wi-Fi network setup is asked to remember. It has no
filesystem, network, store-write, hardware, DMA, PIO, or IRQ authority.

## Persistence

In amnesic mode nothing is written. In install mode the answers and a
setup-done marker go to `/nonos/setup/` through vfs and are persisted to the
store; the policy capsule restores them at boot and the wizard then exits
without drawing. See `docs/userland/setup-wizard/`.
A Wi-Fi network joined on the network step is remembered only when asked and
only in install mode, sealed under a TPM-derived key in `/nonos/wifi/saved`;
see `docs/subsystems/networking/wifi/joining.md`.
