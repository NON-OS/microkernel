# the release sealed image, 26 October 2026: NO-GO

The owner booted a release-sealed image on his own machine (QEMU, 8 CPUs, virtio-gpu, about
4 GiB). He sent an excerpt of the serial log, not the whole log. The desktop came up and
every app spawned and attested `vendor`. Then three failures followed, and the machine
wiped and powered off.

## The sequence

1. The Terminal installed `linux.git` from the market: git and 14 dependencies were fetched,
   and each logged `provenance Verified`. Each also logged
   `unserved persist: install kept in RAM, store at rest unencrypted`.
2. `app.install` (pid 0x4F) spawned and mapped 0x1AC00000 (428 MiB). Then all eight CPUs
   reported a lock wait:
   `[LOCK] cpu=N waiting ms=1000 ... at src/memory/paging/manager/api/address_space.rs:45`,
   `held from .../address_space.rs:41`.
3. The uninstall (`app.linux.install`, pid 0x50) logged, for each dependency,
   `pack /linux-packs/<name>.pack not served: its index is not this machine's`, then
   `uninstall failed` and `[EXIT] code=24 app.linux.install`.
4. `app.prove.job` (pid 0x51) mapped 0x40000000 (1 GiB). Then a 720-byte kernel heap
   allocation failed:
   `[OOM] ALLOCATION FAILED`, `frame_allocated=0xE617E`, `heap_current=0x3D5022A`,
   `to the panic path: wipe, power off`.

## The mapping trail (from the OOM dump)

| pid | what | mapped | system total after |
|---|---|---|---|
| 0x4E | app.linux.install (install) | 0x14000000 | 0x8A292A48 |
| 0x4F | app.install | 0x1AC00000 | 0xA4E92A48 |
| 0x50 | app.linux.install (uninstall) | 0x04000000 | 0xA8E92A48 |
| 0x51 | app.prove.job | 0x40000000 | 0xE8E92A48 |

The total never fell after 0x4E and 0x4F ended. Allocated frames (0xE617E, 3681 MiB) track
it (3726 MiB).

## Defects and owners

- The kernel heap is starved by one app's mapping, and the panic follows. Owner: 2B.
- An exiting process's mappings stay counted, and its frames are kept. Owner: 2B.
- The paging manager lock is held through a dead process's teardown, so every CPU waits.
  Owner: 2B.
- The Linux store's uninstall can't read packs it wrote on an unserved-persist boot.
  Owner: 2D.
- Why persist was unserved, and whether the store is ever at rest unencrypted. Owner: 2E.

Each fix is booted at `-smp 8 -m 4096` with this sequence, or with the selftest probe where
the market is out of reach, before 0.9.3 can be GO.
