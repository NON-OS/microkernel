# Checking the security claims yourself: The whole-image report and the authority map, continued

Part of [Checking the security claims yourself](checking-the-claims.md).

## The whole-image report and the authority map, continued

It reads each mask from `CAPSULE_REQUIRED_CAPS` in the capsule's `Capsule.mk`,
and marks a capsule `[not attested]` when its certificate, manifest or trailer
in the trust set is missing or empty (`Capsule`,
`tools/system_map_capsule.py:24-62`). Its tables count a capsule only when its
binary is built under `userland/<capsule>/target/x86_64-nonos-user/release/`;
`--tsv` lists every `Capsule.mk`, built or not. It does not decode the signed
manifest the spawn gate checks, so it shows the mask the build declares. It
knows all 36 capabilities (`CAPABILITIES`, `tools/system_map_consts.py:33-46`).
It is a report, not a gate: it exits 0 whenever it finds a `Capsule.mk` (`main`,
`tools/nonos_system_map.py:41-67`).
