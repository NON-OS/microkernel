# dead-sites booted at 58bd00add7, against main 722698393a

Both built as standard-dev by `tools/nonos-dev-image`'s steps in the dev
copy of the tree and sealed with throwaway keys; no key under `.keys/` was
read. QEMU only, under TCG. **Not hardware-verified.**

## Four boots, one flow (`flow.sh`)

Each: `--fresh --lane iommu=intel|amd --lane usb-tablet` (`qemu-*.txt`),
setup walked by keys, "Not now", the Text Editor opened from the dock, a
line typed, Ctrl+S to `/tmp/<boot>.txt`; the editor says `saved` and the
tab takes the name (`*-save.png`). Then QEMU quit over QMP.

| Boot | Image | IOMMU | Desktop first frame | File saved |
|---|---|---|---|---|
| ld-intel | 58bd00add7 | VT-d | yes | yes |
| ld-amd | 58bd00add7 | AMD-Vi | yes | yes |
| main-intel | 722698393a | VT-d | yes | yes |
| main-amd | 722698393a | AMD-Vi | yes | yes |

## Branch against main

`*.lines.txt` counts each boot's `[EXIT]`, `[DMA]`, `[VT-D]`, `[AMD-VI]`,
`[ACPI] platform` and SD-host lines (page counts and trailing numbers
dropped; serial lines interleave, so the lines are matched by token, not
by row). For each lane the branch's counts equal main's exactly: the only
`[EXIT]` lines are the three NIC drivers with no card, and no `[DMA]` line
is new.

## F-128 (the ACPI SD host reset)

No QEMU machine here publishes an ACPI SD host: the `sd` and `emmc` lanes
are `sdhci-pci` (PCI), and the aarch64 boots use a device tree. Neither
boot has an `[ACPI] platform … device=` line, so F-128 is host-tested

Requester ids are replaced by `<bdf>`.
