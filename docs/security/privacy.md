# Privacy: every way off the machine

NONOS sends nothing about you anywhere: there is no telemetry, no crash report,
no update check and no account. This page and its parts list every capsule that
can reach the network and every place the image's code names, with why it is
there and how to turn it off. A check holds the list to the code, so a new way
off the machine cannot land without a row here.

## Parts

- [The capsules that hold Network](privacy-capsules.md)
- [What leaves without your asking](privacy-unprompted.md)
- [What leaves only when you ask](privacy-on-request.md)
- [Names in the code that dial nothing](privacy-inert.md)
- [The threat model](privacy-threat-model.md)

What each promise has been shown to hold, booted or on the host, is in
[Privacy guarantees](privacy-guarantees.md); the packets a booted image sends
are judged by `tools/nonos-pcap-egress`.

## In short

With the network on, a NONOS desktop sends these on its own, before you open
anything:

1. A DHCP request for an address, from a random hardware address made this boot.
2. The Nym mixnet client's directory fetch, its gateway connection and its first
   mix routes, to addresses fixed in the image.
3. The Anyone client's consensus fetch from its directory authorities, then the
   circuits it builds.

Nothing else leaves until you open a program that connects. None of these
carries a name, an account, a serial number or anything kept from an earlier
boot: the hardware address is drawn fresh each boot, DHCP sends no host name and
names the machine only by that address, and both anonymity clients make new
identities each boot.

What your local network learns: that this machine runs NONOS's Nym and Anyone
clients. They start under every [network choice](../using/privacy-network.md),
Direct included.

## Turn it all off

Pick `Air-Gapped` in the boot menu. That boot takes the Network capability from
every capsule, so none of them can reach a network service, and nothing above is
sent (`profile_gate.rs` in `src/kernel_core/process_spawn/capsule_spawn/runner/`).
`Safe Mode` and `Recovery` do the same. The `airgapped` build profile compiles
no network driver or stack at all (`nonos.toml`).

## How this list is checked

`python3 tools/nonos-egress-audit` reads each `userland/*/Capsule.mk` for the
Network bit, follows every crate those capsules build from, and finds each host
name, URL, four-byte IPv4 address and build-time endpoint setting in their
source and in `tools/nonos_qwen_tier`. It fails on:

- a capsule that holds Network and is not in the capsule table;
- a place in the code that no table here lists;
- a row here that the code no longer has.

`nonos-ci/run-static-checks.sh` runs it on every change. The tables are read by
their first column; the rest of each row is for people.

What the check cannot see: an address put together at run time from parts, or
one a capsule is handed as data. The places that do this are named in their
rows: the browser's pages, the Nym and Anyone directories, and the model
catalogue.

## See also

- [Privacy networks: Nym, Anyone and Direct](../using/privacy-network.md)
- [What NONOS protects against](protections-and-limits.md)
