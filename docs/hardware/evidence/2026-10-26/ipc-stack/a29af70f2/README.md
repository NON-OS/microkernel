# Selftest of a/ipc-servers a29af70f2, QEMU only: not hardware-verified

`tools/nonos-selftest --only capsule --iommu` at a29af70f2 (VT-d,
selftest profile, throwaway keys): rc=0, `attack suite: PASS`
(selftest-report.txt). The fix for ../f82c86279's failure: the kernel
keeps a capsule's printed lines out of its request inbox (3cac5a27f), and
the case is back to its f82c86279 form.

capsule.log:
- :788 `reply-twice refused: a second MkIpcReplyTo to one request ENOENT (2)`
- :789 `reply-twice refused: MkIpcReplyTo to a request still in the inbox ENOENT (2)`
- :790 `reply-twice refused: MkIpcReplyTo to a call that gave up ETIMEDOUT (110)`
- :867 `deputy-escape refused: app.install through the shell's OP_OPEN_WITH ENOENT (2)`

No reply-twice `[ATTACK-NOTE]`, no "passed over" line, and no `[ATTACK`
text anywhere but at the start of the suite's own lines.
The first attempt of this run died in the kernel compile on the
container's full disk (not the code); this is the rerun.
