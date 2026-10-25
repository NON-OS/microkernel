# ksec: a GUI app from the Terminal, booted at 74779a915, 25 October 2026

## How it ran

- Image: `make dev-image` at 74779a915 (qemu-dev, throwaway keys).
- QEMU TCG, 4 CPUs, `--lane iommu=intel`, `--tpm`, fresh disk, driven
  with `tools/nonos-drive` through setup to the desktop, then the Terminal.

## What it showed

- VT-d enforcing, 0 unconfined grants; `[SMP-PROOF] cpu_count=4 PASS`.
- `install game_2048` from the Terminal: `[SPAWN] name=com.example.game_2048
  caps=0x1819`, `load ok`, the line "it runs in its own window", the
  command marked done, and the 2048 window drawn (`terminal_game_2048.png`).
  Before: the app waited for the shell's focus frame, which only the
  Launchpad sent, and the Terminal marked it failed after 5 s.
- `tokei --version` and `install std_proof` still drain their output and
  finish as before (std_proof holds no display right, so no window is asked).
- No late line: a late call alone prints nothing now. One
  `ipc.call unanswered installer silent 5 s, late replies dropped=4`: the
  Terminal's 20 ms probes during std_proof's 5 s load, none answered in time.
- No `[CAP-DENY]`.
