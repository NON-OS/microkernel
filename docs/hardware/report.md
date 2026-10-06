# Reporting a machine

How anyone can tell the NONOS team what NONOS 0.9.2 did on their machine, so that it enters the [support matrix](MATRIX.md) or gets a driver fixed.

## What a useful report holds

- Which image you ran: the file name and where it came from, or, if you built it, the commit and the profile (Standard, Hardened, Air-Gapped, QEMU).
- The CPU family and the memory size, for example Intel Gemini Lake, 8 GB.
- The ids of the chips that matter: PCI vendor:device for controllers, USB vendor:product for adapters.
- The NONOS log lines for those devices.
- What you did, what you expected and what happened, and whether it happens on every boot.

A device that works is worth reporting too. The matrix lists real hardware only from reports.

## Read the log in the NONOS Terminal

You do not need a serial port. The kernel keeps what it writes to its serial console in memory: the first 64 KiB of the boot for good, then the latest 64 KiB (`CAPACITY`, `src/sys/serial/tail.rs:27-32`). The Terminal's `log` command reads it back (`run`, `userland/capsule_terminal/src/command/builtin/log.rs:33-55`).

Open the Terminal and type:

```text
log
log driver- exit
log touchpad i2chid
log hda audio
log rtl8821ce nvme
log warn error
```

Not tested in this release.

`log` alone shows the newest 200 lines (`NEWEST`, `userland/capsule_terminal/src/command/builtin/log.rs:31`). With words, it shows every line that names any of them, ignoring case (`contains`, `userland/capsule_terminal/src/command/builtin/log.rs:57-59`). Like any command, its output can go to a file, as in `log hda > hda-log.txt`.

The log is there only on images built with `capsule-serial-debug`: Standard, QEMU and development images. A Hardened or Air-Gapped image keeps nothing (`keep`, `src/sys/serial/tail.rs:49-54`), and `log` answers `log: no line matches`. On such an image, describe what you saw on the screen instead: the kernel's own lines also appear there during boot (`info`, `src/sys/boot_log/output.rs:26-31`).

## What the lines mean

| Search word | Example line | Written by |
|---|---|---|
| `driver-` | `[DRIVER-HDA] capsule spawned`, `[DRIVER-NVME] no controller present, not spawned` | the kernel, for every driver it starts at boot |
| `exit` | `[EXIT] <service> status 2: no device present, not started`, or status 6: `device present, bring-up failed and was given up` | the kernel, when a driver ends |
| `touchpad` | `[INFO] touchpad <id> on I2C bus <bus> addr <address> <speed> kHz, ...` | the kernel, from the ACPI tables |
| `acpi` | `[ACPI] power button: fixed feature, polled; a press goes to the desktop as the Power key` | the kernel |
| `hda`, `audio` | `[HDA] controller 8086:<id>`, `[HDA] no playable output: <reason>` | the sound driver and server |
| `i2chid` | `[i2chid] bind addr=<address> ...` | the touchpad driver |
| `rtl8821ce`, `nvme` | the Wi-Fi and NVMe drivers' own lines | those drivers |

The kernel writes the `[DRIVER-...]` lines when it starts a driver or decides not to (`present`, `src/userspace/init/spawn_plan/device_present.rs:29-35`), and the `[EXIT]` line when a driver ends with any status other than 0, in words for status 2 and 6 and as `ended with an error` for the rest (`words`, `src/process/exit/end_rule.rs:38-44`).

A driver writes lines of its own only when the kernel grants it the Debug [capability](../overview/glossary.md#capability), as it does for the sound driver (`serial_debug_cap`, `src/hardware/hda_capsule/spawn.rs:58`). The PS/2, I2C controller, xHCI, USB HID, USB storage, AHCI and Intel Wi-Fi drivers, among others, are started without it; the PS/2 driver's list is one example (`requested_caps`, `src/hardware/ps2_kbd_capsule/spawn.rs:51-57`). For those drivers the kernel's `[DRIVER-...]` and `[EXIT]` lines are what the log has.

Three more Terminal commands help:

```text
version
capsules
display
```

Not tested in this release.

`version` names the release and who signed the Terminal (`run`, `userland/capsule_terminal/src/command/builtin/version.rs:30-40`). `capsules` lists every [capsule](../overview/glossary.md#capsule) running, with the capabilities the kernel granted it, so you can see which drivers are up (`run`, `userland/capsule_terminal/src/command/builtin/capsules.rs:34`). `display` prints the screen size (`run`, `userland/capsule_terminal/src/command/builtin/display.rs:22-39`).
