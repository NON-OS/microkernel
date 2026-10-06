# Realtek Ethernet

NONOS has two Realtek PCI Ethernet driver [capsules](../../overview/glossary.md#capsule) in the image: one for the RTL8139 and one for the RTL8169 family, which covers the RTL8168, RTL8111, RTL810x and the 2.5 GbE RTL8125.

## Which driver takes which chip

| Driver | Chips | PCI ids, vendor 10ec | State |
|---|---|---|---|
| `driver.rtl8139_0` | RTL8139 | 8139 | Partial: in the image, host tests pass, [receive fault](README.md#the-receive-fault), no hardware report, no QEMU run target |
| `driver.rtl8169_0` | RTL8169, RTL8110, RTL8168, RTL8111, RTL810x, RTL8125 | 8169, 8167, 8168, 8161, 8162, 2502, 2600, 8136, 8125, 3000 | Partial: in the image, [receive fault](README.md#the-receive-fault), no hardware report; its flake check fails on a lint |
| none | RTL8126A, RTL8127A | 8126, 8127 | Not supported: left out of the id table |

The USB Realtek RTL8153 is a separate capsule, not in the image; see [USB networking](usb-net.md).

## RTL8139

### Identity and authority

The driver takes vendor 0x10EC, device 0x8139, PCI class network, subclass Ethernet (`userland/capsule_driver_rtl8139/src/discover/support.rs:21-30`, `is_supported`; `userland/capsule_driver_rtl8139/src/constants/pci.rs:17-18`, `RTL8139_DEVICE_ID`). It is the one wired driver that uses port I/O alone: its [capability](../../overview/glossary.md#capability-word) mask 0x198038 holds IPC, Memory, Crypto, Driver, DeviceEnum, Dma and Pio, and no Mmio (`userland/capsule_driver_rtl8139/Capsule.mk:19`, `CAPSULE_REQUIRED_CAPS`). The kernel requests the same set and no Debug (`src/hardware/rtl8139_capsule/spawn.rs:51-60`, `requested_caps`), so its own log lines are dropped. The service is `driver.rtl8139_0` on port 4212 (`userland/capsule_driver_rtl8139/Capsule.mk:13`, `CAPSULE_SERVICE_ENDPOINT`).

### How it runs

- Every register access goes through port I/O granted by the [hardware broker](../../overview/glossary.md#hardware-broker); packet buffers come from DMA grants.
- It polls and binds no interrupt.
- It draws a station address every boot and writes it into the IDR registers; the factory address is not a fallback (`userland/capsule_driver_rtl8139/src/init/mac.rs:23-34`, `program`).
- Operation 6 returns a register snapshot, with status 0 whenever its port reads succeed (`userland/capsule_driver_rtl8139/src/protocol/ops.rs:17-22`, `OP_STATS`), which `net.core` reads as a receive batch; see [the receive fault](README.md#the-receive-fault).

### Tests

`rtl8139_proofs` checks the receive ring walk: every read lands at the wrapped offset and a frame copy never leaves the caller's buffer, whatever length the device writes. Kani harnesses for it sit in `userland/rtl8139_proofs/src/kani_proofs/`; the flake check does not run them. The flake check `proofs-rtl8139_proofs` passed with 15 tests on this commit.
