# IPC stack boot (a/ipc-boot-cases), QEMU only: not hardware-verified

Head booted: 38a7d6247 merged with main as 0c58a2a4a. The branch head is now
7ec51e61e, which the Lead states is behaviour-equivalent; it was not booted.
Lane: standard-dev, throwaway dev keys, `iommu=intel`, usb-tablet
(qemu-ipc-intel.txt). Main for comparison: main-intel.serial.log.

## standard-dev, booted (QEMU)
- Desktop: `[DISPLAY] first frame let through at 7774 ms`.
- Round trips: `[POLICY] capsule spawned`, `[VFS] serving, store status 00`,
  `[STORE] the loader's copy of the store`, `[VFSD] packages staged`, and
  77 `[NYM]` lines; the same tag set and counts as main.
- File save: the Text Editor saved /tmp/ipc-intel.txt (ipc-intel-save.png).
- `[EXIT]`: 6 lines, all the three absent NICs (e1000, rtl8139, rtl8169,
  status 2, no device present); identical to main.
- `[IPC*]`: neither boot prints one (the kernel's only IPC tag is
  `[IPC-DENY]`), so the main-vs-branch diff is 0 against 0.

## selftest (`--only capsule --iommu`), booted (QEMU): FAIL
selftest-capsule.log, selftest-report.txt. Every other attack row is proven
(cap-escape, held-escape, foreign-memory, service-squat, fault containment,
deputy). The reply rows were never attempted:
`[ATTACK-NOTE] reply-twice: its own requests were not in its inbox`.
reply_twice.rs makes three `mk_ipc_call_timeout(4954, …, 50)` self-calls
(each ETIMEDOUT), then `take()` loops 16 × `mk_ipc_recv_req(0, …, NO_WAIT)`
wanting `rc == 1`, `sender == me`, `request != 0`; none matched. So neither
"refused … ENOENT" line is present and the matrix rows fail. No
test.attack timeout line appears apart from those three self-calls.
BDFs are redacted to `<bdf>`.
