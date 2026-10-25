# ksec: unit 1 (load and tool bounds) booted at c8a952663, 25 October 2026

S-13's load and tool-run bounds with ksec's four additions on top: open-with
starts only file openers, a broker naming itself is refused, the instance
proof, and no tool exempt (capsule_linux's install role holds StreamImport).

## How it ran

- Image: `make dev-image` at c8a952663 (qemu-dev, throwaway keys).
- Machine: QEMU TCG, 4 CPUs, `--lane iommu=intel`, `--tpm`, fresh disk.
- Driven with `tools/nonos-drive`: Enter through first-boot setup, Esc on the
  live-session offer, then the launches below. Two runs of the same image.

## What it showed

- `[IOMMU] intel-vt-d present, enforcing=1, unconfined grants=0`;
  `[SMP] answered=3 online=4`, `[SMP-PROOF] cpu_count=4 PASS`.
- `desktop.png`: the desktop with menu bar and dock (5908 colours).
- From the Terminal (caps `0xC100197D`, SpawnWindow held), `terminal_tools.png`:
  - `tokei --version`: `[SPAWN] name=tool.tokei caps=0x59`;
  - `qwen tiers`: `[SPAWN] name=tool.model-fetch caps=0x40000003D`, through
    the tool bound with no exemption; the catalogue printed.
- `install std_proof`, `terminal_install.png`: through the installer,
  `[SPAWN] name=std_proof caps=0x159`, the manifest's own set with its
  optional Debug; `[TERMINAL-INSTALL] load ok`; the proof's PASS lines.
- From the Launchpad, after the consent dialog, `launchpad_launch.png`:
  `[SPAWN] name=app.egui_proof caps=0x1819` through the shell and installer.
- No `[CAP-DENY]` line in either run: nothing legitimate is refused.

## Not shown here

- A refusal in a boot: no shipped capsule asks beyond its caps. The refusals
  are proven on the host (spawn_bound_proofs, installer_proofs,
  desktop_proofs, with mutants). The open-with refusal joins capsule_attack's
  suite next, so the selftest profile boots it.
- `ipc.call unanswered installer`: the Terminal and the shell give a load a
  40 ms first wait and then follow it (`install/follow.rs`), so a load that
  takes seconds logs this once. Routed as the next ksec item.
- `driver.virtio_net0`, `net.socks5`, `wallpaper`, `compositor`,
  `driver.virtio_gpu0` unanswered lines are other owners', as in gpu's run.
