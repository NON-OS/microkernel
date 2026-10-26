# Switch boot: Alt+Tab, and the two defects it found

One image, booted on 2026-10-26 under QEMU TCG: q35, 4 vCPUs, 8 GiB,
`--tpm --headless --fresh`, every setup default, "Not now" on the
live-session dialog. 1920x1080 only: the defects below sent the stack back
before the 1280x800 run, and the next image boots both sizes.

## The image

- **What it is.** 17abb88b7e, a local merge (not pushed) of ui/switch
  7e86e2bcb4 with ui/defects cc6b259900. ui/switch carries main df0b24d223,
  ui/type, ui/tokens, the focus event, the frame, all-edge resize with the
  held press (b08a77c0a1), and the cursor shapes.
- **How it was built.** `env -u NONOS_SIGNING_KEY tools/nonos-dev-image
  --profile qemu`, in the tool's own copy of the tree with the throwaway
  keys it makes there.
- **Serial.** `[DISPLAY] on driver.virtio_gpu0 1920x1080, canvas 1920x1080
  at scale 1.00`.

## What it shows (booted)

| Shot | Shows |
|---|---|
| `01-three-windows` | Settings, Files and About opened from the dock, About on top. Its uptime tile reads `since this boot` (ui/defects). |
| `02-alt-tab` | **Alt+Tab.** About's window stays where it was and Files comes up; the strip shows About, Files and Settings, Files ringed. About never saw the key. |
| `03-alt-tab-after` | 3 s later: the strip is gone, Files has the keyboard, and the menu bar and the dock name Files. |
| `04-two-presses-and-back` | Top: two presses walked on to Settings (Files, then About, then Settings), not back and forth; the first shot caught the strip before its second paint. Bottom: Shift+Alt+Tab went back to Files. |
| `05-shift-alt-tab-burst` | Shift+Alt+Tab from Files, shot at 0.0, 0.2, 0.5 and 0.9 s: the strip rings About, the last in the order, and is gone by the last shot. |
| `07-left-edge-resized` | About's left edge dragged from 598 to 478; its right edge stayed at 1577. |

## Found in this boot, and fixed (not yet booted)

- **The strip let a window's text through.** It was drawn on the shell's
  90% panel, and Files' text read between its tiles (`02`, `05`). Fixed on
  ui/switch a0abbf7da5: the strip draws on PANEL_SOLID. Proof:
  switch_strip_panel_is_opaque; 2 mutants.
- **No resize arrow ever showed** (`06`): the pointer was the arrow 5 px
  inside an edge that resized when dragged. The shell's chrome is a layer
  over the whole display, and the compositor took the shape of the topmost
  layer under the pointer, always the chrome's arrow. Fixed on ui/cursor
  cbb6796180: a layer has a say only once its owner asks, the chrome never
  does, and a window states its own arrow at its first hover. Proofs:
  cursor_overlay_tests (2), the_first_hover_tells_even_the_arrow; 2 mutants.

## Not settled in this boot

- **The held press** (b08a77c0a1): the drags here did not start on an edge
  at a sidebar row's height (the QMP pointer lands within a few pixels), so
  the boot neither shows nor rules out a row chosen by an edge press. It
  stays host-tested (drag_tests/held.rs) until the next boot.

## Cyan (`tools/nonos-ui-measure --budget 2`, exit 0)

`measure.txt`: every full-screen shot is under 2%; the highest is 0.23%.
