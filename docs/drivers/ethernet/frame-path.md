# How a frame travels

How a wired frame reaches `net.core`: the driver capsules on one side, the [hardware broker](../../overview/glossary.md#hardware-broker) on the other, and the link protocol between the stack and every driver. The drivers themselves are listed on the [Ethernet drivers](README.md) page.

```mermaid
flowchart LR
  Core["net.core"] --> E1000["driver.e1000_0"]
  Core --> R8169["driver.rtl8169_0"]
  Core --> R8139["driver.rtl8139_0"]
  Core --> Virtio["driver.virtio_net0"]
  E1000 --> Broker["hardware broker"]
  R8169 --> Broker
  R8139 --> Broker
  Virtio --> Broker
  Broker --> Card["PCI network card"]
```

Each driver is a capsule that moves raw Ethernet frames and nothing more; ARP, IP, DHCP, DNS and TCP live in `net.core` above it. The driver reaches its PCI network card only through [grants](../../overview/glossary.md#grant) from the hardware broker: a device claim, a register mapping or port range, and DMA buffers.

## The link protocol

Every wired driver answers the same link protocol: a 20-byte header tagged `NNET` (0x4E4E4554), and the operations link status (2), MAC address (3), transmit (4) and receive (5) (`userland/capsule_net_core/src/protocol/ops.rs:17-25`, `MAGIC_NNET`, `OP_RX_PACKET`). Operation 6 is where they part. `net.core` and the virtio driver use it for a receive batch (`userland/capsule_net_core/src/protocol/ops.rs:26-27`, `OP_RX_BATCH`); e1000, RTL8139 and RTL8169 use it for a register snapshot (`userland/capsule_driver_e1000/src/protocol/ops.rs:23-28`, `OP_STATS`), and the USB drivers answer it with `E_INVAL`, their counters being at 0x0100.

On its first poll of a newly bound card, `net.core` sends operation 6 with no body, once per port (`userland/capsule_net_core/src/device/rx_probe.rs:30-40`, `serves`; `userland/capsule_net_core/src/device/batch_call.rs:54`, `OP_RX_BATCH`). An empty batch in a status 0 reply marks the driver as one that serves receive batches, and from then on that card is asked only for batches; any other answer, or none, and the card is read one frame at a time with operation 5 (`userland/capsule_net_core/src/device/receive.rs:22-32`, `receive`). [The receive fault](README.md#the-receive-fault) says why a status 0 reply alone is not taken as a yes.

## Who may reach a driver

The kernel holds the four wired endpoints in the image to `net.core` and `net.l2`; no other capsule may send to them (`src/services/registry/held_table.rs:20-24`, `HELD`; `src/services/registry/held_table.rs:41-44`, `WIRED_STACK`). See [held endpoint](../../overview/glossary.md#held-endpoint). The USB network drivers have no entry there yet; see [USB networking](usb-net.md).

## Which card is bound

`net.core` binds the first candidate whose link answers up. Wi-Fi comes first, then `driver.virtio_net0`, `driver.e1000_0`, `driver.rtl8169_0` and `driver.rtl8139_0`, then the USB drivers (`userland/capsule_net_core/src/setup/candidates.rs:25-38`, `WIRED_NICS`). The e1000e and igc capsules are not on that list. It asks the bound driver for its MAC address only when the port it is bound to changes (`userland/capsule_net_core/src/setup/reevaluate.rs:33-54`, `reevaluate`).

## See also

- [Ethernet drivers](README.md)
- [Images and tests](images-and-tests.md)
- [USB networking](usb-net.md)
- [The broker API](../broker-api.md)
