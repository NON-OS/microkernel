# GPUs: NVIDIA

Part of [GPUs](gpu.md).

## Today

Every NVIDIA display function (vendor 10de, class 03) is enumerate-only
(`src/hardware/inventory/support.rs`). The desktop runs on the UEFI GOP framebuffer
that the card's option ROM sets up. The loader chooses the mode in order: the panel's
EDID native timing, then firmware's mode if it has at least 720 lines, then the largest
offered mode (`choose`, `nonos-bootloader/src/display/gop/pick.rs`). The kernel maps it
write-combining (`src/arch/x86_64/pat/program.rs`). A card whose option ROM has no GOP
driver shows nothing. On a laptop whose panel is wired to an iGPU, the desktop is on the
iGPU and the NVIDIA GPU is unused.

The GOP gives no vertical blank, so the kernel's copy into the framebuffer is not
synchronised to the scan, and fast motion can tear. Nothing on the GOP path can fix
that: there is no vblank to wait on. How long a whole-frame copy takes on a given card
is for the owner's machine to measure; this tree has no figure for it yet.

## What NVIDIA has published

All of it is in open-gpu-doc (nvidia.github.io/open-gpu-doc), under an MIT-style
licence:

- `classes/display/`: the method headers of every display class, and a README mapping
  classes to GPUs. That covers EVO from NV50 to Pascal; NVDisplay `C370` on Volta; `C570`
  on Turing; `C670` and `C770` on Ampere and Ada; and `CA70` on Blackwell (GB20x).
- `manuals/`: register references. TU104 has the widest set (display, head, bus,
  FIFO, PBDMA, MMU, RAM, timer, top). GA102 has display, VM, trim (clocks), copy engine
  and context switch. GV100 and TU102 have fewer. GA102's display reference alone
  holds 10,627 definitions.
- `BIOS-Information-Table`, `DCB` (4.x) and `Devinit`: how to read the VBIOS's tables,
  including which outputs the board wires and the init scripts it carries.
- `Falcon-Security`: from Maxwell on, some registers can be written only by
  microcode NVIDIA has signed.
- `classes/compute/`, `classes/host/` and the QMD layouts: what gpu-5 would need.

## What native display would still need

The references read for this page (GA102's display manual and the `C67D` core channel
header) name registers, fields and methods, with short notes. They give no bring-up
order, no link training steps and no clock programming procedure like Intel's Vol 12.
A native driver for the first realistic target, Turing (NVD_30, RTX 20 and GTX 16,
with the widest manuals), would need all of this:

1. **A takeover at firmware's mode.** Read the head's raster and the scanout surface
   from PDISP, then flip the surface address through the core and window channels. This
   needs a pushbuffer and its instance memory in IOVA pages, confined by the IOMMU.
   The order in which channels are allocated and started is not given there.
2. **Hardware cursor and hot plug**, from the SOR and cursor registers.
3. **Mode setting and DisplayPort training** on the SOR, with clocks from the trim
   manual. Here the missing sequences matter most.
4. **Ampere and later.** NVIDIA states that its open driver runs much of this in GSP.
   The documents do not say whether the display engine can be programmed from the host
   without it. An owner-run register read on an RTX 30 card would answer that before
   any code is written.

None of this can be checked under QEMU. Each step would be written from the documents,
held to a register model, and booted by the owner. Without published sequences, each
step is also a hardware experiment, not only a reading. Honest cost: step 1 alone is
several weeks with an owner's Turing card on the bench. Steps 1 to 3 together are
months.

## Recommendation

- Keep NVIDIA on the boot framebuffer for 0.9.x.
- Native display: start after Intel Gen12 mode setting lands, Turing first, takeover
  only, and only with an owner's Turing machine able to run register-read builds.
- Native compute: see gpu-5's design; it rests on the same channel and MMU documents.
