# The real-hardware kit

What to boot, how, what to bring back and where to send it, for the owner and anyone
testing NONOS on their own machines. Each boot closes rows in the
[coverage map](coverage.md); each report becomes a [support matrix](MATRIX.md) row or a fix.

## The machines, in order

Boot as many as you can reach, first to last. One of each kind is worth more than three of
one kind.

| # | Machine | What it proves | Coverage rows |
|---|---|---|---|
| 1 | The bench Gemini Lake laptop | nothing regressed since 6 October | the whole matrix |
| 2 | Any Intel laptop, 8th to 10th gen | AX201/9560 Wi-Fi, LPSS touchpad, the dword ACS | wifi, input, platform |
| 3 | Any Intel laptop, 11th gen or later | Intel VMD storage, AX211 Wi-Fi | storage, wifi |
| 4 | An Intel desktop, 300 to 800-series board | I219 or I225/I226, Realtek ALC, xHCI, hubs | net, audio, usb |
| 5 | An AMD desktop, 400 to 800-series board | RTL8111/8125, AMD-Vi, AHCI on the chipset | net, platform, storage |
| 6 | An AMD laptop | MediaTek Wi-Fi, AMDI0010 touchpad, EC and battery | wifi, input |
| 7 | A rack or tower server | I350, BCM5720 or X710, SATA or NVMe | net, storage |

Every machine is booted twice: once with VT-d or AMD-Vi on in the firmware setup, and once
with it off. That second boot is the no-IOMMU posture: the boot log must carry
`[IOMMU] posture=dma-unconfined`, About's DMA row must read red, the splash must say
`device DMA not confined by hardware` in red, and the compositor's band must have a red edge.

## The stick

1. Build the image with the real drivers: `make dev-image PROFILE=standard`. The image is
   `target/dev/tree/target/release/standard-dev/nonos.img`.
2. Write it and check the write: [Write a USB stick](../install/usb-stick.md). Note the
   commit (`git rev-parse --short HEAD`); a report without it cannot be placed.
3. A second stick, or the same one rewritten, with `make dev-image PROFILE=hardened` for one
   boot per machine with the IOMMU off: it must refuse device claims and say so.
4. The development image seals every capsule with throwaway keys, so it carries e1000e,
   igb and the hub driver, which a sealed Standard image cannot until the owner seals them.
   For an I225/I226 board (machine 4), igc is wired but not listed: before step 1 set
   `features = ["nonos-capsule-driver-igc"]` in `nonos.toml`, and put it back afterwards.
   Never seal such an image for use.

## On each machine

[The checklist](real-hardware-checklist.md) is one page per machine: fill it in as you go.

## Sending it back

Put the checklist, the report file and the photos in one folder per machine, named by the
machine's number above and the date, and send it to the maintainers. Never include serial
numbers, MAC addresses, network names, passwords or anything from your own files; the
[report rules](report.md#what-not-to-include) say what to leave out. The maintainers file each
report under `HANDOFF/received/`, and this session turns it into matrix rows and fixes.
