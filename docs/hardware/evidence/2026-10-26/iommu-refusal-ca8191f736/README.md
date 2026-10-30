# Air-Gapped with no IOMMU: the stop screen (ca8191f736), booted

Booted by the maintainers on main 607b74dffa, which carries ca8191f736 (the stop
band clears the loader's screen under it). The image also carried a scratch
diagnostic patch, never committed, in the xhci0 server and the kernel's
starved-call print; neither touches the posture stop.

- QEMU TCG, no IOMMU lane, `--fresh`, boot-menu key 4 (Air-Gapped).
- `refusal.png`: only the band, "NONOS BOOT STOPPED / IOMMU / air-gapped
  needs an IOMMU to confine device DMA; this machine has none", over black.
  No loader text is left under it.
- `serial-excerpt.log`: the `[FATAL] IOMMU` line. No capsule spawned.
