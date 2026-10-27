# The paging-lock fix, booted: c3f522575 with MkGetPid back

S1's fix for [the paging-lock waits at boot](../regression-30a4b58c4-paging-lock/README.md):
adb53a20d (the manager lock taken in turn, never by a switch or a system
call) and abae61bc9 (a guest's peer calls hold that guest's lock). The lead's
bisect put the waits on 0448b98c3, MkGetPid for every capsule, which main
reverted (a3c313619). These lanes boot the fix with 0448b98c3 put back, the
build that printed 44/12, 1/80 and 44/38 `unanswered`/`[LOCK]` lines before.

- Image: `tools/nonos-dev-image --profile standard` of c3f522575 plus a
  cherry-pick of 0448b98c3 (067a7b957 in a scratch tree), throwaway keys.
- QEMU TCG, no KVM, in a 4-vCPU cloud container that was also building and
  running host proofs; Intel IOMMU lane, fresh disk, setup to the desktop.

## mkgetpid-4cpu

`tools/nonos-drive --qemu-arg=--smp --qemu-arg=4 --qemu-arg=--lane
--qemu-arg=iommu=intel --qemu-arg=--fresh`, setup to the desktop, then the
Terminal.

- `serial.log`: `[IOMMU] intel-vt-d present, enforcing=1, unconfined
  grants=0`, `[SMP-PROOF] cpu_count=4 PASS`, 68 `[ZK-ATTEST] ok`, and
  **0 `[LOCK]` lines** (the same build printed 12 and 80 before the fix).
  No `[TRAP`, `[PANIC` or `[FATAL` line.
- 6 `[DIAG] ipc.call unanswered desktop_shell err=-110`, all in the minute
  a `market install linux.age` was fetching, while the host ran the
  kernel_proofs suite beside the VM. The install itself failed on the
  relay: its first fetch of the apk through the container's proxy outlasted
  the guest's wait (`gave no whole 200 reply`). Whether those six are the
  host's load or the image's is what the next lanes, on an idle host, say.
- `desktop.png`, `terminal.png`: the desktop, and the Terminal open on
  0.9.3 at 4m 48s.

Not here yet: the 8-CPU lane, and a run of Linux guests under the new peer
locks. The owner's local session runs the same lanes on its machine and
pushes them to `evidence/s1-lock`.
