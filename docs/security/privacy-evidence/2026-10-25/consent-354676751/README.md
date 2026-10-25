# A stale consent erased on a live stick, booted

The image is the standard profile's development twin at 354676751, built
with test keys (sha256 `790ba4fe3455a371…`). It was booted as a live stick
under QEMU (TCG, `--tpm`, the `iommu=intel`, `usb-kbd` and `usb-tablet`
lanes) and driven by `tools/nonos_zero_state/steps/consent-{a,b,c}.steps`.

## The runs

1. **Boot A.** Setup in Install mode, which keeps, with "Also software
   installed here" (`a11-software-allowed.png`). The stick then held the
   consent token `/nonos/consent/local.token` (32 bytes), the answers, the
   setup marker and the settings record.
2. **Between boots.** On a copy of that stick, the host erased only the
   setup marker (`tools/nonos_zero_state/store_rows.py`, `erase`). The token
   and the answers stayed, so the next boot runs setup again with a
   consent an earlier boot kept.
3. **Boot B.** Setup ran in Amnesic, which keeps nothing, and found local
   software allowed (`b11-software-was-allowed.png`). It was set to "Only
   NONOS software" (`b11-software-only-nonos.png`), and the boot reached the
   desktop (`b20-desktop.png`). The serial log shows `[ZS] store erase on a
   boot that keeps nothing` twice, once for the payload and once for the
   descriptor.
4. **Boot C.** Setup again, from the stick boot B left. At the
   installed-software step it found no token: "Only NONOS software"
   (`c11-software-found.png`).

## The stick, judged on the host

| | Token row | Token sector | Token bytes anywhere on the stick |
|---|---|---|---|
| After boot A | present, 32 bytes | the token | present |
| After boot B | gone | all zero | 0 copies |

The answers (139 bytes) and the settings record (129 bytes) were left as
they were: boot B withdrew only what the person revoked.

`serial-extract.txt` holds every `[ZS]` and policy line of the three
boots, and the store rows of each stick.

## What it does not show

- One run, under TCG, on one virtual machine; real hardware is not covered.
- Boot C has one `ipc.call late policy after 152 ms` line, input_router's
  150 ms ask, while the policy service was blocked in its own call to vfs.
  It is not part of the erase. It is the remaining policy lateness, reported
  separately.
