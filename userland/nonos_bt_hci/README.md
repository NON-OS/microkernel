# nonos_bt_hci

The host side of the Bluetooth Host Controller Interface for an LE-only host
(Decision 21: LE only, HID over GATT). Written from the Bluetooth Core
Specification 5.4. Every fact in the source cites its volume, part and
section. It builds on `nonos_bt_wire` for the packet formats.

| Module | What it holds |
|---|---|
| `transport` | `Transport`: whole HCI packets each way, polled, never waited on. |
| `usb` | HCI over USB (Vol 4, Part B). Finds the Bluetooth interface (class E0/01/01) and its three endpoints in a configuration descriptor. Joins interrupt and bulk transfers into whole packets. `UsbHci` implements `Transport` over `UsbPipes`, which a capsule backs with its USB controller driver. |
| `host` | `Host`: commands with command flow control and a poll budget (Vol 4, Part E, 4.4), events met while waiting kept in a bounded queue, bring-up, the event masks, passive scanning, and ACL data with buffer credits (Vol 4, Part E, 4.1.1). |

Bring-up uses standard commands only, in this order:
1. Reset.
2. Read Local Supported Features. A controller without LE stops here.
3. Read Local Version Information.
4. Read BD_ADDR.
5. LE Read Buffer Size, or Read Buffer Size when the LE buffers are shared.
6. Set Event Mask.
7. LE Set Event Mask.

No vendor command is sent. A controller that needs vendor setup or
firmware before it answers these stops at the first one it fails, with
that command's status or a timeout.

Scanning is passive only. The controller listens and sends no scan
request, so looking for devices puts nothing of ours on the air.

Every wait is bounded. A command waits at most `POLL_BUDGET` polls with a
`pause` between empty ones (about two seconds in a capsule). The event
queue holds at most `QUEUE_MAX` events and drops the oldest past that,
counted in `dropped()`. An incoming packet over its bound is refused.

The proofs live in `nonos_bt_hci_proofs`. They drive the whole stack (host,
USB transport, joiner) against a simulated controller behind `UsbPipes`.
