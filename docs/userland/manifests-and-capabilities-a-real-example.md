# Manifests and capabilities: A real example

Part of [Manifests and capabilities](manifests-and-capabilities.md).

## A real example

This is `userland/capsule_hello/Capsule.mk`, in full:

```make
CAPSULE_SLUG             := hello
CAPSULE_HANDLE           := app.hello
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_hello
CAPSULE_BIN_NAME         := hello
CAPSULE_FEATURE          := nonos-capsule-hello
CAPSULE_NAMESPACE        := systems.nonos.app.hello
CAPSULE_SERVICE_ENDPOINT := service:4810:app.hello
CAPSULE_REPLY_ENDPOINT   := reply:4811:endpoint.app.hello.reply
# CoreExec|IPC|Memory|GraphicsDisplayQuery|GraphicsSurfaceCreate
CAPSULE_REQUIRED_CAPS    := 0x1819
# Debug, granted only by a build that compiles `capsule-serial-debug`: the
# kernel mirror folds it in through serial_debug_cap(), for its [APP] log lines.
CAPSULE_OPTIONAL_CAPS    := 0x100
CAPSULE_KERNEL_MIRROR    := src/userspace/capsule_hello

include nonos-mk/capsule.mk
```

Read it this way:

- The capsule registers the service `app.hello` on port 4810 and owns the [reply
  inbox](../overview/glossary-peer-list.md#reply-inbox)
  `endpoint.app.hello.reply` on port 4811.
- It needs CoreExec, IPC, Memory, GraphicsDisplayQuery and
  GraphicsSurfaceCreate: `0x1 | 0x8 | 0x10 | 0x800 | 0x1000 = 0x1819`.
- Debug (`0x100`) is optional, and no ceiling is set, so the certificate's
  ceiling is `0x1919`.
- The namespace sits under `systems.nonos`, so the [spawn
  gate](../overview/glossary-rollback-index.md#spawn-gate) treats it as an
  enrolled system capsule
  (`src/kernel_core/process_spawn/capsule_spawn/runner/tier.rs:22-28`,
  `classify`).
- Its [kernel mirror](../overview/glossary-hardware-broker.md#kernel-mirror)
  asks for exactly the five required bits plus `serial_debug_cap()`
  (`src/userspace/capsule_hello/spawn.rs:47-52`, `requested_caps`), and
  `serial_debug_cap` returns Debug only in a build with the
  `capsule-serial-debug` feature (`src/capabilities/serial_debug.rs:40-50`).

To declare an app of your own from this file, follow [Shipping an
app](shipping-an-app.md).
