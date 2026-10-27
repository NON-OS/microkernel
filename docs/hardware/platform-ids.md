# Platform controller ids

The xHCI, HD Audio, SATA, I2C, SMBus, VMD and ACP functions of every Intel platform from the 300-series PCH to Arrow and Lunar Lake, and every AMD platform from Raven Ridge to Strix Point, with the source of each id. What NONOS does with them is in [Mainstream hardware](mainstream.md). Gathered by S4 for NONOS. All sources were fetched on 2026-10-27. Linux sources are `torvalds/linux` master as of that date. pci.ids is version 2026.10.27.

## What NONOS claims

Checked against the drivers on 27 October 2026, by how each matches:

| Function | How NONOS matches it | Every platform below claimed? |
|---|---|---|
| xHCI (PCH, SoC, AMD FCH and PROM21, Thunderbolt/USB4 TCSS) | class 0c/03/30, any vendor (`capsule_driver_xhci`, broker scope) | Yes |
| HD Audio | class 04/03 from any vendor, 04/01 from Intel (`capsule_driver_hda`) | Yes for the codec path. Intel 04/03/80 is claimed as 04/03; a DSP-only machine's microphones need SOF, which NONOS does not have |
| SATA AHCI | class 01/06, any vendor (`capsule_driver_ahci`) | Yes |
| NVMe | class 01/08/02, any vendor (`capsule_driver_nvme`) | Yes |
| Intel LPSS I2C | id table with each family's clock (`capsule_driver_i2c_pci`, `src/hardware/inventory/i2c_host.rs`) | Yes, Cannon Lake to Lunar Lake since 27 October 2026; others through the unlisted gate |
| AMD DesignWare I2C | ACPI AMDI0010 | Yes |
| SMBus | Intel class 0c/05 with an I/O BAR (`capsule_driver_smbus`) | Intel yes; AMD FCH 1022:790b no |
| Intel VMD | kernel id table, Linux's `vmd` ids (`src/drivers/pci/vmd/domain/ids.rs`) | Yes, but for 28c1; never run on a VMD machine |
| AMD ACP | none | No |
| Intel Touch Host Controller (Lunar Lake a848/a84a) | none | No |

## How to read this

- **What a source proves.** When a cell cites a Linux driver, that driver's id table or its comments name the id and the platform. When a cell cites only [pci.ids][ids], the id exists and has that name in the community database, but no driver table ties it to a platform. Where the name in pci.ids and the name in Linux disagree, the row says so.
- **UNVERIFIED** marks any id or platform mapping could not be confirmed in a primary source.
- **Class codes** are base/sub/prog-if:
  - xHCI is `0c/03/30`.
  - AHCI is `01/06/01`.
  - VMD is `01/04/00` (RAID). Linux-hardware files 9a0b/467f/a77f under "Storage/raid" in [DevicePopulation][lhwpop].
  - Intel LPSS I2C is `0c/80/00`. Linux-hardware files a0e8/51e8 under "Serial bus controller".
  - SMBus is `0c/05/00`.
  - AMD ACP is `04/80/00`. Linux-hardware files 1022:15e2 under "Multimedia controller".
  - None of the drivers below match on the I2C, VMD or ACP class codes, so those three are UNVERIFIED as exact prog-if values.
- **Intel HD Audio (HDA): one id, three class codes.** Intel HDA uses the same PCI device id whether the audio DSP is on or off. Firmware sets the class code, and Linux decides from it ([intel-dsp-config.c][dspcfg], `snd_intel_dsp_driver_probe`):
  - `04/03/00` means no DSP, so the legacy `snd-hda-intel` driver is used.
  - `04/01/00` means a DSP is present and may be needed for digital microphones (DMIC) or SSP. SOF (Sound Open Firmware) or AVS is used.
  - `04/03/80` means either driver may be used.
  - So the tables below give one HDA id per platform. "DSP/SOF function" is a class-code state of that one function, not a separate PCI function.
- **Intel I2C.** Intel LPSS (Low Power Subsystem) I2C functions are bound by the MFD driver `intel-lpss-pci` ([intel-lpss-pci.c][lpss]). It creates `i2c_designware` platform children. `i2c-designware-pcidrv.c` only covers Medfield, Merrifield, Baytrail and similar ([i2c-designware-pcidrv.c][dwpci]).
- **AMD I2C is not PCI.** It is the ACPI devices `AMDI0010` and `AMDI0019` (`AMD0010` on older parts), bound by `i2c-designware-platdrv` ([platdrv][dwplat]). Clock setup is in [acpi_apd.c][apd].
- **xHCI binds by class.** `xhci-pci` matches every `0c/03/30` function. Its named ids exist only for quirks ([xhci-pci.c][xhci]). Missing ids below do not mean "unsupported".

## Intel

### xHCI (PCH or SoC standalone, and TCSS / Thunderbolt 4 / USB4)

| Platform | Function | vendor:device | Class | Source |
|---|---|---|---|---|
| Coffee Lake-S/H (300-series PCH, CNL-H) | PCH xHCI | 8086:a36d | 0c/03/30 | [pci.ids][ids] ("300/C240 Series ... USB 3.1 xHCI") |
| Whiskey Lake / CNL-U (CNP-LP) | PCH xHCI | 8086:9ded | 0c/03/30 | [pci.ids][ids] |
| Comet Lake-U (400 On-Package) | PCH xHCI | 8086:02ed | 0c/03/30 | [pci.ids][ids] |
| Comet Lake-H/S (400 series) | PCH xHCI | 8086:06ed | 0c/03/30 | [pci.ids][ids] |
| Comet Lake B460/H410 (CML-V PCH) | PCH xHCI | 8086:a3af | 0c/03/30 | [xhci-pci.c][xhci] `COMET_LAKE_XHCI 0xa3af`; [pci.ids][ids] |
| Ice Lake-U/Y | PCH xHCI | 8086:34ed | 0c/03/30 | [pci.ids][ids] |
| Ice Lake-U/Y | TCSS (TBT3) xHCI | 8086:8a13 | 0c/03/30 | [xhci-pci.c][xhci] `ICE_LAKE_XHCI 0x8a13` |
| Tiger Lake-UP3/UP4 | PCH xHCI | 8086:a0ed | 0c/03/30 | [xhci-pci.c][xhci] `TIGER_LAKE_PCH_XHCI` |
| Tiger Lake-UP3/UP4 | TCSS (TBT4) xHCI | 8086:9a13 | 0c/03/30 | [xhci-pci.c][xhci] `TIGER_LAKE_XHCI` |
| Tiger Lake-H (500 series PCH) | PCH xHCI | 8086:43ed | 0c/03/30 | [pci.ids][ids] |
| Tiger Lake-H | TCSS xHCI | 8086:9a17 | 0c/03/30 | [pci.ids][ids] |
| Alder Lake-S (600 series) | PCH xHCI | 8086:7ae0 | 0c/03/30 | [pci.ids][ids] |
| Alder Lake-P/U and Raptor Lake-P/U/H (shared PCH-P die) | PCH xHCI | 8086:51ed | 0c/03/30 | [xhci-pci.c][xhci] `ALDER_LAKE_PCH_XHCI`. That RPL-P reuses it is UNVERIFIED: pci.ids names it only "Alder Lake PCH". |
| Alder Lake-P | TCSS (TBT4) xHCI | 8086:461e | 0c/03/30 | [pci.ids][ids]. Its NHI 463e/466d is in [nhi.h][nhi]. |
| Alder Lake-N | PCH xHCI | 8086:54ed | 0c/03/30 | [xhci-pci.c][xhci] `ALDER_LAKE_N_PCH_XHCI` |
| Alder Lake-N | SoC xHCI | 8086:464e | 0c/03/30 | [pci.ids][ids] |
| Raptor Lake-S (700 series) | PCH xHCI | 8086:7a60 | 0c/03/30 | [pci.ids][ids] |
| Raptor Lake-P | TCSS (TBT4) xHCI | 8086:a71e | 0c/03/30 | [pci.ids][ids]. Its NHI a73e/a76d is in [nhi.h][nhi]. |
| Meteor Lake-H/U | SoC xHCI | 8086:7e7d | 0c/03/30 | [pci.ids][ids] |
| Meteor Lake-H/U (and Arrow Lake, see note 1) | TCSS xHCI | 8086:7ec0 | 0c/03/30 | [pci.ids][ids] ("Core Ultra 200 Series Processors USB xHCI"). Linux [nhi.h][nhi] names the sibling NHI 0x7ec2 `MTL_P_NHI0`. |
| Arrow Lake-S (800 series PCH) | PCH xHCI | 8086:7f6e | 0c/03/30 | [pci.ids][ids] |
| Arrow Lake-H/U | SoC "standalone" xHCI | 8086:777d | 0c/03/30 | [pci.ids][ids]. The platform is inferred from the 77xx block, which [intel-lpss-pci.c][lpss] labels ARL-H. |
| Arrow Lake-H/S | TCSS xHCI | 8086:7ec0 (UNVERIFIED) | 0c/03/30 | See note 1 |
| Lunar Lake | SoC standalone xHCI | 8086:a87d | 0c/03/30 | [pci.ids][ids] |
| Lunar Lake | TCSS xHCI | 8086:a831 | 0c/03/30 | [pci.ids][ids]. Its NHI a833/a834 is in [nhi.h][nhi] `LNL_NHI0/1`. |
| Discrete Thunderbolt 4/5 (Maple Ridge, Barlow Ridge) | xHCI / NHI | 8086:1138 (Maple Ridge xHCI); NHI 5781/5784 (Barlow Ridge) | 0c/03/30 | [xhci-pci.c][xhci]; [nhi.h][nhi] |

Note 1: pci.ids now labels 7ec0–7ec7 "Core Ultra 200 Series Processors". Linux names that block MTL-P. Arrow Lake-H is built from the Meteor Lake SoC lineage, so sharing is plausible. No lspci from an Arrow Lake machine showing 7ec0 was found, so it is UNVERIFIED.

### HD Audio (same id whether HDA or SOF; DSP state is in the class code)

| Platform | vendor:device | Class options | Linux default when DSP class is set | Source |
|---|---|---|---|---|
| Coffee Lake (CNL-H PCH) | 8086:a348 | 04/03/00 or 04/01/00 | SOF only if DMIC or SoundWire is present | [hda intel.c][hda] `HDA_CNL_H`; [intel-dsp-config.c][dspcfg] |
| Whiskey/Cannon Lake-LP | 8086:9dc8 | same | SOF if DMIC/SoundWire | [hda][hda]; [dspcfg][dspcfg] |
| Comet Lake-LP | 8086:02c8 | same | SOF if DMIC/SoundWire | [hda][hda] |
| Comet Lake-H | 8086:06c8 | same | SOF if DMIC/SoundWire | [hda][hda] |
| Comet Lake-S / CML-V | 8086:a3f0 | 04/03/00 | legacy | [hda][hda] `HDA_CML_S` |
| Ice Lake-LP | 8086:34c8 | same | SOF if DMIC/SoundWire | [hda][hda] |
| Tiger Lake-LP | 8086:a0c8 | same | SOF if DMIC/SoundWire | [hda][hda]; [dspcfg][dspcfg] |
| Tiger Lake-H | 8086:43c8 | same | SOF if DMIC/SoundWire | [hda][hda] |
| Alder Lake-S | 8086:7ad0 | same | SOF if DMIC/SoundWire | [hda][hda] |
| Alder Lake-P / -PS / -PX / -M | 8086:51c8 / 51c9 / 51cd / 51cc | same | SOF if DMIC/SoundWire | [hda][hda] |
| Alder Lake-N | 8086:54c8 | same | SOF if DMIC/SoundWire | [hda][hda] |
| Raptor Lake-S (700 PCH) | 8086:7a50 | same | SOF if DMIC/SoundWire | [hda][hda] `HDA_RPL_S` |
| Raptor Lake-P/U/H | 8086:51ca, 51cb (M: 51ce, PX: 51cf) | same | SOF if DMIC/SoundWire | [hda][hda] `HDA_RPL_P_0/1` |
| Meteor Lake-H/U | 8086:7e28 | same | SOF if DMIC/SoundWire; firmware `intel/sof-ipc4/mtl/sof-mtl.ri` | [hda][hda] `HDA_MTL`; [pci-mtl.c][sofmtl] |
| Arrow Lake-S (800 PCH "ACE") | 8086:7f50 | same | SOF `intel/sof-ipc4/arl-s/sof-arl-s.ri` | [hda][hda] `HDA_ARL_S`; [pci-mtl.c][sofmtl] |
| Arrow Lake-H/U | 8086:7728 | same | SOF `intel/sof-ipc4/arl/sof-arl.ri` | [hda][hda] `HDA_ARL`; [pci-mtl.c][sofmtl] |
| Lunar Lake | 8086:a828 | same | SOF `intel/sof-ipc4/lnl/sof-lnl.ri` | [hda][hda] `HDA_LNL_P`; [pci-lnl.c][soflnl] |

Linux-hardware labels a0c8, 34c8 and a348 "cAVS" or "Smart Sound". That reflects the class most often seen in probes: OEM laptops with DMIC ship the 04/01 class ([DevicePopulation][lhwpop]).

### SATA AHCI

| Platform | vendor:device (AHCI) | RAID-mode ids | Source |
|---|---|---|---|
| Coffee Lake (300 series) | 8086:a352 (desktop), a353 (mobile) | a355/a356/a357, a35e (Optane) | [pci.ids][ids]; [ahci.c][ahci] lists a356 as "Cannon Lake PCH-H RAID" |
| Whiskey/CNL-LP | 8086:9dd3 | — | [ahci.c][ahci] "Cannon Lake PCH-LP AHCI" |
| Comet Lake-U | 8086:02d3 | 02d5/02d7 | [ahci.c][ahci] "Comet Lake PCH-U AHCI" |
| Comet Lake-H/S | 8086:06d2/06d3 | 06d5/06d7, 06de | [pci.ids][ids]; [ahci.c][ahci] (06d6, 06d7) |
| Ice Lake-LP | 8086:34d3 | — | [ahci.c][ahci] |
| Tiger Lake-LP | 8086:a0d3 | a0d5/a0d7 | [pci.ids][ids] |
| Tiger Lake-H (500 series) | 8086:43d2/43d3 | 43d4–43d7 | [pci.ids][ids]; [ahci.c][ahci] |
| Alder Lake-S | 8086:7ae2 | — | [pci.ids][ids] |
| Alder Lake-P / RPL-P | 8086:51d3 | — | [pci.ids][ids] |
| Alder Lake-N | 8086:54d3 | — | [pci.ids][ids] |
| Raptor Lake-S | 8086:7a62 | — | [pci.ids][ids] |
| Meteor Lake | none found in pci.ids or ahci.c (UNVERIFIED whether the SoC has SATA at all) | — | — |
| Arrow Lake-S (800 series) | 8086:7f62 | 7f66 | [pci.ids][ids] |
| Arrow Lake-H | 8086:7763 | 7767 | [pci.ids][ids] ("200H/200V"; the H/V split is UNVERIFIED) |
| Lunar Lake | none found | — | — |

Any `01/06/01` function binds `ahci` by class ([ahci.c][ahci], `PCI_CLASS_STORAGE_SATA_AHCI`). The per-id entries add only quirks and RAID-mode ids.

### I2C (Intel LPSS DesignWare, bound by intel-lpss-pci)

| Platform | I2C function ids (I2C0..I2C5 or more) | Source |
|---|---|---|
| CNL-H (300 series) | 8086:a368, a369, a36a, a36b | [lpss][lpss] "CNL-H" |
| CNL-LP | 8086:9de8–9deb, 9dc5, 9dc6 | [lpss][lpss] "CNL-LP" |
| CML-LP | 8086:02e8–02eb, 02c5, 02c6 | [lpss][lpss] "CML-LP" |
| CML-H | 8086:06e8–06eb | [lpss][lpss] "CML-H" |
| CML-V | 8086:a3e0–a3e3 | [lpss][lpss] "CML-V" |
| ICL-LP | 8086:34e8–34eb, 34c5, 34c6 | [lpss][lpss] "ICL-LP" |
| TGL-LP | 8086:a0e8–a0eb, a0c5, a0c6, a0d8, a0d9 | [lpss][lpss] "TGL-LP" |
| TGL-H | 8086:43e8–43eb, 43ad, 43ae, 43d8 | [lpss][lpss] "TGL-H" |
| ADL-S | 8086:7acc–7acf, 7afc, 7afd | [lpss][lpss] "ADL-S" |
| ADL-P (and RPL-P, same PCH; UNVERIFIED) | 8086:51e8–51eb, 51c5, 51c6, 51d8, 51d9 | [lpss][lpss] "ADL-P" |
| ADL-M / ADL-N | 8086:54e8–54eb, 54c5, 54c6 | [lpss][lpss] "ADL-M". That ADL-N shares these is UNVERIFIED. |
| RPL-S | 8086:7a4c–7a4f, 7a7c, 7a7d | [lpss][lpss] "RPL-S" |
| MTL-P | 8086:7e78–7e7b, 7e50, 7e51 | [lpss][lpss] "MTL-P" |
| ARL-S (800 PCH, which Linux calls "MTP-S") | 8086:7f4c–7f4f, 7f7a, 7f7b | [lpss][lpss] "MTP-S"; [pci.ids][ids] "800 Series Chipset Family I2C" |
| ARL-H | 8086:7778–777b, 7750, 7751 | [lpss][lpss] "ARL-H" |
| LNL-M | 8086:a878–a87b, a850, a851 | [lpss][lpss] "LNL-M" |
| LNL (touch path) | 8086:a848, a84a (THC QuickI2C) | [quicki2c-dev.h][thc]. Lunar Lake touchpads may sit behind the Touch Host Controller instead of LPSS (UNVERIFIED which OEMs do this). |
| AMD (all of Raven…Strix) | not PCI: ACPI `AMDI0010` (`AMDI0019` with arbitration semaphore) | [platdrv][dwplat]; [acpi_apd.c][apd] |

### SMBus (i2c-i801)

| Platform | vendor:device | Source |
|---|---|---|
| Cannon Lake-H (300) | 8086:a323 | [i2c-i801.c][i801] header table |
| Cannon Lake-LP | 8086:9da3 | [i801][i801] |
| Comet Lake (LP) | 8086:02a3 | [i801][i801] |
| Comet Lake-H | 8086:06a3 | [i801][i801] |
| Comet Lake-V | 8086:a3a3 | [i801][i801] |
| Ice Lake-LP | 8086:34a3 | [i801][i801] |
| Tiger Lake-LP / -H | 8086:a0a3 / 43a3 | [i801][i801] |
| Alder Lake-S / -P / -M (N) | 8086:7aa3 / 51a3 / 54a3 | [i801][i801] |
| Raptor Lake-S | 8086:7a23 | [i801][i801] |
| Meteor Lake-P / SoC-S / PCH-S (800 series, used with ARL-S) | 8086:7e22 / ae22 / 7f23 | [i801][i801] |
| Arrow Lake-H | 8086:7722 | [i801][i801] |
| Lunar Lake | 8086:a822 (pci.ids) | **Not in the i2c-i801 id table on master** ([i801][i801]; [pci.ids][ids]) |
| AMD FCH (Raven…Strix, AM5) | 1022:790b | `i2c-piix4`, `PCI_DEVICE_ID_AMD_KERNCZ_SMBUS` ([pci_ids.h][pciidsh]; [i2c-piix4.c][piix4]). That every listed AMD platform uses 790b is UNVERIFIED. |

### VMD (Intel Volume Management Device, bound by `vmd`)

| vendor:device | Platform | Source |
|---|---|---|
| 8086:9a0b | Tiger Lake | [vmd.c][vmd] `VMD_9A0B` (client feats). Platform from [pci.ids][ids] "Volume Management Device NVMe RAID Controller" plus the TGL 9axx block (UNVERIFIED). |
| 8086:4c3d | Rocket Lake-S (500 series) — UNVERIFIED | [vmd.c][vmd] |
| 8086:467f | Alder Lake | [vmd.c][vmd]. ADL platform inferred from the 46xx block (UNVERIFIED). |
| 8086:a77f | Raptor Lake ("RST Volume Management Device Controller") | [vmd.c][vmd]; [pci.ids][ids]. P vs S is UNVERIFIED. |
| 8086:7d0b | Meteor Lake / Arrow Lake-H / (Lunar Lake?) | [vmd.c][vmd] (`VMD_FEAT_INTERRUPT_QUIRK`); [pci.ids][ids] "Core Ultra 200H/200V ... VMD". Lunar Lake use is UNVERIFIED. |
| 8086:ad0b | Arrow Lake-S | [vmd.c][vmd]; [pci.ids][ids] "Core Ultra 200 Series Processors VMD" |
| 8086:b60b, b06f, b07f, d70b, d73b | Panther Lake / Nova Lake and later (outside scope) | [vmd.c][vmd]; [pci.ids][ids] d70b "Series 4" |
| 8086:09ab | Placeholder "RST VMD Managed Controller" seen when VMD hides the NVMe (system-peripheral class); no driver | [pci.ids][ids]; [DevicePopulation][lhwpop] files it under "System peripheral" |

Prevalence: see mainstream.md, storage section.

## AMD

### xHCI

| Platform | Function | vendor:device | Class | Source |
|---|---|---|---|---|
| Raven Ridge / Picasso (Ryzen 2000/3000 mobile, APU desktop) | SoC xHCI ×2 | 1022:15e0, 1022:15e1 | 0c/03/30 | [xhci-pci.c][xhci] `RAVEN_15E0/15E1`; [coreboot pci_ids.h][cb] `FAM17H_MODEL18H_XHCI0/1` |
| Raven2 / Dali / Pollock | SoC xHCI | 1022:15e5 | 0c/03/30 | [xhci-pci.c][xhci] `RAVEN2_XHCI`; [cb][cb] |
| Renoir / Lucienne / Cezanne / Barcelo | SoC xHCI (two functions, same id) | 1022:1639 | 0c/03/30 | [xhci-pci.c][xhci] `RENOIR_XHCI`; [pci.ids][ids] "Renoir/Cezanne USB 3.1" |
| Rembrandt (Ryzen 6000) | USB4-capable xHCI #1–#8 | 1022:161a, 161b, 161d, 161e, 15d6, 15d7, 161c, 161f | 0c/03/30 | [pci.ids][ids] "Rembrandt USB4 XHCI controller #n" |
| Rembrandt | USB4 NHI | 1022:162e, 162f | 0c/03/40 (UNVERIFIED) | [pci.ids][ids] |
| Phoenix / Hawk Point (Ryzen 7040/8040, 8000G) | SoC xHCI | 1022:15b9 confirmed as "AMD Family 19h USB XHCI Host Controller". 15ba, 15c0, 15c1 are UNVERIFIED (not in pci.ids, Linux or coreboot). | 0c/03/30 | [linux-hardware 1022:15b9][lh15b9] |
| Phoenix | USB4 NHI | 1022:15c4, 15c5 ("Phoenix USB4/Thunderbolt NHI"); 1668, 1669 ("Pink Sardine USB4 NHI") | — | [pci.ids][ids] |
| Phoenix2 | SoC xHCI | 1022:15bb, 15c2, 15c3 | 0c/03/30 | [pci.ids][ids] |
| Hawk Point | same SoC as Phoenix | GPU id differs (1002:1900/1901); xHCI ids assumed identical to Phoenix — UNVERIFIED | — | [pci.ids][ids] |
| Raphael / Granite Ridge (AM5 CPU) | CPU-integrated xHCI | 1022:15b6, 15b7 (USB 3.x), 15b8 (USB 2.0) | 0c/03/30 | [pci.ids][ids] "Raphael/Granite Ridge USB 3.1 xHCI" |
| AM5 600-series chipset (Promontory 21: B650/X670, A620) | chipset xHCI | 1022:43f7 (600 series), 43f9 (B665), 43fa (A620) | 0c/03/30 | [xhci-pci.c][xhci] `PROM21_XHCI_43F7/43F9/43FA`; [pci.ids][ids] |
| AM5 800-series chipset (X870/B850) | chipset xHCI | 1022:43fc, 43fd | 0c/03/30 | [xhci-pci.c][xhci] `PROM21_XHCI_43FC/43FD`; [pci.ids][ids] |
| AM4 500/300 chipset (for comparison) | chipset xHCI | 1022:43ee (500), 43ec (A520), 43b9/43ba/43bb/43bc (Promontory A) | 0c/03/30 | [xhci-pci.c][xhci]; [pci.ids][ids] |
| Strix Point (Ryzen AI 300) | SoC xHCI | 1022:151a, 151b, 151e, 151f | 0c/03/30 | [pci.ids][ids] "Strix USB 3.1 xHCI" |
| Strix Point | USB4 NHI | 1022:151c, 151d | — | [pci.ids][ids] |

### HD Audio (plain HDA; AMD has no DSP-mode HDA)

| Platform | Function | vendor:device | Class | Source |
|---|---|---|---|---|
| Raven → Strix Point, Raphael (all Zen APUs since Raven) | "Ryzen HD Audio Controller" (analog codec link) | 1022:15e3 | 04/03/00 | [hda intel.c][hda] `/* AMD Raven */ PCI_VDEVICE(AMD, 0x15e3)`. AM5 is shown by the X870E quirk on 15e3 in the same file. Strix is UNVERIFIED. |
| Raven / Picasso | iGPU HDMI/DP audio | 1002:15de | 04/03/00 | [pci.ids][ids] |
| Renoir / Cezanne | iGPU HDMI/DP audio | 1002:1637 | 04/03/00 | [pci.ids][ids] |
| Rembrandt / Phoenix / Raphael / Strix | iGPU HDMI/DP audio "Radeon High Definition Audio Controller" | 1002:1640 | 04/03/00 | [pci.ids][ids]. The per-platform mapping is UNVERIFIED. |
| AM4 Matisse / Vermeer (for comparison) | Starship/Matisse HDA | 1022:1487 | 04/03/00 | [hda][hda] |

### ACP (Audio Co-Processor): 1022:15e2, told apart by PCI revision

All ACP generations use device id 1022:15e2 ([mach-config.h][machcfg] `ACP_PCI_DEV_ID 0x15E2`). The driver chosen depends on the revision byte.

| Platform | ACP rev | Linux driver (non-SOF) | SOF variant (only on DMI-listed machines) | Source |
|---|---|---|---|---|
| Raven / Picasso | 0x00 | `snd-pci-acp3x` (I2S) | — | [raven/pci-acp3x.c][raven] |
| Renoir / Cezanne | 0x01 | `snd-rn-pci-acp3x` (PDM DMIC); `snd-acp-pci` "acp_asoc_renoir" | `sof-rn.ri` (`ACP_RN_PCI_ID 0x01`) | [rn-pci-acp3x.c][rn]; [acp-pci.c][acppci]; [sof acp.h][sofacp]; [pci-rn.c][sofrn] |
| Van Gogh (Steam Deck) | 0x50 | — | `sof-vangogh` | [sof acp.h][sofacp] |
| Rembrandt (Yellow Carp) | 0x6f (0x60 also accepted) | `snd-acp6x` (yc) PDM DMIC | `sof-rmb.ri` | [yc pci-acp6x.c][yc]; [pci-rmb.c][sofrmb] |
| "RPL" in AMD driver = Raphael (desktop AM5) — expansion UNVERIFIED | 0x62 | `snd-acp6x` | — | [yc pci-acp6x.c][yc] (`case 0x62: /* RPL */`) |
| Phoenix / Hawk Point (Pink Sardine, ACP 6.3) | 0x63 | `snd-pci-ps` (PDM DMIC and SoundWire) | `sof-acp_6_3.ri` | [pci-ps.c][ps]; [acp63.h][acp63]; [pci-acp63.c][sofacp63] |
| Strix Point / Krackan / Strix Halo (ACP 7.0/7.1/7.2) | 0x70, 0x71, 0x72 | `snd-pci-ps` | `sof-acp_7_0.ri` | [pci-ps.c][ps]; [acp-config.c][acpcfg] (comments "Strix Point, ACP 7.0" and "Strix Point, ACP 7.2"); [pci-acp70.c][sofacp70] |

### SATA, SMBus, I2C, VMD on AMD

| Platform | Class | vendor:device | Source |
|---|---|---|---|
| FCH (Raven…Cezanne laptops/APUs) | SATA AHCI | 1022:7901 | [ahci.c][ahci] (`PCI_VDEVICE(AMD, 0x7901)`); [pci.ids][ids] |
| AM5 600-series chipset (PROM21) | SATA AHCI | 1022:43f6 | [pci.ids][ids] "600 Series Chipset SATA Controller" |
| AM4 500 / 400 / 300 | SATA AHCI | 1022:43eb / 43c8 / 43b7 | [pci.ids][ids] |
| All | SMBus | 1022:790b (i2c-piix4) | [pci_ids.h][pciidsh] |
| All | I2C | ACPI AMDI0010 / AMDI0019, not PCI | [platdrv][dwplat] |
| All | VMD | none (AMD uses RAIDXpert2 on the chipset, not VMD) | [ASUS TUF B650 spec][tufb650] ("AMD RAIDXpert2 Technology") |

[ids]: https://pci-ids.ucw.cz/v2.2/pci.ids
[xhci]: https://github.com/torvalds/linux/blob/master/drivers/usb/host/xhci-pci.c
[nhi]: https://github.com/torvalds/linux/blob/master/drivers/thunderbolt/nhi.h
[hda]: https://github.com/torvalds/linux/blob/master/sound/hda/controllers/intel.c
[dspcfg]: https://github.com/torvalds/linux/blob/master/sound/hda/core/intel-dsp-config.c
[sofmtl]: https://github.com/torvalds/linux/blob/master/sound/soc/sof/intel/pci-mtl.c
[soflnl]: https://github.com/torvalds/linux/blob/master/sound/soc/sof/intel/pci-lnl.c
[ahci]: https://github.com/torvalds/linux/blob/master/drivers/ata/ahci.c
[lpss]: https://github.com/torvalds/linux/blob/master/drivers/mfd/intel-lpss-pci.c
[dwpci]: https://github.com/torvalds/linux/blob/master/drivers/i2c/busses/i2c-designware-pcidrv.c
[dwplat]: https://github.com/torvalds/linux/blob/master/drivers/i2c/busses/i2c-designware-platdrv.c
[apd]: https://github.com/torvalds/linux/blob/master/drivers/acpi/acpi_apd.c
[thc]: https://github.com/torvalds/linux/blob/master/drivers/hid/intel-thc-hid/intel-quicki2c/quicki2c-dev.h
[i801]: https://github.com/torvalds/linux/blob/master/drivers/i2c/busses/i2c-i801.c
[piix4]: https://github.com/torvalds/linux/blob/master/drivers/i2c/busses/i2c-piix4.c
[pciidsh]: https://github.com/torvalds/linux/blob/master/include/linux/pci_ids.h
[vmd]: https://github.com/torvalds/linux/blob/master/drivers/pci/controller/vmd.c
[machcfg]: https://github.com/torvalds/linux/blob/master/sound/soc/amd/mach-config.h
[raven]: https://github.com/torvalds/linux/blob/master/sound/soc/amd/raven/pci-acp3x.c
[rn]: https://github.com/torvalds/linux/blob/master/sound/soc/amd/renoir/rn-pci-acp3x.c
[yc]: https://github.com/torvalds/linux/blob/master/sound/soc/amd/yc/pci-acp6x.c
[ps]: https://github.com/torvalds/linux/blob/master/sound/soc/amd/ps/pci-ps.c
[acp63]: https://github.com/torvalds/linux/blob/master/sound/soc/amd/ps/acp63.h
[acppci]: https://github.com/torvalds/linux/blob/master/sound/soc/amd/acp/acp-pci.c
[acpcfg]: https://github.com/torvalds/linux/blob/master/sound/soc/amd/acp-config.c
[sofacp]: https://github.com/torvalds/linux/blob/master/sound/soc/sof/amd/acp.h
[sofrn]: https://github.com/torvalds/linux/blob/master/sound/soc/sof/amd/pci-rn.c
[sofrmb]: https://github.com/torvalds/linux/blob/master/sound/soc/sof/amd/pci-rmb.c
[sofacp63]: https://github.com/torvalds/linux/blob/master/sound/soc/sof/amd/pci-acp63.c
[sofacp70]: https://github.com/torvalds/linux/blob/master/sound/soc/sof/amd/pci-acp70.c
[cb]: https://github.com/coreboot/coreboot/blob/main/src/include/device/pci_ids.h
[lh15b9]: https://linux-hardware.org/?id=pci:1022-15b9
[lhwpop]: https://github.com/linuxhw/DevicePopulation
[tufb650]: https://www.asus.com/motherboards-components/motherboards/tuf-gaming/tuf-gaming-b650-plus-wifi/techspec/
