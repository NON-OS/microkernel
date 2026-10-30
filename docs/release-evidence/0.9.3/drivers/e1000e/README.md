# e1000e on QEMU: DHCP lease through the card

What ran: the Standard development image (`tools/nonos-dev-image --profile standard`, the `standard-dev` profile: Standard's drivers, throwaway keys) built from `drivers` at 4b1eeb014, booted under QEMU (TCG, no KVM) with QEMU's e1000e model in place of the virtio NIC, user-mode networking. 27 October 2026, by S4.

The command:

```sh
tools/nonos-drive --image target/dev/tree/target/release/standard-dev/nonos.img --out <dir> \
  --qemu-arg=--lane --qemu-arg=nic=e1000e "until:[NET-CORE] lease :1200" "wait:45"
```

The lines that prove it, from [serial.log](../../../../hardware/evidence/2026-10-26/runs-6f0bbe716/nic-e1000e/serial.log):

```text
[DRIVER-E1000E] capsule spawned
e1000e: up 10d3 family=82574 link=down
e1000e: link up 1000 full
[NET-CORE] bind: interface up on driver.e1000e_0
[NET-CORE] lease 10.0.2.15/24 gw 10.0.2.2
[NET-CORE] rx batch: 1024 calls, 0 not answered in time, bound 24 ms
```

- The driver starts only for its own card: every other wired driver says `no controller present, not spawned`.
- `driver.e1000e_0` is bound by net.core and the lease is bound: QEMU's DHCP server's offer and ack came in through the card, so its receive path works end to end.
- The `rx batch` line is net.core reading the card with OP_RX_BATCH, and how many of its calls were answered late (a late batch is kept and sent again, not lost).
- No line holds a MAC address.

Not shown here: traffic past the lease to the internet. This machine has no outbound network, so Nym and Anyone reach a TCP handshake through the card and no further; that run is the local machine's.
