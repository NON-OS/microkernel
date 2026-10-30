# Hardware support matrix

Look up your chips here to see whether NONOS 0.9.3 drives them, how far, and what that answer rests on: host tests, a QEMU run or a real machine.

## How to read it

Find your machine's ids first: [Reporting a machine](report.md#read-the-ids-on-another-operating-system) shows how to read them with `lspci` and `lsusb`. Then find the row for each chip. Each row has one of four states:

- Works: the 0.9.3 image carries the whole path for this device.
- Partial: part of the path is there; the row says what is missing.
- Refused: the kernel or the driver finds the device and declines it, for the reason the row gives. Where the refusal reaches the boot log or Settings, the row says so.
- Not supported: no driver for it in the 0.9.3 image. Some of these have driver source and passing [proof crates](../overview/glossary-peer-list.md#proof-crate) in the tree, but no image builds them: they have no kernel feature and no place in the boot's spawn plan. The USB Ethernet drivers among them could not reach an adapter either, since the [held endpoint](../overview/glossary-hardware-broker.md#held-endpoint) `driver.xhci0` takes requests only from the USB HID and storage drivers (`HELD`, `src/services/registry/held_table.rs:20-32`).

How verified:

- Proof crate: host tests that compile the shipping driver source against a model of the device. The number is the test count of its flake check at commit bff12b97, and the check passes unless the row says otherwise.
- QEMU: a QEMU run in the tree attaches this device: `make boot` (`devices`, `tools/nonos_qemu/machine.py:91-100`), CI's boot smoke, or the `mk/10-qemu.mk` and `mk/40-run.mk` targets. No boot log from this commit is recorded here.

Reports since 0.9.2, for the 0.9.3 image:

| Machine | What ran | Result | Source |
|---|---|---|---|
| KVM, QEMU 7.2 q35, `-cpu host`, OVMF, 8 GiB | the G run (b53232446c), local run | ICH9 AHCI (8086:2922) with nothing attached exits 3, "controller present, nothing behind it to serve"; with a SATA disk on `ide.0` it serves it (`[ahci] serving port 0`) | local report, 28 Oct |
| the same KVM machine | an NVMe target (`-device nvme`) | the installer lists the NVMe disk and writes it | local report, 28 Oct |
| the same KVM machine | the stick on virtio-blk, then on usb-storage on qemu-xhci | the stick boots and the install flow passes | local report, 28 Oct |
| QEMU TCG q35 | dev image of main ad2a58a01 | default machine and the e1000e, igb, e1000 and RTL8139 lanes each lease; every Wi-Fi driver stays out with its card absent | [evidence](../release-evidence/0.9.3/boot-ad2a58a01/README.md) |

No other Wi-Fi card (Intel, RTL8822CE, RTL8852BE, MediaTek) has run on real hardware yet; their rows say host-proven.

Every row is for x86_64 machines; [Architectures](../architectures/README.md) says what the other ports have. The rows describe the image the Standard [build profile](../overview/glossary-boot-root-record.md#build-profile) makes. The Air-Gapped image drops every network driver (`networkFeatures`, `tools/nix/config.nix:92`), and the QEMU image leaves out the drivers only real hardware has (`microkernel-desktop-gui`, `tools/nix/config.nix:95-98`).

PCI ids are vendor:device in hex. USB ids say USB, and ACPI ids are firmware `_HID` strings. Every driver named here is a [capsule](../overview/glossary-boot-root-record.md#capsule) unless the row says kernel.

## Parts

- [Platform; Display](matrix-platform-display.md)
- [GPUs, family by family](gpu.md)
- [Storage; USB; Input](matrix-storage-usb-input.md)
- [Audio; Wi-Fi](matrix-audio-wifi.md)
- [Ethernet](matrix-ethernet.md)
- [Sources](matrix-sources.md)
- [How the matrix is kept current; See also](matrix-kept-current.md)
