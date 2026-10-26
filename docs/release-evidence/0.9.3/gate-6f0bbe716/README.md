# Gate boot of main 6f0bbe716, 26 October 2026

the third boot gate for 0.9.3. Main at 6f0bbe716 carries ksec's
whole-line console sweep (224e8aa35), the xhci0 scan window (3be5f780a),
L81b part B (45cc353a9), agent's receipts (1241a4bd6), the stop band
(ca8191f73) and the gpu note wrap.

- Image: `make dev-image PROFILE=standard`, throwaway keys.
- QEMU TCG, 4 CPUs, Intel IOMMU lane, fresh disk, setup to the desktop.
- `serial.log`: `[IOMMU] ... enforcing=1, unconfined grants=0`,
  `[SMP-PROOF] cpu_count=4 PASS`, `[INIT] Capsules spawned` at column 0,
  and 67 `[ZK-ATTEST] ok`. There are 0 `unanswered`, 0 `[TRAP`, `[PANIC` or
  `[FATAL` lines, and no tagged kernel line starts mid-line.
- `desktop.png`: the desktop, menu bar and dock.

Not covered: L81b part C (78edf5d46) and ksec's audit v2 (51f16539a), which
landed after this image was copied.
