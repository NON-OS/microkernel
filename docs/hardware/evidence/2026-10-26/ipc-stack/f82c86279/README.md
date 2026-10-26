# Selftest of a/ipc-servers f82c86279, QEMU only: not hardware-verified

`tools/nonos-selftest --only capsule --iommu` at f82c86279 (VT-d, selftest
profile, throwaway keys). rc=1, `attack suite: FAIL`. The report fails:
- reply-twice: never attempted, so all three reply rows (a second reply
  ENOENT, a reply to a request still queued ENOENT, a reply to a call that
  gave up ETIMEDOUT) have no line;
- deputy: `app.install through the shell's OP_OPEN_WITH` refused ETIMEDOUT,
  not the ENOENT the matrix row wants (the earlier run at 0c58a2a4a had
  ENOENT; see ../selftest-capsule.log).

reply-twice-lines.txt holds every reply-twice and deputy line. The case
passed over 32 messages, every one `8 bytes first byte 91 sender 56
request 0`: from itself (test.attack is pid 0x38 = 56), at least 8 bytes,
starting `[` (0x5b), and not a request (id 0). It then said `its own
requests were not in its inbox`: `take` reads at most 16 messages per
body, and the 32 plain messages came ahead of the queued calls.
