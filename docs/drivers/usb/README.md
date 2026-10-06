# USB and the xHCI host controller

How NONOS drives USB: one xHCI capsule owns every USB controller, and class capsules above it bind keyboards, mice and disks.

## What is supported

| Hardware | Matched by | Capsule | State in 0.9.2 |
|---|---|---|---|
| xHCI host controller, USB 2 and USB 3 ports | PCI class 0Ch, subclass 03h, prog-if 30h | `driver.xhci0` | Served |
| Intel Thunderbolt and USB4 xHCI controllers | 15 Intel device ids | `driver.xhci0` | Served after the chipset's controller, best effort |
| EHCI, OHCI and UHCI host controllers | prog-if 20h, 10h and 00h | none | Not supported: no driver |
| Keyboards and mice in boot protocol | interface class 03h | `driver.usb_hid0` | Served. See [USB keyboards and mice](hid.md). |
| USB sticks and disks, Bulk-Only | interface class 08h | `driver.usb_msc0` | Served. See [USB mass storage](../storage/usb-mass-storage.md). |
| Hubs | interface class 09h | `driver.usb_hid0` | The hub comes up; devices behind it are not reached. See [USB hubs](hubs.md). |
| USB network adapters | per adapter | none in the image | Not in the image. See [USB network adapters](../ethernet/usb-net.md). |
| Audio devices, cameras | | none | Not supported: no isochronous transfers |

## How the pieces fit

```mermaid
flowchart LR
    HC[xHCI controller] --- X[driver.xhci0]
    X --- H[driver.usb_hid0]
    X --- M[driver.usb_msc0]
    H --> Ring[kernel input ring]
    Ring --> R[input_router]
    M --> B[kernel block layer]
```

`driver.xhci0` owns the xHCI controllers: their registers, rings and DMA. It knows nothing of keyboards or disks. The class [capsules](../../overview/glossary.md#capsule) `driver.usb_hid0` and `driver.usb_msc0` read descriptors and run transfers through it. The kernel lets only those two send to it, because the controller carries raw transfers to every device behind it (`src/services/registry/held_table.rs:30-32`, `driver.xhci0`). Keyboard and mouse events go to the kernel input ring and from there to `input_router`; disk sectors go to the kernel block layer.
