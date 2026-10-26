# ksec: the browser's address bar and the 2048 palette, booted at 1d5ef69e84, 26 October 2026

- Image: `make dev-image` at 1d5ef69e84 (qemu-dev, throwaway keys); QEMU TCG, 4 CPUs,
  `--lane iommu=intel`, `--tpm`, fresh disk, 1920x1080, driven by `tools/nonos-drive`.
- 2048 after 48 moves: every tile's digits in the ground's ink, 2 to 64, readable
  (game_2048_palette_1080p.png; WCAG AA host-proved in demo_screen_proofs).
- `file:///notes.txt` typed in the bar: gone to, not searched, and refused on its
  page as "This browser opens web addresses, http and https, only; that address is
  not one." (browser_file_scheme_1080p.png).
- `http://10.0.2.2:9/notes.txt` typed: the bar holds the address and the load is
  under way (the stop button); the last page stays until the new one commits
  (browser_http_address_1080p.png). Nym does not connect in this sandbox.
- No `ipc.call unanswered` line in the serial log.
