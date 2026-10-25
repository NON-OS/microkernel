# P-06: user pages never GLOBAL, the desktop boot

Booted for Platform's P-06 at the request. The question: do user mappings
still work after the change?

## The image

- **Head:** platform/review-user-not-global `c6ad70599`, which has main
  `2f5997b88` merged in (`head.txt`).
- **Image:** `tools/nonos-dev-image --profile standard` with throwaway keys,
  sha256 `50b276d8…eb59e2`.
- **QEMU:** 11.1.1, q35, TCG, `-smp 4`, 6 GiB, `--tpm`, lanes `iommu=intel`,
  `usb-kbd` and `usb-tablet`. The full command line is in `qemu-run.txt`.

## What was done

1. Setup was walked with the D-13 keys.
2. The live-session dialog was dismissed.
3. Each app was opened from the dock and closed with its window's close
   button. Every step has a screendump.
   - **Terminal:** opened (`30`), `ls` run, closed (`32`).
   - **Files:** opened (`33`), closed.
   - **Text editor:** opened (`35`), `hello` typed, closed. It asked to
     close again to discard, then closed (`39`).
   - **Settings:** opened (`37`), closed.
   - **Terminal again:** opened, `tokei` run as a tool through MkToolRun
     (`41`), closed (`42`).

## How it ended

QEMU was stopped through QMP `quit` at 10:04:28 UTC, after the last close.
It was a live boot with nothing to keep. QEMU did not exit by itself, and it
was not asked to.

## What the serial log shows (`serial.log`)

- **`[CPU-PROT]`:** `pge=1` on all four CPUs (cpu 0 has no `cpu=` field), with
  smep, smap, umip, nx and wp all 1.
- **No faults:** no FATAL, PANIC, #PF, #GP, `panicked` or ZEROSTATE line.
- **Spawns:** 73 `[SPAWN] name=` lines. The first are ramfs, keyring,
  entropy_pool, crypto_pool, policy, input_router, compositor,
  app.boot_splash, then the virtio drivers.
- **Apps opened from the dock:** each came up as a new instance
  (`app.terminal.1`, `app.file_manager.1`, `app.text_editor.1`,
  `app.settings.1`, then `app.terminal.1` again), and `tool.tokei` came up
  through MkToolRun.
- **Teardown:** each close is followed by a `[MUNMAP]` line for that instance
  ending in `total_pid=0x0`, so the process's user mappings were all released.
- **tool.tokei exited with code 1:** its own error, `'.' not found` (`41`).
  That's the tool itself, not a fault.
- **Capsules that did not start:** five drivers left with
  `status 2: no device present` (i2c_pci0, e1000_0, rtl8139_0, rtl8169_0,
  i2c_hid0), as on every QEMU boot. No other capsule failed.
- **Already on main:** `[MUNMAP] release_va_failed` for the compositor
  (`serial.log:387`). The same line is in the walk, P-05, egress, amnesic and
  bga serial logs from main-based images.
