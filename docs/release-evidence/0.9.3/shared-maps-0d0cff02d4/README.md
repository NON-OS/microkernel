# A stored file mapped shared, as LMDB maps one: 0d0cff02d4

privacy enhancing features will be inactive" on the sealed b434cdf175. Its
ring database is LMDB, which maps its files MAP_SHARED, and the Linux
personality answered every shared file map with ENOSYS. This boot shows the
fix, shared-maps, working end to end on a development image.

- **Image:** `tools/nonos-dev-image --profile standard` of land-d at
  0d0cff02d4: main with shared-maps (373e35a4d5, cd4f3b67c3, 621098e269,
  f7e6f27d77) and the standard tail (0d0cff02d4). Throwaway keys, sha256
  beginning `778695db682542df`.
- **Machine:** QEMU TCG, 4 vCPUs, Intel IOMMU lane, fresh disk, setup to the
  desktop, then the Terminal.
- **Steps:** `coherence.steps`, here as run. The perl one-liner calls the
  syscalls by number: mmap 9, munmap 11, msync 26, pread64 17, pwrite64 18,
  read 0.

## What the program got

| Line (`coherence.png`) | What it shows |
|---|---|
| `P ro ok` | an 8 KiB file mapped PROT_READ, MAP_SHARED over 16 KiB, as LMDB maps `data.mdb`: served (0.9.3 before: ENOSYS) |
| `P seen HELLO BEYOND` | a pwrite at 100, and one at 9000, past the file's end, read back through the map at once, as LMDB reads its own writes |
| `P over refused Function not implemented` | a writable map of a range a read map already holds is refused: one writer per file range, the rule every other guarantee rests on |
| `P rw ok` | the read map unmapped, a PROT_READ\|PROT_WRITE MAP_SHARED map, as LMDB maps `lock.mdb` |
| `P read WORLD` | bytes put into the writable map (by a read from a pipe) seen by a pread of the file: the map's change was written back before the read |
| `P msync 0 unmap 0` | msync with MS_SYNC and munmap answer 0 |
| `P exec refused Permission denied` | a PROT_READ\|PROT_EXEC shared map of the file is refused: a file that can change after it is proved never runs (W^X) |
| `[LINUX] process 2 exited, status 0` | the run ended cleanly |

A second run of perl, a new process (`next-run.png`), reads the file:
`P next_run HELLO WORLD BEYOND size 9006`. Every write, through the file and
through the map, reached the store and outlived the process that made it.

## The serial log

- 0 `[LOCK]` lines; no `[TRAP`, `[PANIC` or `panicked` line.
- 2 `[DIAG] ipc.call unanswered`, on a 4-vCPU virtual machine building and
  running static checks beside the VM.
- None of the program's words (HELLO, WORLD, BEYOND) is on the serial.

## Not shown here

- Monero itself: its ring database set in one run of the wallet and printed
  in the next (`tools/nonos_s1_lanes/monero.steps`). The install needs a
- The host proofs are in `capsule_linux_proofs` (tests/shared_maps, 28
  tests including a model of three processes on two files checked against a
  Linux page cache, and tests/whole), with seven mutants and one more that
  fail.
