# Reporting a machine

How anyone can tell the NONOS team what NONOS 0.9.2 did on their machine, so that it enters the [support matrix](MATRIX.md) or gets a driver fixed.

## What a useful report holds

- Which image you ran: the file name and where it came from, or, if you built it, the commit and the profile (Standard, Hardened, Air-Gapped, QEMU).
- The CPU family and the memory size, for example Intel Gemini Lake, 8 GB.
- The ids of the chips that matter: PCI vendor:device for controllers, USB vendor:product for adapters.
- The NONOS log lines for those devices.
- What you did, what you expected and what happened, and whether it happens on every boot.

A device that works is worth reporting too. The matrix lists real hardware only from reports.
