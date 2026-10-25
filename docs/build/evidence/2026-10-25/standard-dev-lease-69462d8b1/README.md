# standard-dev leases: booted on main 69462d8b1, and before H-08

`tools/nonos-dev-image --profile standard` built and sealed standard-dev
with throwaway keys, in its own copy of the tree; no key under `.keys/` was
read. Two images, one lane:

- main **69462d8b1**, which has H-08 (1a9842924): its sha256 is
  4d645114c2c35d7ea41b45b41440eae53292a8c0343b8330f7438588b649a087;
- main-era **22cf8843e**, which predates H-08.

## The lane

`qemu.txt`: q35 with `kernel-irqchip=split` and the Intel IOMMU
(`--lane iommu=intel`), virtio-net behind it on QEMU user networking, 6 GiB,
a software TPM, `--fresh`, run for 15 minutes. The harness's `--capture`
wrote what the guest sent and what it was sent as two pcaps. The pcaps are
not committed; `traffic.txt` counts them and the `dhcp-*.txt` files list each
DHCP message with its time from the first captured frame.

## Both images lease

| | 69462d8b1 | 22cf8843e |
|---|---|---|
| DISCOVER, OFFER, REQUEST, ACK | +33.491 s to +33.502 s | +38.727 s to +38.793 s |
| address | 10.0.2.15 | 10.0.2.15 |
| TCP after (Anyone :9230, :9000, :443) | yes | yes |
| `[NET-CORE] lease` lines in the serial | 2, and state=3 | none |
| net.core's caps at spawn | 0x57D | 0x47D |

`serial.log` is 69462d8b1's boot. Lines 574 to 588 have net.core's
registration, its bind on driver.virtio_net0, "DHCP asking for a lease",
the lease 10.0.2.15/24 gw 10.0.2.2, its DNS server, and lease-status
state=3; setup was reached (`setup.png`, "[SETUP] keyboard held").

## Why 22cf8843e's serial was silent

Before H-08, the nix config took `capsule-serial-debug` out of a profile's
development twin as well as the profile, so a standard-dev kernel granted no
capsule Debug (bit 0x100): net.core spawned with 0x47D, not 0x57D
(`serial-pre-h08-22cf8843e-net.txt`), and every capsule's own lines, the
lease lines among them, never reached the serial. The kernel's own
`[DIAG] ipc.call` lines still printed, which is why those boots read as
"net.core goes quiet". Their pcap shows the lease was there all along.

So a standard-dev boot from before 1a9842924 cannot say from its serial
whether it leased; docs/market/evidence/2026-10-25/meter-717c5d6c5/standard-dev
is one (net.core 0x47D there too).
