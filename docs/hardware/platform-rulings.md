# Hardware matrix: ACPI, platform, Thunderbolt, fuzzing

Part of [README.md](README.md); evidence words and commonality as defined there.
Counts are `#[test]` items at 1536ccef3.

## ACPI and platform

| Item | Common | Where | State | Evidence |
|---|---|---|---|---|
| Static tables: RSDP, XSDT (RSDT fallback), FADT (v1 to 6, X_ fields), MADT incl. x2APIC, MCFG, HPET, SRAT, SLIT, HMAT, LPIT, WDAT, BGRT, SPCR, DBG2, PCCT, SDEV, DMAR, IVRS | every PC | kernel | done: bad root/FADT checksums warned and used; bad DMAR/IVRS dropped (DMA then refused, P2) | host kernel_proofs 992, seeded sweep of 3000 seeds over 13 tables; booted (q35, IOMMU cells) |
| DMAR, IVRS in the seeded sweep | every PC with an IOMMU | kernel | done: DMAR's walk and DRHD, RMRR, ANDD parsers, IVRS's walk, spans, aliases, specials and IVMD swept; it found an IVHD range END below its START kept as a span (ivhd_kind.rs, fixed) | host kernel_proofs seeded_tables |
| AML interpreter (nonos_aml in capsule_acpi) | every PC | capsule_acpi, **out** (no profile, no capsules.json entry) | partial: written from ACPI 6.5; waits on the owner's seal and the Admin-or-Firmware ruling (f/08) | host nonos_aml_proofs 439, capsule_acpi_proofs 49; never booted |
| Battery `_BIX`/`_BIF`/`_BST`/`_PSR` | every laptop | capsule_acpi | partial: in the capsule only; images show "Battery status unavailable" | host |
| Thermal zones `_TMP`, `_CRT`, `_HOT`, `_PSV`, `_ACx` | every PC | capsule_acpi | partial: as battery | host |
| Fixed-feature power button | most desktops, older laptops | kernel | partial: polled, reaches the desktop; desktop power-off needs capsule_acpi's power.off | **hw** (GLK) |
| AML power button PNP0C0C, lid, hardware-reduced | most laptops since 2015 | capsule_acpi | partial: capsule only | host |
| Power-off `_S5`, reboot (reset register, 8042, CF9) | every PC | kernel | partial: kernel sequence present; nothing asserts it in a boot | host |
| Suspend S3, s2idle | every laptop | capsule_acpi | none: SUSPEND = false (Decision 6); kernel resume NotWired | design |
| ACPI hotplug, docks | common | capsule_acpi | partial: native pciehp off (NATIVE_HOT_PLUG_ON = false) | host |

## Isolation groups (VT-d on): the audit

The broker gives an isolation group to one capsule (K11b,
src/hardware/broker/confine/join_group.rs). A function of a multi-function
device shares a group with its sibling functions unless the claimed
function's own ACS redirects (acs_path.rs hop 0; an Intel root complex
integrated endpoint on bus 0 counts as redirecting, acs_layout.rs). Since
H-03 the relation holds both ways: a claim is also refused when a device
another capsule holds has it in its own group (port_only.rs `blocked_by`),
so no outcome depends on which side claims first. Booted in QEMU (TCG, throwaway key, not a sealed image), before and after H-03 on q35 with VT-d, alone and with an e1000, EHCI, a USB stick, or HDA beside an e1000: the same refused lines both ways (0, but for the HDA beside the e1000, refused for its group), and no group read as unreadable. Since
H-02 a **port-only** claim (a capsule holding neither Dma nor Mmio, with
Memory Space and Bus Master cleared at the claim and kept off) neither
joins nor blocks a group, but across a bridge to a shared conventional bus
(confine/port_only.rs). smbus is the one port-only driver claiming a PCH
function. Claims follow the spawn order (spawn_plan/orchestrator.rs): disks
(ahci, nvme), the USB hosts (xhci, ehci and companions), then the bus
drivers (iwlwifi, rtl8821ce, gpio, i2c_pci,
i2c_hid, smbus, spi), input, NICs (e1000, e1000e, igb, igc, ixgbe,
realtek), the USB class drivers, then hda, sdhci, ufs. Capsules run concurrently, so
spawn order is the usual claim order, not a guarantee: the second guard
only.

With no remapping unit (VT-d off, an AMD machine, a failed bring-up), or
on a device no unit in service covers, a PCI claim is refused unless it
is port-only (H-02b). A port-only claim goes ahead with no domain and no
DMA grant (confine/decide.rs `Verdict::NoDma`), and says `port-only with
no remapping unit over it: Bus Master off, no DMA grant`. A capsule
holding Dma or Mmio stays refused. So on a machine with VT-d off smbus can
read SPD, and ahci, nvme, hda, xhci and the NICs cannot. smbus is in no
image until the owner's seal (item 52). Booted in QEMU (q35, VT-d off, TCG, throwaway key, smbus added to the
image in a scratch copy only): smbus reads SPD at 8 of 8 addresses beside that line, and the refused lines
fall from 85 to 73, every one of them still `no remapping unit confines it`.
A count of refused lines is not a fixed number: drivers retry their claims,
so the same image gives a few lines more or fewer from boot to boot (71, 73
and 74 in three boots of one image). Read a count for which lines appear and
which claims stop being refused, not for its exact value.

| Chipset | Group (functions, driver) | Who claims, who is refused | Evidence |
|---|---|---|---|
| QEMU q35 (ICH9) | 1f.0 LPC (none), 1f.2 SATA (ahci, DMA), 1f.3 SMBus (smbus, port-only) | both: smbus is port-only; ahci also spawns first | booted before H-02 (privacy session): `[VT-D] refused: 00:1f.3 reaches it through 00:1f.2 without ACS; driver.smbus0 holds it`, then `driver.ahci0 status 6`. After H-02, booted (QEMU TCG, VT-d, throwaway key, not a sealed image), in both orders (ahci first as shipped; smbus first in a scratch copy): 0 `[VT-D] refused` lines; smbus reads SPD at 8 of 8 addresses; ahci serves port 0 (1032192 sectors) after IDENTIFY, by DMA through the IOMMU. Those boots are amnesic: the store is the loader's copy, so no disk was mounted. Without VT-d every PCI claim but a port-only one is refused (H-02b): smbus reads SPD there, ahci stays refused |
| Intel 6- to 9-series PCH (2011-2015) | 1f.0 LPC, 1f.2 SATA (ahci, DMA), 1f.3 SMBus (smbus, port-only), 1f.6 thermal (none); HDA at 1b.0 and LAN at 19.0 alone | both: as q35 | inferred from the rule and published lspci layouts |
| Intel 100- to 700-series PCH (2015-2025) | 1f.0 LPC/eSPI, 1f.2 PMC (none), 1f.3 HDA (hda, DMA), 1f.4 SMBus (smbus, port-only), 1f.5 SPI flash (refused to every capsule, broker/firmware_flash.rs), 1f.6 I219 LAN (e1000e, DMA); SATA at 17.0 alone | smbus always. **hda and e1000e both DMA: a real conflict.** e1000e spawns first (NICs before hda), so with e1000e in an image the machine has wired network and **no audio**; hda logs nothing, its claim refused with `[VT-D] refused: 00:1f.6 reaches it through 00:1f.3 without ACS; driver.e1000e0 holds it`. e1000e is in no image today, so no image shows it yet | inferred. The Lead's ruling: no split without a sourced statement that the PCH routes no peer requests between 1f.x functions; the network wins by default; a user choice of audio is the owner's call. A device cell with HDA and an Intel NIC as sibling functions is what moves this row to booted |
| Intel CNVi laptops (Cannon Lake on: AX201/AX211 at 14.3) | 14.0 xHCI (xhci, DMA), 14.2 shared SRAM (none), 14.3 CNVi Wi-Fi (iwlwifi, DMA, an RCiEP) | CNVi Wi-Fi shares an isolation group with xHCI (inferred); USB wins; Wi-Fi needs a USB adapter or a non-CNVi card. Since H-03 the group holds both ways, so the refusal no longer depends on the claim order (before, xHCI first then iwlwifi was admitted, with xHCI's requests to 14.3 unredirected), and the USB hosts spawn before the Wi-Fi: iwlwifi is refused with `[VT-D] refused: 00:14.0 reaches it through 00:14.0 without ACS; driver.xhci0 holds it` | inferred from the rule; the owner is asked for `lspci -vv` of 14.0 and 14.3 and the iommu_groups listing from a CNVi laptop |
| Intel Gemini Lake, Apollo Lake | 1f.0 LPC, 1f.1 SMBus (smbus, port-only); HDA at 0e.0, SATA at 12.0 alone | both | inferred |
| AMD FCH | 14.0 SMBus (smbus, port-only), 14.3 LPC (none) | smbus alone in practice | inferred |

## Thunderbolt and USB4

| Item | Common | State | Evidence |
|---|---|---|---|
| Thunderbolt 3/4, USB4 (NHI, connection manager) | common on 2018-2025 laptops | none: xhci only orders Intel TCSS xHCIs last; USB-C ports' USB works through the TCSS xHCI where firmware set the tunnels up | design |

## Fuzzing of device data and firmware tables

Seeded or hostile sweeps exist for: i2c_hid descriptors, usb_hid, xhci
event ring, usb_msc, uas, ccid, ctaphid, uvc, rndis, cdc_ncm, usbnet,
usb_hub, rtsx, emmc, ufs, nvme, ahci, igb, igc, ixgbe, virtio_net receive,
Wi-Fi core, EDID, SMBIOS, AML, `_CRS`, 13 static ACPI tables with DMAR
and IVRS, AMD IVHD, the i2c-hid `_DSM` matcher, the HDA codec walk, PS/2,
ARM ACPI, FDT. No cargo-fuzz target covers device data.

No sweep yet: SDHCI CSD/CID; spi_hid frames; smbus and i2c_pci probes;
uhci, ohci and ehci TD retire. B's b/101 queue holds some of these; each
is checked against it before it is taken.

## Gaps this session takes, in order

1. Done: DMAR and IVRS in the seeded sweep, the `_DSM` sweep (item 7),
   and the HDA and PS/2 sweeps.
2. capsule_acpi, suspend and Thunderbolt stay with F and the owner's rulings.
