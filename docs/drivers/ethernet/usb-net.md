# USB networking

The tree holds five USB network driver [capsules](../../overview/glossary.md#capsule) (CDC-ECM, CDC-NCM, RNDIS, ASIX AX88179 and Realtek RTL8153) on one shared core, `nonos_usbnet`; none of them runs in 0.9.2. This page says what each binds, what kept them out of 0.9.2, and where they stand now.

## State

USB Ethernet adapters and USB tethering from a phone do not work in NONOS 0.9.2. Four facts in that release's code stopped them, and each is now addressed on this tree:

1. No image built them. Each now has a kernel mirror and a spawn wherever an xHCI controller is (`src/userspace/init/spawn_plan/drivers_usbnet.rs`, `spawn`), and none has an owner certificate yet, so no image carries one until the owner signs it.
2. The USB host driver did not serve their polled bulk IN, operation 0x0014 (`userland/nonos_usbnet/src/xhci/wire.rs`, `OP_BULK_IN_POLL`). It now keeps one IN transfer armed per slot on its own page and answers each look at once (`userland/capsule_driver_xhci/src/controller/bulk/poll.rs`, `poll_bulk_in`).
3. The kernel held `driver.xhci0` to the USB keyboard, storage and audio drivers alone; the five are now on its list (`src/services/registry/held_table.rs`, `XHCI_CLIENTS`), and each is held to `net.core` and `net.l2` through the card table (`src/services/registry/nics.rs`, `NICS`).
4. The shared core answered operation 6, which `net.core` asks first as its receive batch, with its counters and status 0, so `net.core` would have taken the counters for frames ([the receive fault](README.md#the-receive-fault)). The counters are now on 7 and 6 is refused (`userland/nonos_usbnet/src/nnet/wire.rs`, `OP_STATS`).

`net.core` tries the five after the PCI cards, in the kernel's card table order (`userland/capsule_net_core/src/setup/candidates.rs`, `candidate`). For a machine with no supported Ethernet port, see [Wi-Fi chips with no driver](../wifi/not-supported.md#what-to-use-instead) for what does work.

## How the pieces fit

```mermaid
flowchart LR
  Dev["USB adapter or phone"] --> Xhci["driver.xhci0"]
  Xhci --> Class["class capsule"]
  Usbnet["nonos_usbnet"] --> Class
  Class --> Core["net.core"]
```

Each class capsule holds only its binding and its framing. The shared core `nonos_usbnet` holds the client side of `driver.xhci0`, the descriptor walk, the search for a device on the root ports and the NNET frame service that `net.core` speaks (`userland/nonos_usbnet/src/lib.rs:17-21`, `run`). No USB network capsule touches the controller: it holds no Driver, DeviceEnum, Mmio, Irq, Dma or Pio [capability](../../overview/glossary.md#capability-word).

## What each capsule would bind

| Capsule | Service and port | Binds |
|---|---|---|
| `capsule_driver_cdc_ecm` | `driver.cdc_ecm0`, 4250 | a communications interface of subclass 0x06 with a Union and an Ethernet Networking descriptor |
| `capsule_driver_cdc_ncm` | `driver.cdc_ncm0`, 4252 | a communications interface of subclass 0x0D, protocol 0, with the NCM descriptor |
| `capsule_driver_rndis` | `driver.rndis0`, 4254 | three RNDIS control classes, listed below |
| `capsule_driver_ax88179` | `driver.ax88179_0`, 4256 | 13 USB vendor:product pairs |
| `capsule_driver_rtl8153` | `driver.rtl8153_0`, 4258 | 19 USB vendor:product pairs |

- CDC-ECM: `find_ecm` takes the first communications interface of subclass 0x06 that has a Union descriptor, an Ethernet Networking descriptor and a data interface with a bulk pair (`userland/capsule_driver_cdc_ecm/src/ecm/function.rs:26-57`, `find_ecm`). It leaves 0bda:8153 and 17ef:721e to the RTL8153 capsule, as Linux does (`userland/capsule_driver_cdc_ecm/src/ecm/vendor.rs:22`, `LEFT_TO_RTL8153`). One Ethernet frame goes in each bulk transfer, with one padding byte when its length is a multiple of 64 or of the packet size (`userland/capsule_driver_cdc_ecm/src/ecm/frame.rs:22-36`, `padded_len`).
- CDC-NCM: `find_ncm` needs subclass 0x0D, protocol 0, and both the Ethernet Networking and the NCM functional descriptors (`userland/capsule_driver_cdc_ncm/src/ncm/function.rs:26-64`, `find_ncm`).
- RNDIS: the control interface must be CDC ACM with the vendor protocol (0x02, 0x02, 0xFF), the wireless controller class phones use for tethering (0xE0, 0x01, 0x03), or the miscellaneous RNDIS class (0xEF, 0x04, 0x01) (`userland/capsule_driver_rndis/src/rndis/function.rs:27-31`, `CONTROL_CLASSES`). A communications function that declares modem capabilities is a modem and is skipped (`userland/capsule_driver_rndis/src/rndis/function.rs:57-64`, `acm_capable`).
- ASIX AX88179: the ids Linux's ax88179_178a driver binds, with 0b95:1790 for the AX88179 and 0b95:178a for the AX88178A; the other 11 pairs are adapters built on these chips (`userland/capsule_driver_ax88179/src/ax/products.rs:17-35`, `PRODUCTS`).
- Realtek RTL8153: 0bda:8153 for the RTL8153 and RTL8153B, and 18 pairs for adapters and docks built on them. For those, the chip's version register decides, and an adapter built on another chip is refused by name (`userland/capsule_driver_rtl8153/src/r8153/ids.rs:17-51`, `RTL8153_FAMILY`). The RTL8152, the RTL8156 family and two ids whose chip Linux does not name are left out.

## Addresses and capabilities

Unlike the PCI drivers, these capsules use the adapter's own station address. They hold no Crypto capability to draw one. The AX88179 capsule reads AX_NODE_ID and stops the bind when it holds no usable address (`userland/capsule_driver_ax88179/src/ax/station.rs:27-37`, `NO_STATION`), and the RTL8153 capsule reads PLA_BACKUP (`userland/capsule_driver_rtl8153/src/r8153/up/mac.rs:32-41`, `station_address`).

All five ask for IPC and Memory, the mask 0x18, with Debug (0x100) optional, granted only by a build that compiles `capsule-serial-debug` (`userland/capsule_driver_cdc_ecm/Capsule.mk`, `CAPSULE_REQUIRED_CAPS`, `CAPSULE_OPTIONAL_CAPS`). Their loop drops the messages it sent itself by its own pid, which it reads back with `MkServiceLookup` under its service name rather than `MkGetPid`, so no CoreExec is needed (`userland/nonos_usbnet/src/run/me.rs`, `own_pid`). Earlier they asked for 0x200018, which grants InputSource and no Debug.

## Tests on this commit

Each capsule has a [proof crate](../../overview/glossary.md#proof-crate) that runs its binding and framing on the host against scripted devices, some of them built from QEMU's `usb-net` descriptors. No QEMU run and no hardware boot exists for any of them.

| Check | Result |
|---|---|
| `proofs-usbnet_proofs` | passed, 17 tests |
| `proofs-cdc_ecm_proofs` | passed, 7 tests |
| `proofs-cdc_ncm_proofs` | passed, 33 tests |
| `proofs-rndis_proofs` | passed, 22 tests |
| `proofs-ax88179_proofs` | passed, 25 tests |
| `proofs-rtl8153_proofs` | passed, 27 tests |

```sh
cd userland/cdc_ncm_proofs && cargo test --release --config profile.release.overflow-checks=true
```

Not tested in this release.

## See also

- [Ethernet drivers](README.md)
- [Realtek Ethernet](realtek.md)
- [USB host controller](../usb/README.md)
- [Wi-Fi chips with no driver](../wifi/not-supported.md)
- [Hardware support matrix](../../hardware/MATRIX.md)
