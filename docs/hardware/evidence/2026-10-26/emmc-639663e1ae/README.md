# eMMC on QEMU's sdhci-pci (639663e1ae), and a 4-CPU gate, booted by the developers

The image is main 639663e1ae, standard-dev, throwaway keys, `--fresh`.

- emmc lane: `driver.emmc: host 0x1b36:0x7 (SDHCI)`, the card brought up at
  3300 mV, and the installer's disk screen lists "eMMC QEMU!! 4.3 GB, serial
  DEADBEEF, blank" (`06-disks.png`). No "register window too small" line.
- gate (4 CPUs, iommu=intel, setup to the desktop): SMP-PROOF 4 PASS,
  Capsules spawned, 67 ZK-ATTEST ok, 0 unanswered, 0 PANIC/FATAL/TRAP.
- `[CAP-DENY]` still seen: policy MkDebug (priv's), wallpaper MkDebug and
  virtio_gpu0 MkGetPid (both fixed since in 31e2ffd18, not in this image).
