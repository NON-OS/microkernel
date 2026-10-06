# Manifests and capabilities

A [capsule](../overview/glossary.md#capsule) states what it is and what it may do in its `Capsule.mk`; the build turns that into a signed binary [manifest](../overview/glossary.md#manifest), and the kernel fixes the capsule's [capability word](../overview/glossary.md#capability-word) from it at spawn.

## Where a manifest comes from

Nobody writes a manifest by hand. Each capsule directory holds a `Capsule.mk` that sets `CAPSULE_*` variables and includes `nonos-mk/capsule.mk`, and the signing step encodes those values. The template refuses to load when a required variable is missing (`nonos-mk/capsule.mk:28-54`, `CAPSULE_NAMESPACE`).

| Variable | Required | Meaning | Default |
|---|---|---|---|
| `CAPSULE_SLUG` | yes | the name make targets use | |
| `CAPSULE_BIN_NAME` | yes | the ELF name, and the prefix of every artifact and key file | |
| `CAPSULE_DIR` | yes | the crate directory | |
| `CAPSULE_HANDLE`, `CAPSULE_DOMAIN` | yes | hashed into the [publisher](../overview/glossary.md#publisher)'s NONOS ID | |
| `CAPSULE_RECOVERY` | no | a recovery string hashed into the NONOS ID with them | empty |
| `CAPSULE_NAMESPACE` | yes | reverse-domain name; the certificate is issued for it | |
| `CAPSULE_SERVICE_ENDPOINT` | yes | `service:<port>:<name>` | |
| `CAPSULE_REPLY_ENDPOINT` | yes | `reply:<port>:<inbox name>` | |
| `CAPSULE_REQUIRED_CAPS` | yes | capabilities the capsule cannot run without | |
| `CAPSULE_OPTIONAL_CAPS` | no | capabilities a spawn site may add | `0x0` |
| `CAPSULE_CAPS_CEILING` | no | the certificate's ceiling | required and optional bits together |
| `CAPSULE_INSTANCE_ENDPOINTS` | no | extra [endpoint](../overview/glossary.md#endpoint) pairs for on-demand instances, such as a second Terminal window | none |
| `CAPSULE_TARGET` | no | target triple | `x86_64-nonos-user` |
| `CAPSULE_VERSION` | no | `major.minor.patch` | `0.1.0` |
| `CAPSULE_BUILD_STD` | no | the `-Zbuild-std` crates | `core,alloc` |
| `CAPSULE_PREBUILT_BIN` | no | copy this ELF instead of running cargo | none |
| `CAPSULE_DEV_ONLY` | no | sign and enroll only in a development image | unset |
| `CAPSULE_KERNEL_MIRROR` | no | the kernel directory that embeds and spawns it | none |
| `CAPSULE_FEATURE` | no | the kernel feature that embeds it; an image ships the capsules whose feature its profile turns on | `nonos-capsule-<slug>` |

The defaults are set in one place: the optional capabilities at `nonos-mk/capsule.mk:69` (`CAPSULE_OPTIONAL_CAPS`), the ceiling at `nonos-mk/capsule.mk:74` (`CAPSULE_CAPS_CEILING`), the target, version and serial at `nonos-mk/capsule.mk:75-77` (`CAPSULE_TARGET`), and the feature at `nonos-mk/capsule.mk:88` (`CAPSULE_FEATURE`). The image build picks capsules by that feature (`tools/nix/image.nix:54-57`, `shipped`). A few rarer inputs, such as `CAPSULE_CARGO_FEATURES`, `CAPSULE_ID_CERT_SERIAL` and `CAPSULE_KEY_PUB_PREFIX`, are listed in the reset block at the end of the template (`nonos-mk/capsule.mk:325-352`, `CAPSULE_SLUG`).

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

- The capsule registers the service `app.hello` on port 4810 and owns the reply inbox `endpoint.app.hello.reply` on port 4811.
- It needs CoreExec, IPC, Memory, GraphicsDisplayQuery and GraphicsSurfaceCreate: `0x1 | 0x8 | 0x10 | 0x800 | 0x1000 = 0x1819`.
- Debug (`0x100`) is optional, and no ceiling is set, so the certificate's ceiling is `0x1919`.
- The namespace sits under `systems.nonos`, so the [spawn gate](../overview/glossary.md#spawn-gate) treats it as an enrolled system capsule (`src/kernel_core/process_spawn/capsule_spawn/runner/tier.rs:22-28`, `classify`).
- Its [kernel mirror](../overview/glossary.md#kernel-mirror) asks for exactly the five required bits plus `serial_debug_cap()` (`src/userspace/capsule_hello/spawn.rs:47-52`, `requested_caps`), and `serial_debug_cap` returns Debug only in a build with the `capsule-serial-debug` feature (`src/capabilities/serial_debug.rs:40-50`).
