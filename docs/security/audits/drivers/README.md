# Driver audit, 2026-10-11

The record of the driver audit of 2026-10-11. It covers the kernel's DMA
and interrupt isolation, the device-grant broker and PCI stack, IPC admission to driver
endpoints, the input and entropy paths, and 35 audit entries: 3 kernel subsystems (IOMMU,
grant broker, PCI stack) and 32 drivers and libraries (storage, USB, input, wired and USB
network, audio, graphics, SD, entropy, and Wi-Fi with its shared core and client).

## Method

1. Audit, in 11 groups. Per driver: readiness on the audit's scale (skeleton, partial, near),
   a summary, test coverage, TCB and privacy, what is missing, and findings with file, line,
   category, severity, title, detail, evidence and fix. 398 findings.
2. Adversarial verification of every critical and high (62), and of 36 mediums and 9 lows
   besides: 107 findings re-traced through the code and the kernel paths around it, with
   QEMU probes and standalone builds where needed. Each was confirmed, refuted or re-rated
   with corrected reasoning, and a verdict overrides the audit. The verifiers found 4 new
   defects; the kernel reply route was then verified on its own.
3. Grouping into 56 fixes, each with a label: policy P1 to P9 and capsule rules C1 to C8;
   this audit's own and name no ticket outside it.

## Counts

| Severity | Audit | After verification |
|---|---|---|
| critical | 6 | 4 |
| high | 56 | 29, 1 of them new |
| medium | 202 | 210, 2 new |
| low | 134 | 150, 1 new |
| refuted | | 9 |
| total | 398 | 402 |

Re-rated: 2 critical to high, 24 high to medium, 4 high to low, 2 medium to high, 13 medium
to low. Refuted: 4 high, 3 medium, 2 low. New: the reply route to held drivers (high), the PCI
boot pass that sets Bus Master on every device (medium, found twice), rtl8821ce's retune after
a join (medium), and the kernel input ring keeping drained keys (low). Findings a verifier met
again (xHCI slot ownership, the USB NIC InputSource mask) are not counted twice; virtio_gpu's
legacy path, split from its critical, was refuted.

## Notation

- Per driver: a heading with its readiness, then one line on what it is and its path root.
- Bullets: every finding the audit or its verifier rated medium or above: severity after
  verification ("was X (reason)" when re-rated, "refuted" with the reason), file:line, the
  defect, and "->" the fix that covers it, by its label. "low:" groups the rest per driver; a
  label in brackets marks a low that a fix covers.
- "-> wave 4": a finding in the drivers that no fix names yet; a later round of driver work
  covers them in general terms, fixes to come ("named there": rtsx SD, bga surface, ps2 touchpads).
  "-> none": a kernel or Wi-Fi finding no fix names. "near X" is the closest fix; "in part"
  means a fix's tests or change cover some of it.

## Areas

| Area | File | Audit c/h/m/l | Verified c/h/m/l, refuted |
|---|---|---|---|
| IOMMU | [iommu.md](iommu.md) | 3/2/12/9 | 2/1/14/9, 0 |
| Grants, PCI boot pass, held endpoints and reply route, input caps, entropy | [kernel-grants-and-ipc.md](kernel-grants-and-ipc.md) | 0/4/10/5 | 0/3/9/9, 1 (3 new) |
| xHCI | [xhci.md](xhci.md) | 0/6/13/6 | 0/3/16/6, 0 |
| usb_hid, i2c_hid, i2c_pci, ps2_input | [usb-hid-and-input.md](usb-hid-and-input.md) | 0/3/24/14 | 0/0/25/16, 0 |
| nvme, ahci, virtio_blk, usb_msc, nonos_blk_client | [storage.md](storage.md) | 0/4/32/31 | 0/4/28/32, 3 |
| e1000, e1000e, igc | [wired-nics-1.md](wired-nics-1.md) | 0/3/18/6 | 0/0/16/9, 2 |
| rtl8139, rtl8169, virtio_net | [wired-nics-2.md](wired-nics-2.md) | 1/3/15/11 | 0/2/15/11, 2 |
| nonos_usbnet, ax88179, cdc_ecm, cdc_ncm, rndis, rtl8153 | [usb-nics.md](usb-nics.md) | 0/14/19/11 | 0/13/18/13, 0 |
| hda, virtio_gpu, bga, rtsx, virtio_rng capsule and kernel | [media.md](media.md) | 2/8/21/13 | 2/0/26/16, 0 |
| iwlwifi, rtl8821ce | [wifi-1.md](wifi-1.md) | 0/7/24/15 | 0/2/29/15, 1 (1 new) |
| nonos_wifi_core, nonos_wifi_client | [wifi-2.md](wifi-2.md) | 0/2/14/13 | 0/1/14/14, 0 |

## Where the risk is

- Critical: the IOMMU identity domain for every unclaimed device; the broker's fail-open claims where no unit confines a device; virtio_gpu and virtio_rng given IOVAs under a vIOMMU without ACCESS_PLATFORM.
- High, isolation: the reply route to all 15 held drivers; no device scope for Driver and Admin holders; any input driver may drain the keystroke ring; interrupt injection without remapping; xHCI freeing DMA under a running controller (D9); virtio_net rings past their grant (D1).
- High, function: nvme wedged by one late error (D5); ahci stale ERR and CLO-only timeouts (D6); xHCI Ring Stopped and hubs (D9); usb_msc probing (D12); virtio_net oversize-frame panic (D1); Wi-Fi never detects link loss; the USB NICs' MACs, client contract, op 6 and rtl8153 link (D10, D11).
