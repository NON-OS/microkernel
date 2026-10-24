# GPUs: Intel

Part of [GPUs](gpu.md).

## The driver

`capsule_driver_intel_display` claims an Intel function of PCI class 03, sub 00 or 80
(the `INTEL_GFX` broker row, `src/hardware/broker/scope_rows.rs`). It takes the device
only when its id is in one of the capsule's id files
(`userland/capsule_driver_intel_display/data/gen*_ids.txt`). With no listed id it exits
absent, and the desktop stays on the firmware framebuffer. Each family's generation, and
whether that generation sets modes, comes from `Family::gen` and `Family::sets_modes`
(`userland/capsule_driver_intel_display/src/family.rs`). Only Gen9 answers `MODE_SET`.
The capsule ships only in `microkernel-driver-intel-display` and has never been booted
on a machine. Its proofs run against a register model (`userland/intel_display_proofs`).

## Families

| Family | Display | Ids in | With the Intel profile | Display PRM (Intel doc no.) |
|---|---|---|---|---|
| Ivy Bridge and older | Gen7 and older | none | GOP | 2012 and older volumes |
| Haswell | Gen7.5 | `gen7_5_ids.txt` | takeover | Vol 11a Display, 690989 |
| Broadwell | Gen8 | `gen8_ids.txt` | takeover | Vol 11 Display, 690828 |
| Skylake | Gen9 | `gen9_ids.txt` | mode set | Vol 12 Display, 685893 |
| Kaby, Coffee, Comet Lake | Gen9 | `gen9_ids.txt` | mode set | Kaby Lake Vol 12, 723969 |
| Apollo, Gemini Lake | Gen9 LP | none | GOP | Apollo Lake Vol 7, 685504 |
| Ice Lake | Gen11 | `gen11_ids.txt` | takeover | Vol 12 Display Engine, 684506 |
| Jasper, Elkhart Lake | Gen11 | none | GOP | none found apart from Ice Lake's |
| Tiger Lake | Gen12 (Xe-LP) | `gen12_ids.txt` | takeover | Vol 12 Display Engine, 705833 |
| Rocket Lake | Gen12 | `gen12_ids.txt` | takeover | Vol 4 Display Engine, 705833 |
| Alder Lake S, Raptor Lake S | Gen12 | `gen12_ids.txt` | takeover | none published |
| Alder Lake P, Raptor Lake P | display 13 | `gen12_ids.txt` | takeover | none published |
| Alder Lake N | display 13 | none | GOP | none published |
| Meteor, Arrow Lake | Xe-LPD+ | none | GOP | none published |
| Lunar Lake | Xe2-LPD | none | GOP | none published |
| Iris Xe MAX (DG1) | Gen12 | none | GOP | Vol 12 Display Engine, 682696 |
| Arc A series (DG2) | Xe-HPG | none | GOP | Vol 12 Display Engines, 774520 |
| Arc B series (Battlemage) | Xe2 | none | GOP | none published |

The PRMs are on Intel's "Graphics for Linux, Programmer's Reference Manuals" pages
(intel.com, docs/graphics-for-linux/developer-reference/1-0/hardware-specs). They
download from `cdrdv2.intel.com/v1/dl/getcontent/<doc no.>`. The Comet and Coffee Lake
pages carry only a Configurations volume, so their display rows lean on Kaby Lake's.
"None published" means the site had no volume for that platform on 24 October 2026.
Rocket Lake's display volume is the same document as Tiger Lake's.

## What the gaps mean

- **Alder Lake and Raptor Lake** are sold in most Intel laptops and desktops from 2022
  register dumps show the same layout. Without a published volume, no register that
  differs can be written.
- **Meteor Lake, Arrow Lake and Lunar Lake** have no public display volume. They stay
  on the firmware framebuffer until Intel publishes one.
- **Arc A series** has a public PRM, but no id file or engine yet. It is a discrete
  card with its own memory, so it needs a unit of its own.


- On a machine, `lspci -nn -d 8086::0300` on another operating system gives the id to
  find in the files above.
