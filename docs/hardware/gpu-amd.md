# GPUs: AMD

Part of [GPUs](gpu.md).

## Today

Every AMD display function (vendor 1002, class 03) is enumerate-only
(`src/hardware/inventory/support.rs`). There is no AMD display capsule, and wire kind 8
(`AMD`, `userland/nonos_gfx_wire/src/kind.rs`) is reserved for one. The desktop runs on
the UEFI GOP framebuffer at firmware's mode. A card whose option ROM has no GOP driver
shows nothing.

## Families

| Family | Display block | Typical parts | Today | Display register reference |
|---|---|---|---|---|
| R5xx, R6xx | AVIVO | X1000, HD 2000/3000, RS690 IGP | GOP or none | public (below) |
| R7xx to Northern Islands | DCE 3 to 5 | HD 4000 to HD 6000 | GOP or none | none published |
| Southern and Sea Islands (GCN 1, 2) | DCE 6 to 8 | HD 7000, R7/R9 200 | GOP | none published |
| Volcanic Islands, Polaris (GCN 3, 4) | DCE 10 to 11.2 | RX 400, 500 | GOP | none published |
| Vega (GCN 5) | DCE 12 | RX Vega, Radeon VII | GOP | none published |
| Raven, Picasso, Renoir APUs | DCN 1 to 2.1 | Ryzen 2000U to 5000U | GOP | none published |
| RDNA 1, 2 | DCN 2 to 3.0.x | RX 5000, RX 6000 | GOP | none published |
| Rembrandt, Phoenix APUs | DCN 3.1 to 3.1.4 | Ryzen 6000 to 8000 | GOP | none published |
| RDNA 3, 4 | DCN 3.2, 4 | RX 7000, RX 9000 | GOP | none published |

The display block names are AMD's own generation names. G-10 lists which AMD document
covers each era. Mapping an individual part to a block is for the owner to confirm.

## What AMD has published

On www.x.org/docs/AMD/old (AMD's own files, 2007 to 2013):

- display register references: `42589_rv630_rrg_1.01o.pdf`, `42590_m76_rrg_1.01o.pdf`,
  `43372_rs690_rrg_3.00o.pdf`, `RRG-216M56-03oOEM.pdf` (R5xx and R6xx only);
- 3D engine registers and programming guides for R6xx to Sea Islands (`R6xx_3D_Registers`,
  `evergreen_3D_registers_v2`, `cayman_3D_registers_v2`, `SI_3D_registers`,
  `CIK_3D_registers_v2`, `si_programming_guide_v2`), with no display registers;
- instruction set references from R600 to Southern Islands.

On GPUOpen (gpuopen.com, "AMD ISA documentation"), AMD publishes shader instruction set
references for GCN, RDNA and CDNA. They cover compute, not display.

No display register reference for DCE 6 or later, or any DCN, was found. Those are
every AMD GPU and Ryzen APU sold since about 2012.

## What that means

A native AMD display driver for any current part cannot be written from AMD's own
documents. G-10 put three choices to the owner:

- (a) read the register facts (names, offsets, fields) in another project's
  per-generation register headers, as facts only, and cite each one;
- (b) ask AMD for a display register reference under terms that allow this use;
- (c) leave AMD on the firmware framebuffer.

Until the owner rules, the tree holds (c), and the GPU breadth work writes no AMD
register. With (a) or (b), the order would be:

1. a takeover on DCN at firmware's mode: surface address and flip, vblank, cursor;
2. DMCUB firmware, sealed by digest as Intel's DMC is;
3. mode setting, link training and hot plug.

Each step needs its own proofs against a register model, and an owner boot.
