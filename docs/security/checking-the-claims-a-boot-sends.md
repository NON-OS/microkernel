# Checking the security claims yourself: A boot sends nothing unexpected

Part of [Checking the security claims yourself](checking-the-claims.md).

## A boot sends nothing unexpected

What a boot sent is read off the wire. A QEMU boot with a network card,
`QEMU_NET_MODE` set to `nat` or `hostfwd`, adds a `filter-dump` of that card
writing to `QEMU_NET_CAPTURE` (`mk/10-qemu.mk:124-143`). The mode defaults to
`nat` and the capture to `target/qemu-net.pcap`; `QEMU_NET_MODE=off` attaches no
card, and an empty `QEMU_NET_CAPTURE` boots without a capture
(`mk/10-qemu.mk:35-39`).

`tools/nonos-pcap-egress` reads a capture and judges every frame the guest sent (`judge`, `tools/nonos_pcap_egress.py`):

- The host's own frames are not judged. They come from QEMU's user-net MACs: `52:55:0a:00:02:02`, and `52:56:00:00:00:02` for IPv6.
- Frames from the NIC's assigned MAC (`--nic-mac`, QEMU's `52:54:00:12:34:56` by default) sent before the guest uses any other address are the UEFI firmware's, before NONOS runs. They are counted as firmware frames and not judged.
- NONOS replaces that address with a random one, so a frame from it after that fails as `mac`. The run fails on any of
these, each named in the output as `[egress] FAIL <kind>: <what>`:

| kind | what |
|---|---|
| `dns` | UDP or TCP to port 53 or 853 |
| `ntp` | UDP to port 123 |
| `discovery` | mDNS (5353), LLMNR (5355), SSDP (1900), NetBIOS (137, 138) |
| `ipv6` | any IPv6 frame |
| `mac` | a source MAC that is not a locally administered unicast address |
| `dhcp-id` | DHCP client option 12, 60, 77 or 81: a hostname, vendor class, user class or FQDN |
| `sni` | a TLS ClientHello whose server name is not given with `--allow-name` |
| `http` | a plaintext HTTP request whose Host is not given with `--allow-name`, to a peer not given with `--allow` |
| `peer` | a TCP or UDP peer the image did not choose: not given with `--allow`, and not a relay named by a consensus that a peer given with `--allow` sent in the same capture |

The relays come from the capture itself (`relays`, `tools/nonos_pcap_egress.py`). The tool joins every reply that a peer given with `--allow` sent on a TCP stream, inflates the HTTP body when it is zlib, and takes the address on each `r` line, which is how an onion network's consensus names its relays. Only an allowed authority's consensus can add relays, so a host nobody pinned cannot vouch for itself. Each peer is printed with a frame count and why it was allowed (`pinned` or `relay`), or `unchosen` when it fails. The DHCP parameter request list and every TLS server name and HTTP host are printed as well. `--json FILE` writes the whole report. A capture in which the guest sent nothing is `EMPTY`, not a pass: it shows that no path was used, not that one was clean, which is what a NIC whose driver never carried traffic looks like. `--expect-silence` takes silence as the claim, for an image with no network. The exit status is 0 for a pass, 1
for a failure, 2 for a file that is not an Ethernet pcap and 3 for an empty capture.

```sh
python3 tools/nonos-pcap-egress --self-test
python3 tools/nonos-pcap-egress CAPTURE --allow-name validator.nymtech.net --json report.json
```

`--self-test` builds one capture per failure kind, plus a clean one with DHCP, a pinned gateway, a pinned TLS name, a consensus from a pinned authority and a relay it names. It checks each verdict, that a host's own consensus does not vouch for it, and that a capture with no guest frame is empty. It then reads
3000 randomly damaged and truncated copies and requires each to end in a
verdict, with no exception. The flake's `static-abi` check runs it
(`tools/nix/checks.nix`).

`scripts/no_telemetry_qemu_capture.sh` is the boot form (`make
nonos-mk-no-telemetry-capture`, `mk/50-ci.mk`). With
`NONOS_NO_TELEMETRY_IMAGE=<nonos.img>` it boots that image headless through
`tools/nonos-qemu --capture` for `NONOS_NO_TELEMETRY_TIMEOUT` seconds (180 by
default). It then judges the capture with the bootstrap names and addresses the
capsules pin:

- Nym's directory API: `pinned.rs` and `live.rs` in `capsule_net_nym/src/directory_sync`;
- the Nym gateways it enters through: `capsule_net_nym/src/state/bootstrap.rs` (it reaches no mix node itself);
- Anyone's directory authorities: `capsule_net_anon/src/directory/authority/list.rs`. Without an image it boots
`make nonos-mk-run-serial-net`, and `NONOS_NO_TELEMETRY_BOOT_CMD` replaces both.
It prints `PASS`, `FAIL` (exit 1) or `GAP` (exit 2, no capture to judge) and
writes `report.json` to `target/no-telemetry`. Its earlier form passed only on
an empty capture. A networked boot always sends DHCP and the anonymity networks'
bootstrap, so that form could pass only when the network never came up.

`make nonos-mk-qemu-net-audit` checks the QEMU command lines instead, read from
five `make -n` runs of the serial boot targets (`default_cmd`,
`scripts/audit_qemu_network_mode.sh:30-43`). Its first check expects the default
serial boot to attach no network card, but `QEMU_NET_MODE` defaults to `nat`
(`mk/10-qemu.mk:35`), so by the code that check fails at this commit.

Only make runs the boot forms: the target `ci-soak` runs the capture and
`ci-security` the audit (`mk/50-ci.mk`). No GitHub workflow calls either target.
The results of a booted run are in [Privacy guarantees](privacy-guarantees.md).
