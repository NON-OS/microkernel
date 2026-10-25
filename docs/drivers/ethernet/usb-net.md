# USB networking

Five USB network driver [capsules](../../overview/glossary-boot-root-record.md#capsule) (CDC-ECM,
CDC-NCM, RNDIS, ASIX AX88179 and Realtek RTL8153, the last two now quarantined) sit in the tree on
one shared core, `nonos_usbnet`; none runs in 0.9.2. This page says what kept them out and where
they stand now; [what each binds](usb-net-binding.md) has its own page.

## State

USB Ethernet adapters and USB tethering from a phone do not work in NONOS 0.9.2. Four facts in that release's code stopped them, and each is now addressed on this tree:

1. No image built them. Each now has a kernel mirror and a spawn wherever an xHCI controller is
   (`src/userspace/init/spawn_plan/drivers_usbnet.rs`, `spawn`), and the build includes their
2. The USB host driver did not serve their polled bulk IN, operation 0x0014
   (`userland/nonos_usbnet/src/xhci/wire.rs`, `OP_BULK_IN_POLL`). It now keeps one IN transfer armed
   per slot on its own page and answers each look at once
   (`userland/capsule_driver_xhci/src/controller/transfer/bulk/poll.rs`, `poll_bulk_in`).
3. The kernel held `driver.xhci0` to the USB keyboard, storage and audio drivers alone; the three
   are now on its list (`src/services/registry/held_lists.rs`, `XHCI_CLIENTS`), and each is held to
   `net.core` and `net.l2` through the card table (`src/services/registry/nics.rs`, `NICS`).
4. The shared core answered operation 6, which `net.core` asks first as its receive batch, with its
   counters and status 0, so `net.core` would have taken the counters for frames ([the receive
   fault](README.md#the-receive-fault)). The counters are now on 7 and 6 is refused
   (`userland/nonos_usbnet/src/nnet/wire.rs`, `OP_STATS`), so `net.core` reads these drivers one
   frame at a time with operation 5.

`net.core` tries the three after the PCI cards, in the kernel's card table order (`userland/capsule_net_core/src/setup/candidates.rs`, `candidate`). For a machine with no supported Ethernet port, see [Wi-Fi chips with no driver](../wifi/not-supported.md#what-to-use-instead) for what does work.

## How the pieces fit

```mermaid
flowchart LR
  Dev["USB adapter or phone"] --> Xhci["driver.xhci0"]
  Xhci --> Class["class capsule"]
  Usbnet["nonos_usbnet"] --> Class
  Class --> Core["net.core"]
```

Each class capsule holds only its binding and its framing. The shared core `nonos_usbnet` holds the client side of every USB host, the descriptor walk, the search for a device on the hosts' root ports and the NNET frame service that `net.core` speaks (`userland/nonos_usbnet/src/lib.rs`, `run`). No USB network capsule touches the controller: it holds no Driver, DeviceEnum, Mmio, Irq, Dma or Pio [capability](../../overview/glossary-boot-root-record.md#capability-word). What each capsule binds is on [its own page](usb-net-binding.md).

The ASIX AX88179 and Realtek RTL8153 capsules are quarantined (branches `quarantine/ax88179` and `quarantine/rtl8153`): their register maps, values and bring-up order were taken from another driver, and no public programming reference exists to rewrite them from (Decision 10). RTL8153 adapters are bound by the CDC-ECM capsule in their ECM configuration.

## Addresses and capabilities

Like the PCI drivers, these capsules never use the adapter's own station address. Each draws one from `CryptoRandom` once a run, before the first bind, and hands the same one to every device it binds, so a device bound again after a loss keeps the address `net.core` took at the first bind; without entropy no device is bound (`userland/nonos_usbnet/src/station.rs`, `draw`; `userland/nonos_usbnet/src/scan/net.rs`, `NetScanner::new`). CDC-NCM gives it with SET_NET_ADDRESS when the device offers that, RNDIS sets OID_802_3_CURRENT_ADDRESS, and a device that cannot take it has its packet filter opened; CDC-ECM has no request for it, so its filter is always opened.

All three ask for the capability mask 0x38: IPC, Memory and Crypto, the bit `CryptoRandom` is gated on, with Debug (0x100) optional (`userland/capsule_driver_cdc_ecm/Capsule.mk`, `CAPSULE_REQUIRED_CAPS`; `src/capabilities/types/defs.rs`, `Crypto`). Their loop drops the messages it sent itself by its own pid, which it reads back with `MkServiceLookup` under its service name rather than `MkGetPid`, so no CoreExec is needed (`userland/nonos_usbnet/src/run/me.rs`, `own_pid`). The kernel admits `MkDebug` only with Debug (`src/syscall/contract/cap_table/mk_device.rs`, `can_debug`), so the `[usbnet ...]` lines reach the log only in a build that compiles `capsule-serial-debug`.

## Tests on this commit

Each capsule has a [proof crate](../../overview/glossary-peer-list.md#proof-crate) that runs its binding and framing on the host against scripted devices, some of them built from QEMU's `usb-net` descriptors. No QEMU run and no hardware boot exists for any of them. The counts below are host `cargo test` runs on this commit; the flake checks have not been run on it.

| Crate | Result |
|---|---|
| `usbnet_proofs` | 53 tests pass |
| `cdc_ecm_proofs` | 24 tests pass |
| `cdc_ncm_proofs` | 46 tests pass |
| `rndis_proofs` | 30 tests pass |

```sh
cd userland/cdc_ncm_proofs && cargo test --release --config profile.release.overflow-checks=true
```

Not tested in this release.

## See also

- [What each USB capsule binds](usb-net-binding.md)
- [Ethernet drivers](README.md)
- [Realtek Ethernet](realtek.md)
- [USB host controller](../usb/README.md)
- [Wi-Fi chips with no driver](../wifi/not-supported.md)
- [Hardware support matrix](../../hardware/MATRIX.md)
