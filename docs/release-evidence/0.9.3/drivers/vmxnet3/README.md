# vmxnet3 on QEMU: DHCP lease through the card

What ran: the Standard development image (`tools/nonos-dev-image --profile standard`, the `standard-dev` profile: Standard's drivers, throwaway keys) built from `drivers` at 4b1eeb014, booted under QEMU (TCG, no KVM) with QEMU's vmxnet3 model in place of the virtio NIC, user-mode networking. 27 October 2026, by S4.

The command:

```sh
tools/nonos-drive --image target/dev/tree/target/release/standard-dev/nonos.img --out <dir> \
  --qemu-arg=--lane --qemu-arg=nic=vmxnet3 "until:[NET-CORE] lease :1200" "wait:45"
```

The lines that prove it, from [serial.log](../../../../hardware/evidence/2026-10-26/runs-6f0bbe716/nic-vmxnet3/serial.log):

```text
[DRIVER-VMXNET3] capsule spawned
driver.vmxnet3: up, link up 1000 Mb/s
driver.vmxnet3: link up
[NET-CORE] bind: interface up on driver.vmxnet3_0
[NET-CORE] lease 10.0.2.15/24 gw 10.0.2.2
[NET-CORE] rx batch: 1024 calls, 0 not answered in time, bound 24 ms
```

- The driver starts only for its own card: every other wired driver says `no controller present, not spawned`.
- `driver.vmxnet3_0` is bound by net.core and the lease is bound: QEMU's DHCP server's offer and ack came in through the card, so its receive path works end to end.
- The `rx batch` line is net.core reading the card with OP_RX_BATCH, and how many of its calls were answered late (a late batch is kept and sent again, not lost).
- No line holds a MAC address.

Not shown here: traffic past the lease to the internet. This machine has no outbound network, so Nym and Anyone reach a TCP handshake through the card and no further; that run is the local machine's.
