# The BGA display driver, booted on both its lanes

The image is SCRATCH: a profile [microkernel-full-gui, nonos-capsule-driver-bga]
with capsule serial debug, built with throwaway keys at privacy/k-series
d033d5369 (main's heap fix 85373255b included), sha256
a54b0c122fd8c086fdc876236705048251d759822654dc4773b52431343e3c19.
Booted under QEMU 11.1.1 with OVMF, `--lane iommu=intel`, plus the lane
named in each file.

| lane | device | what booted |
|---|---|---|
| `gpu=bga` | `-device VGA` | `[SPAWN] name=driver.bga0 … caps=0x39118`, `[DRIVER-BGA] capsule spawned`, `[DISPLAY] gfx lookup: driver.bga0`, `[DISPLAY] on driver.bga0 1280x800, canvas 1280x800 at scale 1.00`; first-boot setup drawn there (`gpu-bga-setup.png`) |
| `bochs-display` | `-device bochs-display` | the same lines, and setup drawn (`bochs-display-setup.png`) |

On both lanes the run printed no panic and no allocation failure. The run
on 30e21a53 had ended with "memory allocation of 64 bytes failed" (exit 134)
after the mode set.

Not shown: a line reading "driver.bga: up". No line names the driver up in
those words; the compositor's `[DISPLAY] on driver.bga0` line shows it
answered.
