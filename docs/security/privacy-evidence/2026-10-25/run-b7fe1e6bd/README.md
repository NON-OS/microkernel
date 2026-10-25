# S-13b: the installer gated by MkCapsOf, booted

One scratch boot of privacy/s13b-installer-gate at **b7fe1e6bd** (`head.txt`). That head includes main through 87d670ef9, so the Terminal's sealed mask is 0xc100187d and it holds SpawnWindow: `[SPAWN] name=app.terminal … caps=0x00000000C100197D`.

## The image

- **Built with:** `tools/nonos-dev-image` with throwaway keys, image sha256 `137a46ab…24952`.
- **Profile `s13bproof`, scratch tree only** (`s13bboot.sh.txt`): the kernel features `microkernel-full-gui` and `nonos-capsule-proof-io`, with nothing dropped.
- **The prober:** capsule_proof_io's `main.rs` is `scratch-proof_io-main.rs.txt`. It holds IPC and Memory only (`caps=0x118`) and is no launcher. It sends the installer PKG_QUERY, PKG_COMMIT and PKG_REMOVE (`gui_demo`), and exits 42 only if all three come back EPERM.
- **QEMU:** 11.1.1, TCG, `-smp 4`, 6 GiB, `--tpm`, lanes `iommu=intel`, `usb-kbd` and `usb-tablet`. Setup was walked with the D-13 keys.

## What the serial log shows (`serial.log`)

- **The prober:** `[EXIT] code=42 proof_io` (line 1217). The installer refused all three package ops to a sender without SpawnWindow.
- **The Terminal, a launcher, asking on its own window thread** (output redirected, so the command is not run as a job):
  - `pkg install /home/nonos/readme.txt > /tmp/i.txt` answered in 46 ms with `pkg: malformed package or path` (`35-window-thread-install.png`). The installer admitted it, read the file and found no package: not an EPERM.
  - `pkg remove gui_demo > /tmp/r.txt` answered in 18 ms with `pkg: not installed` (`36-window-thread-remove.png`). It was admitted; gui_demo had already been removed by the job below.
- **The Terminal asking from a `pkg` job** (a worker thread, its own pid with the Terminal's mask):
  - `pkg remove game_2048` removed it: `ls /capsules` no longer lists it (`37-job-remove.png`).
  - The job itself got no reply and gave up after 30 s: `[DIAG] ipc.call unanswered installer caller=0x55` and "installer not ready".
  - Earlier jobs (`pkg remove gui_demo`, `pkg install …readme.txt`, pids 0x4E, 0x4F and 0x54) went the same way.
  - So the gate admitted the job and the op ran, but the reply did not reach the worker thread.
  - Not caused by S-13b's change: the installer's receive is the same syscall as before (`mk_ipc_recv_from_at` is `MkIpcRecvFrom` with its time pointer), its reply path is unchanged, and MkCapsOf touches no IPC. Not yet shown on a main image either; it is reported as its own finding.
- **One more unanswered line:** `caller=0x4D` (the Terminal's own pid), line 1794, falls between `capsule_spawn_start:std_proof` and std_proof's spawn, during `install std_proof`'s load, which then completed (std_proof spawned, `caps=0x159`). Not traced further.
- **The walk:**
  - the Terminal, Files, the editor and Settings were opened and closed, and each `.1` instance was torn down (`[MUNMAP] … total_pid=0x0`);
  - `[CPU-PROT] … pge=1` on all four CPUs;
  - no FATAL, PANIC, #PF, #GP, `panicked` or ZEROSTATE line.
- **How it ended:** QEMU was stopped through QMP `quit` at 15:56:57 UTC, after the walk.

## Not shown here

- **The shell's consent install.** The image carries no `.nonos` package (`tools/nix/store.json`, `packages` is empty), and making one needs a publisher's signature. It is host-tested: the shell holds SpawnWindow, and `installer_proofs` `gate_tests` admits a SpawnWindow sender.
