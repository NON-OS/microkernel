# xhci0 on one CPU: the usb_msc0 scan window (3be5f780a8), booted

is standard-dev with throwaway keys and no diagnostic patch.

- QEMU TCG, `--smp 1`, Intel IOMMU lane, `--fresh`. Wait for "Capsules
  spawned", then 150 s, then Enter through setup, then 180 s.
- `unanswered driver.xhci0`: 0, 0, 0 and 0 over 4 boots. Unpatched main gave
  1, 1, 0 and 1. Every one of those named usb_msc0 as the starved caller,
  waiting about 6 s while the single CPU ran the desktop spawn wave.
- No `unanswered` line of any kind in any of the four boots.
- Still seen, and not this fix's: one "[DISPLAY] compositor silent 3 s:
  screen cleared" per boot. gpu's watchdog change (9ad05a662) addresses it.
