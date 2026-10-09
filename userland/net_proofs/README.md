# net_proofs

Host-runnable proofs for the network capsules' untrusted-input parsers. Each
parser is included from its capsule through `#[path]` and run on the host against
large adversarial input sets. A network parser is a direct attack surface, so
the properties are safety properties: no panic, no out-of-bounds access, and
termination.

## DNS

The response parser terminates and never panics across every two-byte
compression-pointer value, including a self-referential pointer. The classic
compression-pointer loop, a standard denial of service in naive DNS code, is
absent.

## ICMP and ARP

ICMP parsing never panics and never returns a payload slice outside the input.
ARP parsing never panics and rejects every truncated packet.

## TCP

The segment parser never panics and never returns a payload slice outside the
segment. The out-of-order reassembly buffer never panics on hostile streams of
overlapping, out-of-order, and sequence-wrapping segments, and joins contiguous
data in order while stopping at a gap.

## DHCP

The reply parser walks a variable-length option list. It never panics, and an
option whose length field runs past the packet is rejected rather than read out
of bounds.

## Tables and rules from capsule source

Beyond the parsers, the crate runs some of the capsules' own state and rules:

- `core_table_tests.rs`: `net.core`'s connection and port tables, their
  ownership, the per-client share, the take-out of ended clients' entries and
  the forget on a rebuilt stack.
- `udp_bind_tests.rs`: `net.udp`'s bind table, its per-client share, and the
  freeing of ended clients' ports.
- `arp_learn_tests.rs`: what an inbound ARP packet may teach `net.l2`'s cache
  and when it is answered, through the real handler.
- `dhcp_frame_tests.rs`: the IPv4 and UDP checks the DHCP client makes on a
  reply it reads off the link.
- `dns_answer_tests.rs`: the answer for the name asked, along its CNAME chain.
- `lease_admin_tests.rs`: only Settings may request, renew or release the
  lease.
- `sntp_tests.rs`: the NTP client's request and the reply checks: mode,
  stratum, leap indicator, the echoed nonce and a time before 1970.

## Run

```sh
cd userland/net_proofs
cargo test --release
```

`nix flake check` runs it as `proofs-net_proofs` (`tools/nix/checks.nix`).
`.github/workflows/fuzz.yml` fuzzes the `dns` and `net_packets` targets in
`fuzz/`.

See [the network stack](../../docs/handbook/network/stack.md) and
[proofs](../../docs/handbook/verification/proofs.md).
