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

## RTL8169 family, with the RTL8125

### Identity

The driver takes vendor 0x10EC, PCI class network, subclass Ethernet, with a device id in `RTL8169_DEVICE_IDS` (`userland/capsule_driver_rtl8169/src/discover/support.rs:21-30`, `is_supported`). The ids and what Linux's r8169 names them (`userland/capsule_driver_rtl8169/src/constants/pci.rs:19-31`, `RTL8169_DEVICE_IDS`):

| Device id | Chip |
|---|---|
| 8169, 8167 | PCI RTL8169 and RTL8110 |
| 8168, 8161, 8162 | RTL8168 and RTL8111 |
| 2502, 2600 | parts built on the RTL8168 |
| 8136 | RTL810x, 10/100 only |
| 8125 | RTL8125, 2.5 GbE |
| 3000 | a part built on the RTL8125 |

Left out on purpose: 8126 (RTL8126A, 5 GbE) and 8127 (RTL8127A, 10 GbE), whose start differs from the RTL8125's; 8129, which Linux's 8139too also claims; and 5000 and 0e10, whose parts Linux does not name.

### Identifying the chip

The PCI id does not settle the chip. Before writing anything, `identify` reads the XID from TxConfig and looks it up in a copy of Linux's chip table (`userland/capsule_driver_rtl8169/src/chip/detect.rs:36-55`, `identify`). Then:

- A chip of version 70 or later, the RTL8126A and RTL8127A families, is refused (`userland/capsule_driver_rtl8169/src/chip/detect.rs:56-60`, `ChipError::Unsupported`).
- An RTL8125 whose register window is too small for its queue register is refused (`userland/capsule_driver_rtl8169/src/chip/detect.rs:61-63`, `BarTooSmall`).
- The start sequence follows the family: RTL8169, RTL8125, or RTL8168 with the extra RTL8168G steps (`userland/capsule_driver_rtl8169/src/hw/start.rs:30-48`, `start_8125`).
- The RTL8125 moved its interrupt and transmit-poll registers; the driver uses the moved ones (`userland/capsule_driver_rtl8169/src/regmap/layout.rs:20-25`, `REG_TX_POLL_8125`).

### Authority

The mask is 0xB8038: IPC, Memory, Crypto, Driver, DeviceEnum, Mmio and Dma, with Debug listed as optional (`userland/capsule_driver_rtl8169/Capsule.mk:19-24`, `CAPSULE_OPTIONAL_CAPS`). The kernel's spawn request holds no Debug in any build (`src/hardware/rtl8169_capsule/spawn.rs:51-60`, `requested_caps`), so the driver's `rtl8169:` lines never reach the log in 0.9.2. The service is `driver.rtl8169_0` on port 4214 (`userland/capsule_driver_rtl8169/Capsule.mk:13`, `CAPSULE_SERVICE_ENDPOINT`).

### How it runs

- Memory-mapped registers only, no port I/O.
- It polls and binds no interrupt.
- It draws a station address every boot and writes it into the IDR registers; the factory address is not a fallback (`userland/capsule_driver_rtl8169/src/init/mac.rs:23-33`, `program`).
- Operation 6 returns a register snapshot with status 0 (`userland/capsule_driver_rtl8169/src/protocol/ops.rs:17-22`, `OP_STATS`), the cause of [the receive fault](README.md#the-receive-fault).

The kernel starts it on every boot of the full image. The `nonos-mk-ethernet-prod` profile carries it to show that a driver whose chip is absent exits and lets the boot go on; the build comment gives the reason as QEMU having no model of this chip (`mk/20-build.mk:1145-1150`, `ETHERNET_DRIVER_ARTIFACTS`).

### Tests

`rtl8169_proofs` compiles the driver's source and runs it against a modelled part: the station address with the entropy source off, the rings, chip identification from the XID, the RTL8168 and RTL8125 register maps, the per-family start, and link speed decode up to 2500 Mb/s.

On this commit the flake check `proofs-rtl8169_proofs` fails. Its tests pass (the log reports 67 passed); then clippy, which the check runs with warnings denied, stops on `manual_div_ceil` in the driver's log formatter (`userland/capsule_driver_rtl8169/src/log/line.rs:39-41`, `digits`). The lint asks for `div_ceil` in place of an expression that computes the same value.

```sh
cd userland/rtl8169_proofs && cargo test --release --config profile.release.overflow-checks=true
```

Not tested in this release.
