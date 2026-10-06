# Settings

Every panel of the Settings window, what each row changes, and how long a change lasts.

## How Settings works

Settings (`app.settings`) is an editor for the [policy store](../overview/glossary.md#policy-store), the service that holds the system's choices (`userland/capsule_settings/README.md`). When the window opens it reads every value it shows. Each change you make is sent to the policy store at once, and on success the desktop shows the notice `settings applied` (`userland/capsule_settings/src/settings/ipc/notify_shell.rs`). If the policy store does not answer, the values shown are defaults, the status strip says `Policy service unavailable - values shown are not stored`, and Settings asks again on its own (`userland/capsule_settings/src/settings/ui/status_bar.rs`).

A row only exists when some code acts on its value. `ALL_FIELDS` lists the fields Settings shows, each with the code that reads it, and the build fails if a row names a field outside that list, or if a listed field has no row (`userland/capsule_settings/src/settings/schema/coverage.rs`, `userland/capsule_settings/src/settings/schema/coverage_listed.rs`).

## Moving around

| Key | What it does |
|---|---|
| `Up`, `Down`, `Home`, `End`, `PgUp`, `PgDn` | Move between rows. |
| `Left`, `Right` | Step a value down or up. |
| `Space`, `Enter` | Switch a toggle, or step a choice. |
| `Tab`, `]` | Next panel. |
| `[` | Previous panel. |
| `Ctrl+V`, `Shift+Insert` | Paste into the field being edited. |
| `Esc` | Close the window. |

Code: `on_event_browsing` in `userland/capsule_settings/src/settings/event/on_event_browsing.rs`. Click the search field to find a row by name. The Wi-Fi panel has keys of its own, listed below.

## The nine panels

The panels, in sidebar order (`SECTIONS` in `userland/capsule_settings/src/settings/section.rs`):

| Panel | What it covers |
|---|---|
| General | This machine's name, the clock and notifications. |
| Network | How NONOS connects to networks and the internet. |
| Wi-Fi | Find and join a wireless network. |
| Security | This machine's keys and the protections the kernel keeps on. |
| Appearance | The wallpaper and how the pointer moves. |
| Privacy | What this machine keeps once it is switched off. |
| Sound | System tones and how loud they are. |
| Updates | The image this machine is running. |
| Developer | How the scheduler shares the processor. |

The rows of each panel come from `userland/capsule_settings/src/settings/schema/blocks/`, and the labels from `userland/policy_proto/src/field_label.rs`.

### General

| Row | What it changes |
|---|---|
| `Your name` | The name the Terminal shows in its prompt and in `whoami`. Never sent on a network. |
| `Hostname` | The computer's name, shown in the Terminal. Never sent on a network. |
| `Qwen model` | The model tier `qwen` runs when no tier is named. `Left` and `Right` choose. See [Local model](local-ai.md). |
| `Timezone (hours from UTC)` | The offset the menu bar clock uses. |
| `24-hour clock` | The menu bar clock's format. |
| `Notifications` | Notices from apps. Warnings and errors always show. |

### Network

| Row | What it changes or shows |
|---|---|
| `Default network` | The route the browser starts on, and the one Music downloads and the Terminal's `curl` and `git` take: `Nym mixnet`, `Anyone network` or `Direct` (`ROUTE_LABELS` in `userland/policy_proto/src/route.rs`). A route never falls back to another. |
| `Connection`, `IP address`, `Gateway`, `DNS` | What the DHCP client holds, read live. |
| `Wireless adapter` | The name of the first wireless controller on the PCI bus, or `None detected`. It names the adapter only; it does not say NONOS drives it (`userland/capsule_settings/src/wifi/interface.rs`). |

The note under `Default network` says Qwen downloads take it. They do not in this release: model downloads take the Anyone network whatever this row says, unless you ask for one download to go direct, and the wallet turns a `Direct` choice into Nym or Anyone (`for_installs` and `for_wallet` in `userland/nonos_route_link/src/chosen.rs`). See [Privacy networks](privacy-network.md) for what each route hides and costs.

### Wi-Fi

| Row | What it changes or shows |
|---|---|
| `Wi-Fi` | The radio. Off leaves the network and stops every scan and join. |
| `Status`, `Last join` | The link, and how the last join went. |
| Networks | The networks found by the last scan. |
| `Remember networks I join` | Whether a network you join is saved. It is saved only on a machine where `Keep data across reboots` is on (`put` in `userland/nonos_wifi_client/src/saved/write.rs`). |
| Saved networks | The saved networks, sealed with the TPM machine key. |

Only WPA2-Personal and open networks can be joined. WPA3 (SAE) cannot. The panel's keys (`userland/capsule_settings/src/settings/event/wifi_key.rs`): `Enter` or `Space` scans, `C` or a click joins the highlighted network, `D` leaves, `R` switches remembering, `F` forgets the highlighted saved network, and `W` switches the radio. While you type a passphrase, `Tab` shows or hides it, so a symbol the keyboard layout moved can be checked before joining. See [Wi-Fi and networking](wifi-and-networking.md).

### Security

| Row | What it shows |
|---|---|
| `Machine key` | Asked of the TPM each time the panel opens. Saved Wi-Fi passphrases are sealed with it. |

The panel also notes that SMEP, SMAP, UMIP, NX and WP are set at boot when the CPU has them. Nothing here can switch them off. See [Protections and limits](../security/protections-and-limits.md).

### Appearance

| Row | What it changes |
|---|---|
| `Wallpaper` | The desktop's wallpaper. |
| `Wallpapers kept` and the list under it | Which wallpapers are read from the disk at all. The desktop's own wallpaper is always kept. |
| `Pointer speed` | How far the pointer moves. It scales mouse movement only. |

### Privacy

| Row | What it shows |
|---|---|
| `Keep data across reboots` | Whether this machine keeps data. Read only: it is chosen once, during setup, and without it nothing is written to disk (`userland/capsule_settings/src/settings/schema/read_only.rs`). |

The note under the row says `Files and installed apps are kept between boots`. Installed apps are, but files you make in Files or Editor are not. What is kept when the field is on, and what is not even then, is on [Files](files.md#what-is-kept-after-power-off).

### Sound

| Row | What it changes or shows |
|---|---|
| `Output device` | What the sound hardware reported when the panel opened, for example `Speakers and headphone jack` or `Headphones`, or the reason nothing plays. |
| `System sound` | The desktop's own tones, on or off. |
| `Volume` | How loud the desktop's tones are. |
| `Alert sounds` | A tone for warnings and errors. |

`Volume` here is the tones' level only. The master volume, which every sound passes through, is set with the volume keys or the slider in Music. The notes under `System sound` and `Volume` still say Music has its own volume; in this release Music's slider is the master volume (`userland/capsule_audio_player/src/audio_client/master.rs`). See [Sound and media](audio.md).

### Updates

| Row | What it shows |
|---|---|
| `Version`, `Commit`, `Toolchain`, `Architecture` | What was recorded when this image was built. |

The panel shows no "signed" badge: Settings cannot check the image's signature itself, so it does not claim one (`userland/capsule_settings/src/settings/schema/blocks/updates.rs`). To install a newer image, see [Update](../install/update.md).

### Developer

| Row | What it changes |
|---|---|
| `Preemptive scheduling` | Whether the timer ends a program's turn, so none can hold the processor. The policy store passes the change to the kernel (`on_bool_set` in `userland/capsule_policy/src/push/on_bool_set.rs`). |

## What Settings does not have

- No keyboard layout row. The layout is chosen at first boot and cycled with `Ctrl+Alt+Space`; see [Keyboard layouts](keyboard-layouts.md). The policy store has the field, but Settings does not list it (`ALL_FIELDS` in `userland/capsule_settings/src/settings/schema/all_fields.rs`).
- No switch to turn back on an app turned off at setup. That field is not in `ALL_FIELDS` either.
- No shutdown or restart. The desktop has no way to power off in this release.

## How long a change lasts

The policy store keeps its values in memory.

- On an amnesic boot, the default, every change is gone at power off.
- On a machine where `Keep data across reboots` is on, the policy store writes the values it keeps to `/nonos/settings/values` about a second after your last change, and puts them back at the next boot after setup's answers (`QUIET_MS` in `userland/capsule_policy/src/keep/tick.rs`). The kept fields are listed in `KEPT` in `userland/policy_proto/src/settings_record.rs`.

## See also

- [The desktop](desktop.md)
- [Files](files.md)
- [Keyboard layouts](keyboard-layouts.md)
- [Sound and media](audio.md)
- [Wi-Fi and networking](wifi-and-networking.md)
- [Privacy networks](privacy-network.md)
- [First boot](../install/first-boot.md)
