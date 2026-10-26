# the hardware reruns and a 4-CPU gate, booted by the maintainers on main b0f7e93b7

The image is standard-dev with throwaway keys. Every run used `--fresh`.

| run | result |
|---|---|
| gate (smp4, iommu=intel, setup to the desktop) | SMP-PROOF 4 PASS, Capsules spawned, 67 ZK-ATTEST ok, 0 unanswered, 0 TRAP/PANIC/FATAL, no line split |
| nic=rtl8139 | `[NET-CORE] lease` reached |
| usb-audio | `[AUDIO] output driver.usb_audio0`, then `unplug:lane-usb-audio`, then `[AUDIO] output driver.hda0` |
| emmc | `driver.sdhci: device present, bring-up failed; not started (emmc: register window too small)` |
| ufs | 06-disks shows the disk; 0 `slow UFS` lines |

Still in every boot: `[CAP-DENY]` MkDebug from policy (pid 6, 12 to 17 times),
ps2_kbd0 (fixed since in 0cba3f66f9), xhci0 and wallpaper; MkGetPid from
virtio_gpu0; and 1 to 3 `[MUNMAP] release_va_failed`.
