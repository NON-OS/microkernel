# GPUs: NVIDIA

Part of [GPUs](gpu.md).

## Today

Every NVIDIA display function (vendor 10de, class 03) is enumerate-only
(`src/hardware/inventory/support.rs`). The desktop runs on the UEFI GOP framebuffer
that the card's option ROM sets up, at firmware's mode, through the kernel's present
path ([Display](../drivers/display.md#the-gop-framebuffer)). A card whose option ROM
has no GOP driver shows nothing. On a laptop whose panel is wired to an Intel or AMD
iGPU, the desktop is on the iGPU and the NVIDIA GPU is unused.

## Families

| Generation | Typical parts | Today | GSP firmware needed for display |
|---|---|---|---|
| Kepler and older | GTX 600, 700 | GOP or none | no |
| Maxwell, Pascal | GTX 900, 10 series | GOP | no |
| Volta | Titan V | GOP | no |
| Turing | GTX 16, RTX 20 | GOP | optional on NVIDIA's open driver |
| Ampere, Ada | RTX 30, 40 | GOP | yes on NVIDIA's open driver |
| Blackwell | RTX 50 | GOP | yes |

The last column follows NVIDIA's own statements about its open kernel modules. It is
not checked against a document in this tree, and is for the owner to confirm before any
unit rests on it.

## What NVIDIA has published

NVIDIA's open-gpu-doc (nvidia.github.io/open-gpu-doc, and download.nvidia.com/open-gpu-doc)
holds, among others:

- display class methods (the push-buffer methods of each display class);
- display reference manuals;
- the BIOS information table, the display configuration block (DCB) and devinit;
- falcon security notes;
- compute class methods and QMD layouts.

What these cover, and what native display would still need, is the subject of unit
gpu-4's letter. This page will name the documents by file once that unit has read them.

## The firmware framebuffer on NVIDIA

The firmware framebuffer path is the same on every vendor: the mode the loader picks
(`choose`, `nonos-bootloader/src/display/gop/pick.rs`) and a frame mapped
write-combining (`src/arch/x86_64/pat/program.rs`). The GOP gives no vblank, so the
kernel's copy into the framebuffer is not synchronised to the scan, and a fast-moving
picture can tear.
