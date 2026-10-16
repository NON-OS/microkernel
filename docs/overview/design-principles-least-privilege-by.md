# Design principles: Least privilege by capability word

Part of [Design principles](design-principles.md).

## Least privilege by capability word

Every process holds a [capability word](glossary-boot-root-record.md#capability-word), a mask
over the 37 capability bits the kernel defines (`capability_table` in
`src/capabilities/types/defs.rs:21-83`). A [capsule](glossary-boot-root-record.md#capsule)'s word
comes from its signed [manifest](glossary-linux-personality.md#manifest): its required bits, plus
those of its optional bits the spawning code allows (`install_caps` in
`src/security/capsule_manifest/verify/caps_bits.rs:38-47`). At spawn, a grant
outside the manifest is refused with `GrantOutsideManifest` (`check_grant` in
`src/security/capsule_manifest/verify/caps.rs:31-39`). Every syscall the kernel
knows then passes `dispatch`, which refuses with EPERM when the caller's
capability does not resolve for that call
(`src/syscall/contract/dispatch.rs:25-40`).

Services hold the same line. The file store answers only the kernel and holders
of FileSystem (`CAP_FILE_SYSTEM` in
`userland/capsule_vfs/src/server/fs_gate.rs:17-40`). The NVMe, AHCI and
virtio-blk drivers serve raw sectors only to the kernel's own client and to
holders of StoreWrite (`permits` in
`userland/capsule_driver_nvme/src/server/medium.rs:17-30`). Each asks the kernel
on every request instead of caching the answer, so a verdict never outlives the
process it was about.

The checks: `scripts/cap_audit.py --strict` refuses a capsule whose own code
makes a call its mask has no bit for, except the two capsules in `EXEMPT`,
`attack` and `toolkit` (`scripts/cap_audit.py:66-72`). It reports a granted bit
with no call behind it but does not refuse one. `scripts/check_mirror_caps.py`
holds each kernel spawn file and README to its manifest.
`nonos-ci/run-static-checks.sh` runs both inside the `static-tree` flake check.
On this commit the strict audit prints `107 capsules, 8 granted bits with no
evidence, 1 capsules missing a bit a call needs, 0 strict failures (attack,
toolkit exempt)`, and the capsule missing a bit is `toolkit`. The mirror check
reports no mask wider than its manifest. `static-tree` still fails on this
commit, on another rule. It refuses an import named `read` or `write` in a
listed set of capsules, since such names look like the Linux calls
(`forbidden_import_dirs` in `nonos-ci/run-static-checks.sh:4493-4512`), and the
AHCI driver's eMMC handler imports its own `read` and `write`
(`userland/capsule_driver_ahci/src/server/handlers/emmc/dispatch.rs:29`).

The limits: two bits, `IO` and `Hardware`, enforce nothing
(`src/capabilities/types/defs.rs:23-31`), and two more, `Keyring` and `Entropy`,
gate only kernel clients that nothing calls ([ABI
capabilities](../abi/capabilities-checks-outside-the.md#checks-outside-the-syscall-table)).
Exiting, yielding, futex waits, reading the clock and reading process stats take
no particular bit, only a valid [capability
token](glossary-boot-root-record.md#capability-token), which is one that holds at least one bit
(`check` in `src/syscall/contract/cap_table/mk.rs:23-38`). After spawn, a
process that holds Admin can add to another process any bit it holds itself,
whether or not that process's manifest declares it (`sys_cap_grant` in
`src/syscall/microkernel/capability/handlers.rs:33-52`).
