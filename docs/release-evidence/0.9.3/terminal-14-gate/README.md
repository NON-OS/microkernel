# Terminal 14: the gate boot, booted 26 October 2026

The F12 drop-down, the row-only repaint, crisp banner blocks, the dimmed
inactive pane and inactive tabs as pills, on main merged at b4ed0d9eee.
The app skeleton's `wants_rect` is in every app's image, so this is a gate
boot: the IOMMU lane, setup to the desktop.

## How it ran

- Image: `make dev-image` at b4ed0d9eee (qemu-dev, throwaway test keys).
- Machine: QEMU TCG, no KVM, `--lane iommu=intel` (VT-d translation on,
  serial `[VT-D] translation enabled`), 4 CPUs, 8 GiB, fresh disk, 1920x1080.
- Driven over QMP at 120 ms a key: setup on its defaults, Ctrl+Alt+T, two
  `echo` lines, `latency`, F12, F12, F12, Esc, `neofetch`, Ctrl+Shift+T,
  Ctrl+Shift+D, `ls`.

## What it showed

- `1-desktop-menubar-dock.png`: setup done, the desktop with its menu bar
  and dock.
- `2-latency-row-only.png`: `latency` under TCG: middle 5 ms, 95% 8 ms,
  worst 68 ms, where the whole repaint of every key gave 68, 86 and 106 ms
  (terminal-12-latency). The worst is the first key of a line.
- `3-f12-drop.png`: F12: the window drops to the top of the work area, under
  the menu bar, full width and 45% high, the prompt keeping focus.
- `4-drop-closed-esc.png`: F12 again and later Esc each put the window back;
  both shots match the window before the drop pixel for pixel but for the
  clock.
- `5-neofetch.png`: the emblem and facts at full width.
- `6-tab-pills.png`: Ctrl+Shift+T: the inactive tab `local` is a pill with
  an edge, its close mark inside it beside its label.
- `7-split-dimmed.png`: Ctrl+Shift+D: the inactive left pane dimmed, its
  banner drawn in solid blocks; `ls` in the right pane.
