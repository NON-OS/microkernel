# First boot

The thirteen steps of first-boot setup, in order: what each answer changes, and where it is kept.

## What starts before setup

After the boot menu, the loader checks the kernel, shows its proofs panel for a moment and hands over (`nonos-bootloader/src/display/boot/proofs/show.rs`). The kernel starts the compositor and the input router, and setup runs on them before any desktop app (`src/userspace/init/spawn_plan/wizard_plan.rs`).

Setup does not run:

- on a Recovery boot, which goes straight to the desktop with a Terminal open;
- when an earlier boot kept its answers: setup restores them and exits without drawing, and the kernel log says `[SETUP] kept from an earlier boot; starting the desktop`, or `opening the installer` on a boot from the menu's `Install NØNOS` entry (`userland/capsule_setup_wizard/src/main.rs`);
- on an image built with `install = false` in `nonos.toml`, which has no setup and no installer (`installFeatures` in `tools/nix/config.nix`).

There is no login and no password. The `login` service starts with nothing on screen, and no program in this release asks it to start or end a session (`userland/capsule_login/src/setup/run.rs`).

## Keys in setup

Enter goes to the next step and Escape to the one before. In a list, Up and Down or `k` and `j` move, Home and End jump to the ends, and a digit picks that row (`userland/capsule_setup_wizard/src/server/step.rs`). Ctrl+Alt+Space cycles the keyboard layout at any time, in the PS/2 and the USB keyboard drivers alike (`userland/capsule_driver_ps2_input/src/poll/absorb.rs`, `userland/capsule_driver_usb_hid/src/hid/keyboard/push_key.rs`).

## The steps

The step names are `STEP_LABELS` in `userland/capsule_setup_wizard/src/render/theme.rs`, and each screen is a file under `userland/capsule_setup_wizard/src/render/screens/`.

1. Keyboard. Six layouts: US QWERTY, UK, German, French AZERTY, Italian and Spanish, the ones the keyboard drivers have tables for (`POLICY_LAYOUTS` in `userland/nonos_keymap/src/policy.rs`). The choice takes effect when you press Enter. See [Keyboard layouts](../using/keyboard-layouts.md).
2. Your name. The account name the Terminal shows as name@host: lowercase letters, digits, `-` and `_`, starting with a letter, 1 to 32 characters. Left empty, it is `nonos`. A refused key is named on the screen.
3. Time zone. Whole hours from UTC, from UTC-12 to UTC+14, for the menu bar clock. `j` adds an hour and `k` takes one off.
4. Mode. Where this machine keeps what you do:
   - `Amnesic (default)`: RAM only. Nothing is written to any disk, and setup runs again on the next boot.
   - `USB live (unavailable)`: cannot be chosen. Its screen says why: NONOS keeps state only on an NVMe, SATA or virtio disk today, and has no passphrase-keyed volume.
   - `Install to this computer`: keeps your answers in the package store of the disk this boot came from, so setup does not run again, then opens the installer when setup ends.

   After the boot menu's `Install NØNOS` entry, this step starts on Install (`userland/capsule_setup_wizard/src/render/screens/mode.rs`).
5. Network. The first row is `No network (default, private)`. Under it come up to six Wi-Fi networks the card has heard, strongest first. The list refreshes every 2 seconds, and `s` looks again (`userland/capsule_setup_wizard/src/network/poll.rs`).
   - An open network is joined at once on Enter.
   - A secured, WPA2-Personal network asks for its passphrase: 8 to 63 characters, shown as stars. Tab shows it, Escape clears it, Enter joins. A join takes up to 30 seconds. A failed join says why and wipes what you typed.
   - With the Install mode chosen, `r` remembers the joined network, sealed with a key the TPM derives.
   - The step's own note reads `WPA3 (SAE) and enterprise networks cannot be joined.`
   - A wired card is used as soon as a cable is plugged in, without asking.

   On Safe Mode and Air-Gapped boots the step says `This boot runs no network: the boot menu chose it.` See [Wi-Fi and networking](../using/wifi-and-networking.md).
6. Network route. Which network this machine's own traffic leaves through: the browser starts on it and Qwen downloads take it.
   - `Nym mixnet (default)`: hides who you talk to, even from someone watching the whole internet. Pages and downloads are slow.
   - `Anyone network`: onion routing through three relays, much faster than the mixnet. Someone watching both ends at once could match the traffic.
   - `Direct`: no anonymity network. Every site, the model mirror and your own network see this machine's address.

   A route that fails never falls back to another, and Settings changes the choice later. See [Privacy networks](../using/privacy-network.md).
7. Privacy. Statements with no switch (`userland/capsule_setup_wizard/src/render/screens/privacy.rs`): the e1000, RTL8169 and RTL8821CE drivers send from a random MAC address drawn at each start (`userland/capsule_driver_e1000/src/init/station_address.rs`, `userland/capsule_driver_rtl8169/src/init/mac.rs`, `userland/capsule_driver_rtl8821ce/src/station.rs`); shutdown and reboot wipe process memory, kernel stacks and held keys; NONOS has no telemetry.
8. Appearance. Which wallpapers this machine keeps, and which one is the desktop's. Space keeps or drops the highlighted one, `a` keeps them all, `n` keeps only the desktop's, and Enter makes the highlighted one the desktop's and goes on.
9. Qwen model. Which Qwen tier the Terminal's `qwen` runs when you name none. The list starts with `None for now`, then the tiers that fit this machine's memory, smallest first. The step starts on Qwen3 0.6B, the tier the stick carries, when it fits, and otherwise on the largest tier that fits (`userland/capsule_setup_wizard/src/qwen/default.rs`, `userland/capsule_model_fetch/src/default_tier.rs`). On an amnesic stick the model is held in memory and is gone at power off. With no NONOS disk at all, no tier is offered. See [Local AI](../using/local-ai.md).
10. Apps. Each optional app is on until you turn it off with Space. An app turned off is not started, and its dock icon does not open it. With Linux and Qwen off, `qwen` and Linux packages do not run. Required apps are listed without a switch. On a Safe Mode boot no optional app starts, whatever this step says.
11. Installed software. `Only NONOS software`, the default, or `Also software installed here`, which lets this machine run the programs the Marketplace installs and this machine proves.
12. Computer name. The host in name@host: lowercase letters, digits and `-`, starting with a letter and ending with a letter or digit, up to 63 characters. Left empty, it is `nonos`.
13. Review. Your answers in a table, and lines that say what is kept. Enter applies them and starts the desktop, or opens the installer on Install. If the settings service refused an answer, the screen names it once, after `Not applied to this session:`, and the next Enter goes on without it (`userland/capsule_setup_wizard/src/render/screens/review.rs`).

## What is kept, and where

Every answer is handed to the settings service, which applies it to this session in either mode (`userland/capsule_setup_wizard/src/render/screens/commit.rs`). What reaches a disk depends on the Mode step:

| What | Path | When it is written |
|---|---|---|
| The answers: keyboard, time zone, wallpaper and the wallpapers kept, name, Qwen tier, apps turned off, computer name, network route | `/nonos/setup/answers` | Install: written to the store. Amnesic: held in memory for the installer, gone at shutdown |
| The marker that setup is done | `/nonos/setup/done` | Install only, after the answers |
| Consent to run installed software | `/nonos/consent/local.token` | Install, with `Also software installed here` chosen |
| A remembered Wi-Fi network | `/nonos/wifi/saved` | Install, with `r` checked and a TPM present |

The paths are in `userland/policy_proto/src/setup_record/layout.rs`, `userland/capsule_setup_wizard/src/consent/restore.rs` and `userland/nonos_wifi_client/src/saved/file.rs`; the order of writing is in `userland/capsule_setup_wizard/src/keep/save.rs`.

- The answers go first and the marker last, so a save cut short restores nothing and setup simply runs again.
- The answers and the marker are not encrypted. The [store](../overview/glossary.md#store) is written to the disk as it is (`src/syscall/microkernel/store_write.rs`).
- The Wi-Fi passphrase is never written in the clear. It is shown as stars, never sent to the console, and wiped from memory on Escape, after a failed join, and once review has used it.
- Until you install, the store that keeps your answers is the stick's. The installer carries the answers to the disk it writes, so setup does not run there either.
- To run setup again on a stick that kept answers, write the image to it again: a freshly written store holds no answers.

## When setup cannot start

Setup first waits up to 30 seconds for this boot's store to load (`userland/capsule_setup_wizard/src/keep/skip.rs`). If setup cannot draw at all, it writes `[SETUP] not started:` and the reason to the kernel log, and the desktop starts without it (`userland/capsule_setup_wizard/src/main.rs`).

## After setup

```mermaid
flowchart TD
  review[Review] --> mode{Mode step}
  mode -->|Amnesic| desk[desktop]
  mode -->|Install| kept[answers kept]
  kept --> inst[installer]
  inst --> desk
```

Amnesic starts the desktop. Install keeps the answers, then hands the whole screen to the installer with no desktop behind it. If you leave the installer without installing, the desktop starts (`src/userspace/init/supervisor/after_setup.rs`, `src/userspace/init/supervisor/after_install.rs`).
