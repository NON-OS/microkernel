# Checking the security claims yourself: A boot sends nothing unexpected

Part of [Checking the security claims yourself](checking-the-claims.md).

## A boot sends nothing unexpected

What a boot sent is read off the wire. A QEMU boot with a network card,
`QEMU_NET_MODE` set to `nat` or `hostfwd`, adds a `filter-dump` of that card
writing to `QEMU_NET_CAPTURE` (`mk/10-qemu.mk:124-143`). The mode defaults to
`nat` and the capture to `target/qemu-net.pcap`; `QEMU_NET_MODE=off` attaches no
card, and an empty `QEMU_NET_CAPTURE` boots without a capture
(`mk/10-qemu.mk:35-39`).

`tools/nonos-pcap-egress` reads a capture and judges every frame the guest
sent; the host's own frames, from QEMU's user-net MAC `52:55:0a:00:02:02`, are
not judged (`judge`, `tools/nonos_pcap_egress.py`). The run fails on any of
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

Every other TCP and UDP peer is listed with a frame count and marked `listed`
when it is not given with `--allow`. That is not a failure, because the Nym and
Anyone clients reach gateways and relays they learn at run time. The DHCP
parameter request list and every TLS server name and HTTP host are printed as
well. `--json FILE` writes the whole report. The exit status is 0 for a pass, 1
for a failure and 2 for a file that is not an Ethernet pcap.

```sh
python3 tools/nonos-pcap-egress --self-test
python3 tools/nonos-pcap-egress CAPTURE --allow-name validator.nymtech.net --json report.json
```

`--self-test` builds one capture per failure kind, plus a clean one with DHCP,
a pinned TLS name and an inbound reply, and checks each verdict. It then reads
3000 randomly damaged and truncated copies and requires each to end in a
verdict, with no exception. The flake's `static-abi` check runs it
(`tools/nix/checks.nix`).

`scripts/no_telemetry_qemu_capture.sh` is the boot form (`make
nonos-mk-no-telemetry-capture`, `mk/50-ci.mk`). With
`NONOS_NO_TELEMETRY_IMAGE=<nonos.img>` it boots that image headless through
`tools/nonos-qemu --capture` for `NONOS_NO_TELEMETRY_TIMEOUT` seconds (180 by
default). It then judges the capture with the bootstrap names and addresses the
capsules pin: `pinned.rs` and `live.rs` in `capsule_net_nym/src/directory_sync`,
and `capsule_net_anon/src/directory/authority/list.rs`. Without an image it boots
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
