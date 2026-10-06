# Wi-Fi and networking

Put a NONOS machine on a network: join Wi-Fi in Settings, plug in a cable, and read what the status lines and join errors mean.

## Before you start

You need a network card NONOS has a driver for, an image that carries that driver, and a boot that runs the network.

- Wi-Fi: the Realtek RTL8821CE (PCI `10ec:c821`, `PCI_VENDOR_REALTEK` and `PCI_DEVICE_RTL8821CE` in `userland/capsule_driver_rtl8821ce/src/constants/mod.rs:25-26`) and Intel cards through the iwlwifi driver. These two are the only Wi-Fi drivers the panels know (`SERVICES` in `userland/nonos_wifi_client/src/driver/services.rs:32-35`). The Intel driver joins only on the parts whose firmware it boots; see [the Intel Wi-Fi page](../drivers/wifi/iwlwifi.md).
- Wired: `virtio_net` for QEMU, and the `e1000`, `rtl8139` and `rtl8169` driver capsules. Which cards each one takes, by PCI id, is on the [Ethernet](../drivers/ethernet/README.md) pages.
- The standard and hardened images carry all of these drivers: their kernel is built with the `microkernel-full-gui` feature list in `Cargo.toml` (`standard` and `hardened` in `tools/nix/config.nix:70-82`). The `qemu` image carries only `virtio_net`, so it has no Wi-Fi. An image built with the `airgapped` profile carries no network driver, stack or online program at all (`airgapped` in `tools/nix/config.nix:86-92`).
- The Air-Gapped, Safe Mode and Recovery boot modes start no network driver and no network service (`network` in `src/boot/handoff/api/profile.rs:44-46`). See [boot modes](../install/boot-modes.md).

Realtek RTL8821CE Wi-Fi, for scanning, joining, DHCP, DNS and browser traffic: Works on an x86_64 laptop (Intel Gemini Lake, 8 GB), maintainer hardware report, 6 October 2026; the image commit was not recorded.

Everything else on this page is stated from the code. The host tests of the Wi-Fi path pass on this commit: `rtl8821ce_proofs` (171 tests), `nonos_wifi_core_proofs` (70), `wifi_panel_proofs` (23), `iwlwifi_proofs` (205) and `net_core_proofs` (31).

## Join a Wi-Fi network

Open Settings and pick `Wi-Fi` in the sidebar. The page has four cards: `Wi-Fi` (the switch), `Connection`, `Networks` and `Saved networks` (`WIFI` in `userland/capsule_settings/src/settings/schema/blocks/wifi.rs:21-46`).

1. Check that the `Wi-Fi` switch is on. `W` flips it.
2. Press Enter to scan. The `Last join` row reads `Scanning...` until the driver answers. Networks are listed strongest first.
3. Move to a network with the arrow keys and press `C`, or click it.
4. A secured network opens a passphrase line. Type the passphrase, press Tab to show or hide what you typed, Enter to join, or Esc to cancel. Esc wipes what was typed.
5. The row reads `Joining <name>...`, then `Joined` or one of the errors listed below.
6. `D` leaves the network.

The page keeps its own keys (`wifi_key` in `userland/capsule_settings/src/settings/event/wifi_key.rs:42-63`):

| Key | What it does |
|---|---|
| `W` | Wi-Fi on or off. Off leaves the network and stops every scan and join. |
| Enter or Space | Scan. |
| `C` or a click | Join the highlighted network. |
| `D` | Leave the network. |
| `R` | Turn `Remember networks I join` on or off. |
| `F` | Forget the highlighted saved network. |
| Up and Down | Move through the scanned and saved networks. |
| Tab, `[` and `]` | Tab and `]` go to the next section, `[` to the previous one. |
| Esc | Close Settings, or cancel the passphrase. |

There is no Terminal command to scan or join Wi-Fi in this release. The Terminal's network commands are `ping`, `ifconfig`, `nslookup`, `curl` and `nym` (`GROUPS` in `userland/capsule_terminal/src/command/builtin/help_layout.rs:33-41`).

First-boot setup has its own Network step. It starts on `No network (default, private)` (`NO_NETWORK` in `userland/capsule_setup_wizard/src/render/screens/network.rs:10`), so setup joins no Wi-Fi network unless you pick one. A plugged-in cable is used without asking, and setup says so when it sees a wired card (`userland/capsule_setup_wizard/src/render/screens/network_lines.rs`).

## Which networks can be joined

The drivers share one join engine, `nonos_wifi_core`. It runs WPA2-Personal (PSK and PSK-SHA256) and WPA3-Personal (SAE), with CCMP-128 only (`select` in `userland/nonos_wifi_core/src/rsn/select.rs:87-104`). It refuses:

- open networks, with no RSN element (`OpenNetwork` in `userland/nonos_wifi_core/src/mlme/beacon.rs:50-52`);
- TKIP, 802.1X Enterprise, FT and OWE;
- access points that admit only 802.11n stations (`NeedsHt`, same file, lines 58-60).

A network that offers WPA3 and WPA2 side by side is joined with WPA3 first. If that does not finish, for any reason but a wrong password, the RTL8821CE driver joins it again with WPA2 (`retry_with_psk` in `userland/capsule_driver_rtl8821ce/src/serve/connect.rs:156-165`). A network saved after a WPA3 join is saved as WPA3 and is never joined with WPA2 again.

A passphrase is 8 to 63 characters, or 64 hex digits.

Two notes in the interface are older than this code. The `Connection` card in Settings reads `WPA2-Personal or open. WPA3 (SAE) cannot be joined.`, and setup says WPA3 and enterprise networks cannot be joined. The driver code above is what runs: WPA3-Personal joins, open networks do not.

A hidden network cannot be joined in this release. Settings joins only a network its scan heard, and there is no field to type a network name. The drivers can probe by name for a network saved as hidden, but nothing in this release saves a network that way.

## Remember networks

Turn on `Remember networks I join` with `R`. A network you then join is saved, with its passphrase, and joined again at the next boot when it is in range.

- At most four networks are kept; saving a fifth drops the oldest (`SLOTS` in `userland/nonos_wifi_client/src/saved/list.rs:18-19`).
- The list is saved only on a boot that keeps data: the `Keep data across reboots` choice made during setup, which Settings shows under `Privacy`. On any other boot the row reads `Off: this boot keeps nothing`.
- It is saved only with a TPM. The list is sealed with ChaCha20-Poly1305 under a [machine key](../overview/glossary.md#machine-key) the TPM derives for the label `wifi/saved-networks` (`LABEL` in `userland/nonos_wifi_client/src/saved/key.rs:18`). That key is bound to PCRs 0, 4, 7 and 9 (`BOUND_PCRS` in `src/security/tpm/machine_key/pcrs.rs:24`), so after a firmware or kernel change the saved list no longer opens and you join again by hand.
- The sealed record is the file `/nonos/wifi/saved` in the NONOS store (`PATH` in `userland/nonos_wifi_client/src/saved/file.rs:20-23`).

At boot, `net.core` scans and joins the first saved network in range. Each saved network is tried at most once per boot, so a wrong passphrase is not sent to the access point again and again. After eight scans with none in range (`EMPTY_PASSES_MAX` in `userland/capsule_net_core/src/autojoin/machine.rs:41`) it stops and logs `no saved Wi-Fi network in range; join one from Settings`.
