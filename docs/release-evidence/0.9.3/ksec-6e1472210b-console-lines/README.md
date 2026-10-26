# ksec: kernel console lines whole across CPUs, booted at 6e1472210b, 26 October 2026

- Image: `make dev-image` at 6e1472210b (qemu-dev, throwaway keys); QEMU TCG, 4 CPUs,
  `--tpm`, fresh disk, driven through setup to the Terminal and three app installs.
- `[SMP-PROOF] cpu_count=4 PASS`; every `[USER-FIRST]` line whole and at column 0,
  64 of 64 (the boot before this fix had 5 of 64 behind another writer's text);
  `[INIT]`, `[DIAG]` and `[BENCH]` lines never inside another line; no unanswered line.
- One kernel line still landed after a capsule's unfinished `[BLK]` text:
  `[SPAWN]`, printed in pieces by process_spawn's spawn_log.rs. It is converted
  in the next commit, host-proved by kernel_proofs serial_line; 47 kernel files
  still print some line in pieces and are listed for a sweep.
- serial_extract.txt: the [USER-FIRST] and [SMP-PROOF] lines of this boot.
