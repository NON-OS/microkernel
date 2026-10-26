## Bring-up stages

The status operation returns a stage byte, and the panel shows its text (`userland/nonos_wifi_client/src/driver/stage.rs:49-63`, `DriverStage`).

| Stage | Code | Panel text |
|---|---|---|
| `Ready` | 0 | `Ready` |
| `NotClaimed` | 1 | `The card could not be claimed` |
| `PowerFailed` | 2 | `The card did not power on` |
| `DeadMmio` | 3 | `The card's registers read back dead` |
| `FirmwareFailed` | 4 | `The card's firmware did not load` |
| `NoDma` | 5 | `No DMA memory for the radio` |
| `EfuseFailed` | 6 | `The card's calibration did not read` |
| `NoStationAddress` | 7 | `No random address could be drawn` |
| `NoAirPath` | 8 | `card not supported yet; use Ethernet or USB Wi-Fi` |
| any other | | `The driver reported an unknown stage` |

The RTL8821CE also serves `DeadMmio` once its card reads back all ones while serving; its radio is down from then on.

Neither suggestion in the `NoAirPath` line helps on real hardware in 0.9.2: there is no driver for a USB Wi-Fi adapter, and read from the code no wired driver in the image receives frames from a real card ([the receive fault](../../ethernet/README.md#the-receive-fault)). See [what to use instead](../not-supported.md#what-to-use-instead).

When no Wi-Fi driver answers at all, the panel names the chip by its PCI ids and says whether NONOS has a driver for it (`userland/capsule_settings/src/settings/ui/live_wifi.rs:115-130`, `no_driver`). The lines it can show are listed under [how to tell](../not-supported.md#how-to-tell).

## Saved networks

- The list holds at most 4 networks, each passphrase at most 64 bytes (`userland/nonos_wifi_client/src/saved/list.rs:19-21`, `SLOTS`, `PASS_MAX`). Saving a fifth drops the oldest network not pinned to WPA3, or the oldest of all when all four are pinned, keeps nothing of it, and returns its name for the panel to show on screen only (`userland/nonos_wifi_client/src/saved/list_put.rs`, `put_dropping`).
- It is one file, `/nonos/wifi/saved`, sealed with ChaCha20-Poly1305 (`userland/nonos_wifi_client/src/saved/file.rs:6-21`, `PATH`).
- The key is the TPM's [machine key](../../../overview/glossary-linux-personality.md#machine-key) for the label `wifi/saved-networks`, asked for with `machine_key` and wiped after each use (`userland/nonos_wifi_client/src/saved/key.rs:18-29`, `with_key`).
- With no TPM, or after the boot state changed (a firmware, Secure Boot, loader or kernel change), the list cannot be opened (`userland/nonos_wifi_client/src/saved/key.rs:22-26`, `NoTpm`, `BootChanged`).
- A network is written only on a boot that keeps state (`userland/nonos_wifi_client/src/saved/write.rs:21-24`, `keeps_state`).
- A network joined with SAE is saved as WPA3, so no later join accepts WPA2 for it (`userland/nonos_wifi_client/src/saved/store.rs:38-41`, `remember`).

## Autojoin and losing the link

At boot `net.core` tries the saved networks it hears. A round tries each once. One refused for a reason anyone on the air could forge (a handshake that did not finish, which is also how a deauthentication looks, or a beacon's security) comes round again, at most 8 rounds, each after twice the wait before, from 4 s up to 60 s; only a passphrase of the wrong length, a message 3 that did not match the beacon, or the person leaving ends it (`userland/capsule_net_core/src/autojoin/machine/codes.rs`, `refused`; `machine/after.rs`, `after_attempt`). With none in range it scans again after 15 s and gives up after 8 empty passes (`userland/capsule_net_core/src/autojoin/machine.rs:36-41`, `RESCAN_MS`, `EMPTY_PASSES_MAX`). A join handed to a driver is watched for 25 s (`userland/capsule_net_core/src/autojoin/machine.rs:42-45`, `JOIN_WATCH_MS`).

After one join succeeds, autojoin is done for that boot (`userland/capsule_net_core/src/autojoin/machine.rs:154-158`, `after_watch`). An association the access point ends is not joined again by itself: join it again from Settings. When the bound link goes down and comes back, `net.core` asks DHCP for the lease again (`userland/capsule_net_core/src/iface/relink.rs:40-47`, `Change::Returned`).
