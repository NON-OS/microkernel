# Checking the security claims yourself: The whole-image report and the authority map

Part of [Checking the security claims yourself](checking-the-claims.md).

## The whole-image report and the authority map

`tools/nonos_console.py` prints counts and checks over the tree and the built
image, one section at a time, in the order `SECTIONS` gives
(`tools/nonos_console.py:1532-1547`). Its counts are lines that contain a string
under `src/`, found in pure Python by its `rg` helper, which skips `target`
directories (`tools/nonos_console.py:125-141`).

```sh
python3 tools/nonos_console.py --fast safety
```

It reported 0 lines with `unwrap()`, `expect()` or `panic!()` under `src/`,
1,498 lines with `unsafe {` and 653 with `SAFETY:`, and exited 0.

Read its numbers as text counts. Its `syscalls` figure, 236 here, counts every
line holding `tag4(b"` under `src/syscall`, not the published list
(`section_syscalls`, `tools/nonos_console.py:1297-1303`). Its `tcb` figure sums
the lines of a fixed list of kernel directories, every architecture included
(`trusted`, `tools/nonos_console.py:509-512`), which is a different measure from
`nonos-tcb`. Its capability names stop at bit 31 (`CAPABILITIES`,
`tools/nonos_console.py:57-68`), so bits 32 to 35 print as a raw `bit:0x` value.

Its `attack` section copies an attested capsule's manifest to a scratch
directory and runs `capsule-sign verify-manifest` on it. The untouched copy must
be accepted; a flipped byte, another capsule's certificate and a manifest
granting `ProcessControl` must be refused; it also checks a trailer's magic and
looks for a foreign-architecture ELF (`section_attack`,
`tools/nonos_console.py:1450-1527`). A case that is not refused prints `NOT
BLOCKED`, and the console exits 1 (`FAILURES`,
`tools/nonos_console.py:1577-1583`). Two things make that section unreliable at
this commit:

- Without `nonos-sign/target/release/capsule-sign`, `verify_manifest` returns
  `None` (`tools/nonos_console.py:1425-1427`). The control line still reads
  `ACCEPTED`, and every case that calls the signing tool reads `NOT BLOCKED`.
  Build the signing tool first.
- The trailer line compares a trailer's first eight bytes with `TRAILER_MAGIC`,
  `NZKPATH1` (`tools/nonos_console.py:1508-1511`). A release kernel takes only a
  v4 [attestation
  trailer](../overview/glossary-amnesic-boot.md#attestation-trailer), which
  starts with `MAGIC_V4` (`parse_v4`, `nonos-attest-path/src/v4/parse.rs:33`); a
  bare `NZKPATH1` path is taken only by a [development
  image](../overview/glossary-claim-epoch.md#development-image)'s kernel
  (`dev_path`, `src/security/capsule_attest/path.rs:41-45`). Against a sealed
  trailer that line reads `NOT BLOCKED`.

`tools/nonos_system_map.py` prints the authority each built capsule holds: an
image summary, one row per capsule with its size, port and capabilities, how
many capsules hold each capability, and the holders of the twelve capabilities
it calls scarce, which it describes as acting on something the holder does not
own (`SCARCE`, `tools/system_map_consts.py:52-56`).

```sh
python3 tools/nonos_system_map.py --fast
python3 tools/nonos_system_map.py --rare
python3 tools/nonos_system_map.py --tsv
```

Not tested in this release.
