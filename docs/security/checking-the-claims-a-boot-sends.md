# Checking the security claims yourself: A boot sends nothing unexpected

Part of [Checking the security claims yourself](checking-the-claims.md).

## A boot sends nothing unexpected

What a boot sent is read off the wire, in two captures of the guest's card.
A QEMU boot with a network card, `QEMU_NET_MODE` set to `nat` or `hostfwd`,
adds two `filter-dump`s of that card's netdev (`mk/10-qemu.mk:124-143`, and
`capture` in `tools/nonos_qemu/machine.py` for `tools/nonos-qemu --capture`):

- `queue=rx`, the netdev's receive queue, to `QEMU_NET_CAPTURE`: every frame the guest sent, and nothing else. This is the egress, and it is what is judged.
- `queue=tx`, to the same name ending `-in.pcap`: what the network sent the guest. It is read only for the relays an authority's consensus names. The guest cannot write into it, so it cannot vouch for a host.

The mode defaults to `nat` and the capture to `target/qemu-net.pcap`; `QEMU_NET_MODE=off` attaches no card, and an empty `QEMU_NET_CAPTURE` boots without a capture.

`tools/nonos-pcap-egress` judges every egress frame (`tools/nonos_egress/`, the rules in `judge.py`). No source address is exempt: the guest picks its own, so a frame from QEMU's host address is judged like any other. The only frames not judged as the guest's are the firmware's. Those are ARP, DHCP, and IPv6 neighbour discovery and multicast listener messages, from the NIC's assigned address (`--nic-mac`, QEMU's `52:54:00:12:34:56` by default), sent before NONOS sends a frame of its own. Any other frame from that address fails as `mac`. The run fails on any of these, each named in the output as `[egress] FAIL <kind>: <what>`:

| kind | what |
|---|---|
| `dns` | UDP or TCP to port 53 or 853 |
| `ntp` | UDP to port 123 |
| `discovery` | mDNS (5353), LLMNR (5355), SSDP (1900), NetBIOS (137, 138) |
| `ipv6` | any IPv6 frame that is not the firmware's |
| `ethertype` | any frame neither IPv4 nor ARP: 802.1Q-tagged, LLDP, anything else |
| `ip-proto` | IPv4 that is not TCP or UDP, ICMP included |
| `mac` | a source that is not a locally administered unicast address, or the NIC's assigned address on a frame the firmware may not send |
| `arp` | an ARP sender address that is not the frame's source |
| `dhcp-id` | DHCP option 12, 60, 77, 81 or 97 (a hostname, vendor class, user class, FQDN or machine UUID), or a chaddr or client identifier (option 61) that is not type 1 and the frame's source, on any DHCP message, broadcast or not |
| `dhcp-opt` | any other DHCP option but 50, 53, 54, 55 and 57, or sname or file not empty (so option 52 cannot move options there) |
| `dhcp-dst` | DHCP to an address neither 255.255.255.255 nor a server that an OFFER or ACK in the ingress named, by its source or option 54 |
| `sni` | a TLS server name not on the allow-list |
| `http` | a plaintext HTTP Host not on the allow-list, to any address, or a request with no Host to an address and port the image does not pin |
| `peer` | a TCP or UDP address and port neither pinned nor a relay |

The allow-list is a committed file, `tools/nonos_egress/allowlist.json` (format `nonos-egress-allow/1`). It lists the names, and each pinned address with its port:

- Nym's directory API: `live.rs` and `fetch_at.rs`'s `HTTPS_PORT`, in `capsule_net_nym/src/directory_sync`;
- the Nym gateways it enters through, each with its port: `capsule_net_nym/src/state/bootstrap.rs` (it reaches no mix node itself);
- Anyone's directory authorities, each with its `dir_port`: `capsule_net_anon/src/directory/authority/list.rs`. These are marked as authorities.

`--check-allowlist` scrapes those sources (`scrape.py`) and exits 2 when they say something else, so a changed pin waits for a reviewer to commit the new list. The firmware's DHCP, from the NIC's address before the OS sends, is held to the three DHCP rules too; it is spared only the guest's other rules. Relays come from the ingress: each TCP stream an authority sent from its pinned address and port is joined, its HTTP body inflated when it is zlib, and each `r` line's address with its ORPort (and DirPort, when it has one) taken.

A capture that is damaged anywhere is unreadable (exit 2): a record cut short, longer than its original length or the snap length, or bytes left over after the last record. It is never judged on the part that parsed. A capture in which the guest sent nothing is `EMPTY` (exit 3). `--expect-silence` takes silence as the claim, for an image with no network. One with no DHCP, or no pinned peer reached, is `INCOMPLETE` (exit 4): the boot did not get far enough to show anything. Each peer is printed with a frame count and why it was allowed (`pinned` or `relay`), or `unchosen` when it fails. The DHCP parameter request list and every TLS server name and HTTP host are printed as well. `--json FILE` writes the whole report.

```sh
python3 tools/nonos-pcap-egress --self-test
python3 tools/nonos_egress/mutants.py
python3 tools/nonos-pcap-egress EGRESS --ingress INGRESS --json report.json
```

`--self-test` builds a clean boot, one capture per rule, and the evasions an image under test could try:

- DNS from QEMU's host address;
- anything but ARP, DHCP or neighbour discovery from the NIC's address before the OS, and the firmware's own DHCP with a vendor class, an option it may not send, or an off-network destination;
- DHCP to an address the guest picked, with data in option 224, or a renewal to a server the egress alone names, or one a NAK named;
- a consensus sent in the egress, or from an authority's address on another port;
- 802.1Q, LLDP, ICMP with a payload, and GRE.

It also covers the identity rules, a capture cut inside a record (2000 cuts, each unreadable), 2000 corrupted copies (each a verdict or unreadable, never an exception), the empty and incomplete minimums, and the allow-list against the sources. `mutants.py` undoes each rule in turn and requires the self-test to fail on every one: 26 mutants, all caught. The flake's `static-abi` check runs both (`tools/nix/checks.nix`).

`scripts/no_telemetry_qemu_capture.sh` is the boot form (`make
nonos-mk-no-telemetry-capture`, `mk/50-ci.mk`). With
`NONOS_NO_TELEMETRY_IMAGE=<nonos.img>` it boots that image headless through
`tools/nonos-qemu --capture` for `NONOS_NO_TELEMETRY_TIMEOUT` seconds (180 by
default), or until `NONOS_NO_TELEMETRY_EXPECT` is on the serial console. It
checks the allow-list against the sources first. A boot that stopped before
its time (`tools/nonos-qemu` exits 0 only for one still running at the
timeout, or one that printed the line expected) is `STOPPED`, exit 5,
whatever the capture says. Without an image it boots `make
nonos-mk-run-serial-net`, and `NONOS_NO_TELEMETRY_BOOT_CMD` replaces both. It
prints `PASS`, `FAIL` (exit 1), `GAP` (exit 2), `EMPTY` (3), `INCOMPLETE` (4)
or `STOPPED` (5), and writes `report.json` to `target/no-telemetry`.

`make nonos-mk-qemu-net-audit` checks the QEMU command lines instead, read from
five `make -n` runs of the serial boot targets (`default_cmd`,
`scripts/audit_qemu_network_mode.sh:30-43`). Its first check expects the default
serial boot to attach no network card, but `QEMU_NET_MODE` defaults to `nat`
(`mk/10-qemu.mk:35`), so by the code that check fails at this commit.

Only make runs the boot forms: the target `ci-soak` runs the capture and
`ci-security` the audit (`mk/50-ci.mk`). No GitHub workflow calls either target.
The results of a booted run are in [Privacy guarantees](privacy-guarantees.md).
