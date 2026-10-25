# ksec: windows that hold their apps, refusals with reasons, booted at 665d9ca13d, 25 October 2026

## How it ran

- Image: `make dev-image` at 665d9ca13d (qemu-dev, throwaway keys).
- Boot 1: QEMU TCG, 4 CPUs, `--lane iommu=intel`, `--tpm`, fresh disk, 1920x1080,
  driven with `tools/nonos-drive` through setup, then the Terminal by Ctrl+Alt+T.
- Boot 2: the same at 1024x768 (`--lane gpu=1024x768`, added here).

## What it showed

- VT-d enforcing, 0 unconfined grants; `[SMP-PROOF] cpu_count=4 PASS`.
- `install gui_demo`, `install egui_proof`, `install game_2048` from the Terminal:
  each window whole, the hint lines, the egui verdict and all four rows of the
  2048 board inside the frame (game_2048_1080p.png, gui_demo_1080p.png,
  egui_proof_1080p.png), and at 1024x768 (game_2048_1024x768.png).
- `install game_2048` again while it runs: "loading game_2048..." then
  "install rejected: already running, close it first", the reason taken from
  the installer's kept outcome after the Terminal's short wait;
  `install nosuch`: "install rejected: malformed manifest or artifacts"
  (refused_installs_1080p.png).
- No `ipc.call unanswered` line in either serial log: not for the installer
  during std_proof's or any load, not for attest.

## Not shown by these boots

- The Launchpad's refusal toast (host-proved, desktop_proofs launch_tests).
- hello, prove, gui_proof, clock, file_manager help and the browser's text
  pages: not in the dev store or not opened; host-proved by
  demo_screen_proofs app_windows and app_census.
