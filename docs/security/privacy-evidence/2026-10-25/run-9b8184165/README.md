# S-13: the load and tool-run bounds, booted

Two boots of a scratch image built with throwaway keys from privacy/s13-load-bound. Both runs used `s13boot.sh.txt`.

## The image

**Profile `s13proof`,** added in the scratch tree only:
- the kernel features `microkernel-full-gui` and `nonos-capsule-proof-io`, with nothing dropped, because init spawns proof_io only with capsule serial debug;
- capsule_proof_io's `main.rs` replaced by `scratch-proof_io-main.rs.txt`.

**The prober.** It is IPC and Memory only, never a launcher (`[SPAWN] name=proof_io … caps=0x118`). After 20 s it:
1. calls MkToolRun on `tool.install` and on `tool.tokei`;
2. sends the installer a load-by-name of `std_proof` with caps 0 (the manifest's own);
3. sends the same load with u64::MAX.

It expects EPERM each time, and exits 42 only if every step got it.

**QEMU:** 11.1.1, TCG, `-smp 4`, 6 GiB, `--tpm`, lanes `iommu=intel`, `usb-kbd` and `usb-tablet`. Setup was walked with the D-13 keys.

## Run 2: the head, `9b81841652` (`run2-head.txt`, image `d3c7726c…3ba4`)

`run2-serial.log`:
- **The four refusals:**
  - `[CAP-DENY] tool: caller pid=45 tool=tool.install missing caps 0x4008260`
  - `[CAP-DENY] tool: caller pid=45 tool=tool.tokei missing caps 0x40`
  - `[CAP-DENY] load: caller pid=57 parent pid=45 child=std_proof missing caps 0x40`: the installer (pid 57) loading for proof_io, bounded by proof_io.
  - `[CAP-DENY] load: caller pid=57 child=std_proof asked beyond its manifest 0xfffffffffffffea6`
- **The prober finished:** `[EXIT] code=42 proof_io`.
- **A launcher:** the Terminal's `install std_proof` (`run2-31-install-std-proof.png`) gave `[SPAWN] name=std_proof … caps=0x159`. That is the manifest's own set: required 0x59, plus the optional Debug that this serial-debug build grants. The Terminal named no caps.
- **The walk:** the Terminal, Files, the editor and Settings were opened and closed (`run2-3*.png`). Each `.1` instance logged `[MUNMAP] … total_pid=0x0`.
- **No faults:** `[CPU-PROT] … pge=1` on all four CPUs, and no FATAL, PANIC, #PF, #GP, `panicked` or ZEROSTATE line.
- **How it ended:** QEMU was stopped through QMP `quit` at 12:25:22 UTC, after the walk.

## Run 1: `6f6d7e4c5`, before the ambient fix (`run1-*`)

- The same four refusals, and proof_io exited 42.
- Its masks named CoreExec missing: `0x4008261`, `0x41`. CoreExec is ambient to every process, so the bound now counts the ambient set as held (930565f14), and run 2 shows `0x4008260` and `0x40`.
- The Terminal's `install std_proof` gave `caps=0x159`.
- QEMU was stopped through QMP `quit` at 11:14:49 UTC.

## Not shown here

