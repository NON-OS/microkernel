# capsule_policy

## Role

`capsule_policy` is the system policy store. It runs as a CPL=3 capsule and
holds the small set of system-wide policy values (keyed records) that other
capsules read at startup and update at runtime: a typed key-value service
with a bootstrap set of defaults. It owns the policy table and is the single
source of truth for the values it serves. The fields the kernel acts on
(`KernelPreempt`, `Timezone`, `Hostname`, `DomainName`) are pushed into the
kernel with `MkAdminPolicyPush` when they are set. The handbook page is
[System apps and services](../../docs/handbook/apps/system-apps.md).

```text
desktop services / shell
        |
        | OP_GET (key) / OP_SET (key, value)
        v
capsule_policy -- in-memory policy table (bootstrap defaults)
```

## Microkernel contract

```text
CAPSULE_REQUIRED_CAPS = 0x259
```

CoreExec, IPC, Memory, FileSystem and Admin. The capsule serves callers
with `MkIpcRecvFrom` plus `MkIpcReply`, looks the setters up with
`MkServiceLookup`, pushes kernel fields with `MkAdminPolicyPush`, and
terminates only through `MkExit`. It requests no hardware grants. The
kernel mirror is `src/userspace/capsule_policy`.

## Interface contract

| Operation | Input | Output |
|---|---|---|
| `OP_GET` | field id | current value |
| `OP_SET` | field id, value | accepted or rejected |

Each field is a `Field` with a u32 id in `userland/policy_proto`; values
are bool, u8, i8 or a short string. Anyone may read. Only `app.settings`,
its instance windows `app.settings.1` and `app.settings.2`, and
`app.setup_wizard` may write: the pid behind each name is looked up on
every set, and any other sender gets `E_ACCES`.

Every frame goes to `serve` (`src/server/serve.rs`), which answers each one
exactly once: a frame too short to hold a header gets `E_BAD_LEN`, and an
unknown op (including `OP_STATUS`, which `policy_proto` defines and this
capsule does not serve), an unknown field or a short body get `E_INVAL`.

## Authority

The capsule serves policy over IPC. It holds `FileSystem` to restore kept
settings through vfs (`src/restore/tick.rs`), since vfs serves only a holder of FileSystem, and
`Admin` to push kernel-mirrored fields. It has no PCI, MMIO, IRQ, DMA,
PIO, network, display, or focus-routing authority.

## Privacy and persistence

The policy table lives in capsule memory and is seeded from the bootstrap
defaults on every boot. Values set at runtime do not persist across a
reboot, with one exception: when first-boot setup chose a mode that keeps
state, it kept its answers in the vfs store, and `src/restore/` reads them
back through vfs once the store has loaded and sets `Persistent`, the apps
switched off, the network route, the keyboard layout, the wallpaper, the
user name, the model tier, the time zone and the hostname from them
(`src/restore/apply.rs`).

## Verification

- Build: `make nonos-mk-policy`; sign: `make nonos-mk-policy-sign`.
- Host proofs: `userland/policy_proofs` sends `serve` any frame, from any
  sender, and checks each is answered exactly once with a whole header.
