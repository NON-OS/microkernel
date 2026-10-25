# ksec: booted at d2a9d0bfa, 25 October 2026

Five commits on main 3ae2beae9: the open-with attack case, a call to a dying
server answered ESRCH, publisher capsules' names under their namespace
(game_2048, gui_demo, mdview, qrgen), the silence rule, and a selftest image
with no first-boot setup.

## How it ran

- Images: `tools/nonos-selftest --only capsule --iommu` and `make dev-image`
  at d2a9d0bfa (selftest-dev and qemu-dev, throwaway keys).
- QEMU TCG, `--lane iommu=intel`, `--tpm`, fresh disk; qemu-dev on 4 CPUs,
  driven with `tools/nonos-drive` through setup to the desktop.

## What it showed

- Selftest: `attack suite: PASS`. `[ATTACK] deputy-escape refused:
  app.install through the shell's OP_OPEN_WITH ENOENT (2)` beside the shell's
  `[CAP-DENY] pid=56 open-with: app=app.install opens no file`; the row is
  proven (`selftest-deputy.txt`).
- qemu-dev: VT-d enforcing, 0 unconfined grants; `[SMP-PROOF] cpu_count=4 PASS`.
- From the Terminal (`terminal_installs.png`): tool.tokei (0x59),
  tool.model-fetch (0x40000003D), std_proof (0x159), and
  `[SPAWN] name=com.example.game_2048 caps=0x1819`, `load ok`.
- From the Launchpad, after consent (`game_2048_launchpad.png`):
  `com.example.game_2048` with its window drawn.
- No `[CAP-DENY]` in the qemu-dev runs.

## Late and unanswered lines, and why

- `unanswered installer`, once: the silence rule. The Terminal followed the
  game's 5.4 s load with 20 ms probes; none was answered in time for 5 s.
- `late net.dhcp.client` (65 to 88 ms), `late driver.virtio_net0` (10 ms to
  1.1 s), `late driver.virtio_gpu0`, `late wallpaper`, `late policy`: callers'
  own short polls missing under TCG; each answered in time often enough that
  no pair stayed silent 5 s. The late line itself goes quiet in the next
  ksec change (hw's ask); the silence rule stays the report.
- No `attest` line in either run; no driver exited in them.

## Seen, not fixed here

- A GUI app started from the Terminal (`install game_2048`) loads but shows
  no window and the command ends marked failed; from the Launchpad it draws.
  Routed as the next ksec item.
