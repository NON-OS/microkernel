# Settings

Every panel of the Settings window, what each row changes, and how long a change lasts.

## How Settings works

Settings (`app.settings`) is an editor for the [policy store](../overview/glossary.md#policy-store), the service that holds the system's choices. When the window opens it reads every value it shows. Each change you make is sent to the policy store at once, and on success the desktop shows the notice `settings applied`. If the policy store does not answer, the values shown are defaults, the status strip says `Policy service unavailable - values shown are not stored`, and Settings asks again on its own.

A row only exists when some code acts on its value. Settings keeps a list of the fields it shows, each with the code that reads it, and the build fails if a row names a field outside that list, or if a listed field has no row.

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

Click the search field to find a row by name. The Wi-Fi panel has keys of its own, listed below.

## The nine panels

The panels, in sidebar order:

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
| `Default network` | The route the [Browser](browser.md) starts on, and the one Music downloads and the Terminal's `curl` and `git` take: `Nym mixnet`, `Anyone network` or `Direct`. A route never falls back to another. |
| `Connection`, `IP address`, `Gateway`, `DNS` | What the DHCP client holds, read live. |
| `Wireless adapter` | The name of the first wireless controller on the PCI bus, or `None detected`. It names the adapter only; it does not say NONOS drives it. |

The note under `Default network` says Qwen downloads take it. They do not in this release: model downloads take the Anyone network whatever this row says, unless you ask for one download to go direct, and the wallet turns a `Direct` choice into Nym or Anyone. See [Privacy networks](privacy-network.md) for what each route hides and costs.

### Wi-Fi

| Row | What it changes or shows |
|---|---|
| `Wi-Fi` | The radio. Off leaves the network and stops every scan and join. |
| `Status`, `Last join` | The link, and how the last join went. |
| Networks | The networks found by the last scan. |
| `Remember networks I join` | Whether a network you join is saved. It is saved only on a machine where `Keep data across reboots` is on. |
| Saved networks | The saved networks, sealed with the TPM machine key. |

WPA2-Personal and WPA3-Personal (SAE) networks can be joined. Open networks cannot: the join engine refuses a network with no RSN element. The panel's note `WPA2-Personal or open. WPA3 (SAE) cannot be joined.` is older than this code and says the reverse. The panel's keys: `Enter` or `Space` scans, `C` or a click joins the highlighted network, `D` leaves, `R` switches remembering, `F` forgets the highlighted saved network, and `W` switches the radio. While you type a passphrase, `Tab` shows or hides it, so a symbol the keyboard layout moved can be checked before joining. See [Wi-Fi and networking](wifi-and-networking.md).

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
| `Keep data across reboots` | Whether this machine keeps data. Read only: setup's Mode step turns it on with `Install to this computer`, a disk NONOS was installed to has it on at every boot, and without it nothing is written to disk. |

The note under the row says `Files and installed apps are kept between boots`. Only part of that holds. A Qwen model installed from the store is kept, but a Linux package is held in memory until restart, and files you make in Files or Editor are not kept. What is kept when the field is on, and what is not even then, is on [Files](files.md#what-is-kept-after-power-off).

### Sound

| Row | What it changes or shows |
|---|---|
| `Output device` | What the sound hardware reported when the panel opened, for example `Speakers and headphone jack` or `Headphones`, or the reason nothing plays. |
| `System sound` | The desktop's own tones, on or off. |
| `Volume` | How loud the desktop's tones are. |
| `Alert sounds` | A tone for warnings and errors. |

`Volume` here is the tones' level only. The master volume, which every sound passes through, is set with the volume keys or the slider in Music. The notes under `System sound` and `Volume` still say Music has its own volume; in this release Music's slider is the master volume. See [Sound and media](audio.md).

### Updates

| Row | What it shows |
|---|---|
| `Version`, `Commit`, `Toolchain`, `Architecture` | What was recorded when this image was built. |

The panel shows no "signed" badge: Settings cannot check the image's signature itself, so it does not claim one. To install a newer image, see [Update](../install/update.md).

### Developer

| Row | What it changes |
|---|---|
| `Preemptive scheduling` | Whether the timer ends a program's turn, so none can hold the processor. The policy store passes the change to the kernel. |

## What Settings does not have

- No keyboard layout row. The layout is chosen at first boot and cycled with `Ctrl+Alt+Space`; see [Keyboard layouts](keyboard-layouts.md). The policy store has the field, but Settings does not list it.
- No switch to turn back on an app turned off at setup. Settings does not list that field either.
- No shutdown or restart. The desktop has no way to power off in this release.

## How long a change lasts

The policy store keeps its values in memory.

- On an amnesic boot, the default, every change is gone at power off.
- On a machine where `Keep data across reboots` is on, the policy store writes the values it keeps to `/nonos/settings/values` about a second after your last change, and puts them back at the next boot after setup's answers.

## Where this comes from

The source behind the facts above, at the commit in the footer.

- How Settings works
  - An editor for the policy store: `capsule_settings` in `userland/capsule_settings/README.md:5-6`.
  - The `settings applied` notice: `notify_applied` in `userland/capsule_settings/src/settings/ipc/notify_shell.rs:29-32`.
  - The status strip when the policy store does not answer: `message` in `userland/capsule_settings/src/settings/ui/status_bar.rs:51-53`.
  - The build checks rows against the list of fields: `all_placed` in `userland/capsule_settings/src/settings/schema/coverage.rs:50-61`, and `all_listed` in `userland/capsule_settings/src/settings/schema/coverage_listed.rs:38-60`.
- Moving around
  - The keys: `on_event_browsing` in `userland/capsule_settings/src/settings/event/on_event_browsing.rs:30`.
- The nine panels
  - Sidebar order: `SECTIONS` in `userland/capsule_settings/src/settings/section.rs:30`.
  - The rows of each panel are in `userland/capsule_settings/src/settings/schema/blocks/`, and the labels come from `label_of` in `userland/policy_proto/src/field_label.rs:19`.
- Network
  - The three route names: `ROUTE_LABELS` in `userland/policy_proto/src/route.rs:35`.
  - The adapter is named, never driven: `PCI_CLASS_NETWORK` in `userland/capsule_settings/src/wifi/interface.rs:17-27`.
  - Downloads take Anyone and the wallet never goes direct: `for_installs` and `for_wallet` in `userland/nonos_route_link/src/chosen.rs:45-53`.
- Wi-Fi
  - Saved only when data is kept: `put` in `userland/nonos_wifi_client/src/saved/write.rs:17-22`.
  - Open networks refused: `OpenNetwork` in `userland/nonos_wifi_core/src/mlme/beacon.rs:50-51`.
  - The panel's keys: `wifi_key` in `userland/capsule_settings/src/settings/event/wifi_key.rs:42-55`.
- Privacy
  - `Keep data across reboots` is read only: `read_only` in `userland/capsule_settings/src/settings/schema/read_only.rs:27-29`.
- Sound
  - Music's slider is the master volume: `MasterVolume` in `userland/capsule_audio_player/src/audio_client/master.rs:17-23`.
- Updates
  - No signed badge: `UPDATES` in `userland/capsule_settings/src/settings/schema/blocks/updates.rs:19-22`.
- Developer
  - The change goes to the kernel: `on_bool_set` in `userland/capsule_policy/src/push/on_bool_set.rs:22`.
- What Settings does not have
  - The fields Settings lists: `ALL_FIELDS` in `userland/capsule_settings/src/settings/schema/all_fields.rs:24`.
- How long a change lasts
  - Written about a second after the last change: `QUIET_MS` in `userland/capsule_policy/src/keep/tick.rs:29-31`.
  - The kept fields and `/nonos/settings/values`: `KEPT` in `userland/policy_proto/src/settings_record.rs:41-47`.

## See also

- [The desktop](desktop.md)
- [Files](files.md)
- [Keyboard layouts](keyboard-layouts.md)
- [Sound and media](audio.md)
- [Wi-Fi and networking](wifi-and-networking.md)
- [Privacy networks](privacy-network.md)
- [First boot](../install/first-boot.md)
