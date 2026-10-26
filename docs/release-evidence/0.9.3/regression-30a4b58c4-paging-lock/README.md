# Regression on main 30a4b58c4: page-table lock waits at boot

Two 4-CPU, iommu=intel boots of the same standard-dev image of main
30a4b58c4, setup to the desktop, both `--fresh`:

- run1: 44 `unanswered` (attest 17, virtio_gpu0 11, compositor 7, xhci0 3)
  and 12 `[LOCK] ... waiting ms=1000` lines.
- run2: 1 `unanswered` (vfs_pool) and 80 `[LOCK]` lines.

Every waiter is at paging/manager/api/address_space.rs:46 or
mapping/map_page.rs:35. Every holder is at address_space.rs:22 or
mapping_in_asid.rs:34. The gates on 639663e1a, b0f7e93b7 and 6f0bbe716 had
0 `[LOCK]` lines and 0 `unanswered`. The kernel and driver commits in
between are 491a7e237 (wallpaper Debug) and 0448b98c3 (MkGetPid for any valid
capsule).

`uhci/serial.log`: the uhci lane on the same image. driver.uhci0 starts and
stays up (62097ede0 booted).
