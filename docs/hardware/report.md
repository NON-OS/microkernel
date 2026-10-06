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
