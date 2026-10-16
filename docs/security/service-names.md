# Service names

Many checks decide who a caller is by the service name it holds:

- the kernel's held drivers, reachable only by the services that drive them
  (`src/services/registry/held.rs`);
- the network stack's administrative calls (`authz.rs` in `capsule_net_dns`,
  `capsule_net_ip`, `capsule_net_l2` and `capsule_net_nym`);
- the file store's installer gate (`capsule_vfs`);
- the policy and wallpaper setters;
- `input_router`'s pointer grabs;
- every app's check that a control message came from `desktop_shell`.

A name is only as good as the rule for who may hold one.

## Who may hold a name

A capsule in the kernel image registers the names its spawn site gives it. At
run time a capsule may register only its own endpoint, or one of five
`net.*` names when it holds the right to register
(`src/syscall/microkernel/ipc/register_allowed.rs`).

A capsule loaded from the file store with `MkCapsuleLoad` declares its names
in its signed manifest. Since K18 the kernel holds those names to a rule
before anything is spawned (`load/names.rs`, `load/name_rule.rs` under
`src/kernel_core/process_spawn/capsule_spawn/from_vfs/`). A reply inbox
`endpoint.<x>.reply` counts as the name `<x>`.

- The kernel's own inboxes (`endpoint.<number>`) and `proc.` names are
  nobody's.
- Every name a spawn site in this image declares, and every name under it,
  belongs to the namespace that spawn site's capsule is signed under
  (`load/baked_names.rs`, `load/baked_apps.rs`). Only the NONOS root signs
  `systems.nonos.*`.
- Every `driver.*` and `net.*` name belongs to `systems.nonos.<that name>`,
  whether or not the image carries it.
- A publisher's capsule, one whose namespace is not `systems.nonos.*`,
  declares only its own namespace, or names under it:
  `com.example.game_2048` or `com.example.game_2048.worker`, and the reply
  `endpoint.com.example.game_2048.reply`.

The rule reads nothing about who is running. A store capsule that declares
`compositor` is refused while the compositor runs, in the second after it
ends and before init restarts it, and after init has given up restarting it.

The reservation lasts as long as the image. It ends for a name only when a
later signed kernel image no longer carries that spawn site.
`scripts/check_baked_names.py` holds the tables to the image's spawn sites
and to each capsule's `Capsule.mk` namespace, and runs a self-test.

A refused load answers `EPERM`, -1. That is apart from `EACCES` for a
signature or proof that fails, and `EEXIST` for a name a live instance
already holds.

## See also

- [Capsule isolation](capsule-isolation.md)
- [Signing and publisher keys](../userland/signing-and-publisher-keys.md)
- [IPC services](../userland/ipc-services.md)
