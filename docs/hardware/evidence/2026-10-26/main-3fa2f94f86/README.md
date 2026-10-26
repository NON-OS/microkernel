# main 3fa2f94f8 booted (allows df0b24d22 in it), against main 722698393a

Built as standard-dev by `tools/nonos-dev-image`'s steps in the dev copy
of the tree and sealed with throwaway keys; no key under `.keys/` was
read. QEMU only, under TCG. **Not hardware-verified.**

| Boot | IOMMU | Lanes | First frame | Seen |
|---|---|---|---|---|
| m3-intel | VT-d | usb-tablet, usb-kbd, PS/2 off | 9523 ms | setup, save, Settings, About, File Manager, lease |
| m3-amd | AMD-Vi | usb-tablet | 10452 ms | setup, save, lease |
| main-intel (722698393a) | VT-d | usb-tablet | 8878 ms | setup, save, lease |
| main-amd (722698393a) | AMD-Vi | usb-tablet | 9207 ms | setup, save, lease |

## USB keyboard and tablet (m3-intel)
The machine had no PS/2 controller: a scratch switch in the dev copy's
`tools/nonos_qemu` appended `,i8042=off` to `-machine` (qemu-m3-intel.txt).
So every key of first-boot setup and every key typed in the Text Editor
(`m3-intel`, Ctrl+S, `/tmp/m3-intel.txt`) reached the system through the
usb-kbd on xHCI only; the editor says `saved` (m3-intel-save.png). Every
click is an absolute event, which only the usb-tablet takes. Settings,
About and the File Manager opened from the dock (m3-intel-*.png).
QEMU's `input-send-event` with a device id aborts QEMU under
`-display none` (`Property 'qemu-fixed-text-console.device' not found`),
so the PS/2-off machine is how the keyboard path is pinned.

## Network
Both: `[NET-CORE] lease 10.0.2.15/24 gw 10.0.2.2`, `lease-status state=3`.

## Lines against main (lines-diff.txt, tok.py)
Matched by token (numbers dropped; serial lines interleave).
- amd: `[EXIT]` 6 = 6, `[DMA]` 6 = 6, `[CAP-DENY]` 0 = 0, `[IPC-DENY]` 0 = 0.
- intel: `[DMA]` 6 = 6, `[CAP-DENY]` 0 = 0, `[IPC-DENY]` 0 = 0. `[EXIT]`
  6 → 8: the two new lines are `driver.ps2_kbd0 status 2: no device
  present, not started`, the PS/2-off machine's, not the image's. The one
  `[DMA]` row that differs is main's grant line interleaved with an AHCI
  line; the grant count is 4 on both.
- The other `[EXIT]` lines on all four are the three absent NICs.

