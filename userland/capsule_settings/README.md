# capsule_settings

## Role

`capsule_settings` is the desktop's Settings window (`app.settings`), the
editor for the `policy` store. It is an app on `nonos_app_skeleton`, 760 by
520, with nine sections: General, Network, Wifi, Security, Appearance,
Privacy, Sound, Updates and Developer (`src/settings/section.rs`).
The handbook page is
[System apps and services](../../docs/handbook/apps/system-apps.md).

```text
settings app
    |
    | get / set by Field id
    v
policy (service:4108)  -- kernel-mirrored fields --> MkAdminPolicyPush
    |
    `-- desktop_shell: "settings applied" notice
```

## Microkernel contract

- The window, input and frame loop come from `nonos_app_skeleton::run`.
- `MkIpcCall` to `policy` reads every field on open and sets one on each
  edit; on success `notify_applied` sends `desktop_shell` a notice. The
  Qwen model row is a choice among the pinned tiers (setup's `qwen/labels.rs`,
  held to the pins by setup's build): Left, Right and Enter step through them
  (`src/settings/qwen_tier/`, held by `capsule_settings_proofs`).
- `MkDeviceList` finds the Wi-Fi adapters the device broker knows.
- `MkIpcCall` to `net.dhcp.client` reads the lease; the Wi-Fi client
  (`nonos_wifi_client`) scans, joins and remembers networks.
- `CryptoMachineKey` derives the TPM key that seals the remembered networks,
  kept through vfs at `/nonos/wifi/saved`.

## Interface contract

It serves no IPC of its own; its service and the instance endpoints
`app.settings.1` and `app.settings.2` exist so the shell can focus it and
open more windows. The `policy` service accepts writes only from these
three names and the setup wizard.

## Authority

`CAPSULE_REQUIRED_CAPS = 0x987d`:

| Bit | Capability | Purpose |
|---|---|---|
| 0x0001 | CoreExec | run user code |
| 0x0004 | Network | ask `net.dhcp.client` for the Wi-Fi lease |
| 0x0008 | IPC | policy, desktop shell, Wi-Fi client, window services |
| 0x0010 | Memory | heap |
| 0x0020 | Crypto | the TPM machine key that seals saved Wi-Fi networks |
| 0x0040 | FileSystem | read and keep saved networks through vfs, which serves only a holder of it |
| 0x0800, 0x1000 | GraphicsDisplayQuery, GraphicsSurfaceCreate | its window |
| 0x8000 | DeviceEnum | list the Wi-Fi adapters |

No Admin: kernel-mirrored fields reach the kernel through the policy
capsule, which holds it.

## Privacy and persistence

Settings holds no state of its own beyond the window. The values live in
the policy store, which is RAM only: a change made here is gone at reboot,
except the answers first-boot setup kept, which the policy capsule restores
on a persistent install. Saved Wi-Fi networks are the exception: they are
sealed under a TPM-derived key and kept through vfs.

## Runtime lifecycle

On open, `hydrate` reads every field it knows from `policy`. `hydrate_fields`
stops at the first read that timed out, so a silent policy service costs one
timeout rather than one per field, and the status strip says the values shown
are not the stored ones (`src/settings/ipc/hydrate_pass.rs`). Each edit sends
the matching set and, on success, the shell notice.

## Failure model

- Policy unreachable: values shown are defaults, and the status strip says
  so.
- A set the policy service refuses leaves the old value.
- No adapter, no lease or no TPM: the Wi-Fi section says which.
- A scan or join that will not run says why on the Wi-Fi page: the switch
  is off, no driver is running, or the highlighted row is not one the scan
  found (`src/settings/state/wifi_refusal.rs`).
- Kept wallpapers that were not read show `--`, and a change to them is
  refused rather than written over the stored set.

## Operating rules

- A compile-time check fails the build if a field is listed but placed on no
  screen (`src/settings/schema/coverage.rs`, `all_placed`).
- `Persistent` is shown and never edited (`src/settings/schema/read_only.rs`);
  persistence is granted with consent in the setup wizard. The policy store
  itself has no read-only notion.
- Security shows no stored "keys generated" flag, which nothing set. Each
  time the page opens it asks the kernel for the TPM machine key under a
  label of its own, says what came back, and wipes the 32 bytes
  (`src/settings/state/machine_key_probe.rs`).
- Updates reports `rustc -V` and cargo's target as `build.rs` read them.
- Wi-Fi keys (`src/settings/event/wifi_key.rs`): Enter or Space scans, C
  joins, D leaves, R remembers joins, F forgets, W turns the switch on or
  off. Off also leaves the joined network.

## Verification

- Build: `make nonos-mk-settings`; sign: `make nonos-mk-settings-sign`.
- Kernel mirror: `src/userspace/capsule_settings`, under the feature
  `nonos-capsule-settings`.
- `userland/wifi_panel_proofs` runs the Wi-Fi panel and the saved-network
  list on the host.
- `userland/capsule_settings_proofs` runs the section tables, the machine
  key row (`security_tests`), the Qwen tier choice (`qwen_tier_tests`) and
  the Wi-Fi keys and refusals (`wifi_key_tests`).
