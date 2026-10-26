# S-14: the installers restart through capsule_power, holding no Admin

Booted on an image of privacy/s14-installer-admin **653523a51** (`head.txt`; image sha256 `6e173989…c1cf`).
- Built with `tools/nonos-dev-image --profile standard` and throwaway keys (`build.sh.txt`). Every check passed, [F] included.
- QEMU, TCG, `-smp 4`, 6 GiB, `--tpm`, lanes `iommu=intel`, `usb-kbd` and `usb-tablet`. nonos-qemu runs QEMU with `-no-reboot`, so a guest reset ends QEMU with exit 0.

## The runs

| Run | What happened |
|---|---|
| A: stick + blank NVMe, `--fresh --install-target` | Setup in Install mode (`A-03-mode-install.png`). The word "rget" was on screen (`A-23-erase.png`). app.install was spawned at caps **0x84009979, which has no Admin (1<<9)** (serial line 1455). It wrote 184.3 MB and every sector read back (`A-24-installed.png`). **Enter on the Done screen** gave the wiped restart's leave phase: `nvme: power-off: shut down`, then `[POWER] leave: driver.nvme0 done`, `driver.ahci0 done`, `driver.virtio_blk0 done` and `driver.usb_msc0 answered -19` (lines 1661-1665). Then the guest reset, and QEMU stopped with exit 0. |
| I: the installed NVMe alone, `--installed` | The installed disk booted with no stick. setup_wizard took the carried answers (601), then `[ZS] keeping turned on` (1192), then desktop_shell (1229), and the desktop was on screen (`I-20-installed-boot.png`). |

**The same press before the fix:** on the image at 29b6dc82b, the same Enter did nothing. power.off's HELD line admitted the shell alone, and an addressed send to capsule_power's shared inbox passes every gate the owner serves, so the installer's call was refused in the kernel. The fix widens HELD to the shell and both installers. capsule_power then tells them apart by the kernel-stamped sender (`off/who.rs`): an installer may restart, and its POFF frame and the shutdown op are refused.

## How each run ended

- **A:** the guest reset after the leave phase. QEMU (`-no-reboot`) stopped with exit 0.
- **I:** QMP `quit` with the desktop up.
- Neither serial log has a FATAL, PANIC, #PF, #GP or `panicked` line.

## Not shown

- **tool.install's `--reboot`:** the Terminal can't run tool.install. No TOOLS entry names it, and `install` is the market builtin. That's the Terminal owner's to add.
- **The serial echo of kernel warnings:** there are 0 `[KLOG` lines on both boots. The echo is compiled for this image's features, but the kernel's LogManager is never initialised: `init_early()`, the only caller of `crate::log::init_logger()`, went in aa4d73213. So `LOGGER` stays `None`, every `log::warn!` is dropped, and the backend is dead code (the kernel ELF holds no "[KLOG "). Initialising it is a follow-up unit: its x86 VgaBackend writes the 0xB8000 text buffer, which a GOP machine doesn't have. The echo's budget and cfg are held by kernel_proofs `log_budget` and four mutants.
