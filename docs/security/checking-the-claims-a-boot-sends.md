# Checking the security claims yourself: A boot sends nothing unexpected

Part of [Checking the security claims yourself](checking-the-claims.md).

## A boot sends nothing unexpected

What a boot sent is read off the wire. A QEMU boot with a network card,
`QEMU_NET_MODE` set to `nat` or `hostfwd`, adds a `filter-dump` of that card
writing to `QEMU_NET_CAPTURE` (`mk/10-qemu.mk:124-143`). The mode defaults to
`nat` and the capture to `target/qemu-net.pcap`; `QEMU_NET_MODE=off` attaches no
card, and an empty `QEMU_NET_CAPTURE` boots without a capture
(`mk/10-qemu.mk:35-39`).

`tools/nonos-pcap-egress` reads a capture. It lists each IPv4 destination with a
packet count, marked `UNEXPECTED` unless given with `--allow`, then every DNS
name queried, and exits 1 when any destination is outside the allow list
(`dests` and `allow`, `tools/nonos-pcap-egress:50-70`). It reads IPv4 frames
only, so IPv6 and ARP pass unseen, and takes DNS names only from UDP packets to
port 53.

```sh
make nonos-mk-run-serial-nat
python3 tools/nonos-pcap-egress target/qemu-net.pcap --allow ADDRESS
```

Not tested in this release.

Each destination prints as `[egress] <address> <count> packets`, and each name
as `[egress] dns <name> x<count>`. On scratch captures, one holding only the
24-byte file header printed nothing and exited 0, and one packet to an address
outside `--allow` exited 1.

`make nonos-mk-no-telemetry-capture` is the strict form
(`no_telemetry_qemu_capture`, `mk/50-ci.mk:26-27`). It boots `make
QEMU_NET_CAPTURE=<pcap> nonos-mk-run-serial-net` for 120 seconds unless
`NONOS_NO_TELEMETRY_TIMEOUT` says otherwise, and passes only when the capture
holds no packet at all, so a DHCP request fails it as surely as any other packet
(`timeout_secs` and `bytes`, `scripts/no_telemetry_qemu_capture.sh:22-68`). It
prints `no-telemetry capture: PASS no packets captured`, or `FAIL captured <n>
bytes` and exits 1, or `GAP` and exits 2 when the boot failed or wrote no
capture. It writes `report.json` to `target/no-telemetry`.
`NONOS_NO_TELEMETRY_BOOT_CMD` replaces the boot command.

`make nonos-mk-qemu-net-audit` checks the QEMU command lines instead, read from
five `make -n` runs of the serial boot targets (`default_cmd`,
`scripts/audit_qemu_network_mode.sh:30-43`). Its first check expects the default
serial boot to attach no network card, but `QEMU_NET_MODE` defaults to `nat`
(`mk/10-qemu.mk:35`), so by the code that check fails at this commit.

Only make runs them: the target `ci-soak` runs the capture and `ci-security` the
audit (`mk/50-ci.mk`). No GitHub workflow calls either target, and the flake
runs neither. None was run for this release.
