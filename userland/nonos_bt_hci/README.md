# nonos_bt_hci

The host side of the Bluetooth Host Controller Interface for an LE-only host
(Decision 21: LE only, HID over GATT). Written from the Bluetooth Core
Specification 5.4. Every fact in the source cites its volume, part and
section. It builds on `nonos_bt_wire` for the packet formats.

| Module | What it holds |
|---|---|
| `transport` | `Transport`: whole HCI packets each way, polled, never waited on. |
| `usb` | HCI over USB (Vol 4, Part B). Finds the Bluetooth interface (class E0/01/01) and its three endpoints in a configuration descriptor. Joins interrupt and bulk transfers into whole packets. `UsbHci` implements `Transport` over `UsbPipes`, which a capsule backs with its USB controller driver. |
| `host` | `Host`: commands with command flow control and a poll budget (Vol 4, Part E, 4.4), events met while waiting kept in a bounded queue, bring-up, the event masks, passive scanning, and ACL data with buffer credits (Vol 4, Part E, 4.1.1). Opening a link as central (`connect`, Vol 4, Part E, 7.8.12 and 7.8.13), encrypting it (`encrypt`, 7.8.24), and new timing for it once a peripheral's request is accepted (`update_link`, 7.8.18). |

Bring-up uses standard commands only, in this order:
1. Reset.
2. Read Local Supported Features. A controller without LE stops here.
3. Read Local Version Information.
4. Read BD_ADDR.
5. LE Read Local Supported Features (Vol 4, Part E, 7.8.3).
6. LE Read Buffer Size, or Read Buffer Size when the LE buffers are shared.
7. Set Event Mask.
8. LE Set Event Mask. The LE Directed Advertising Report (bit 10) is
   asked for only when the LE features have Extended Scanner Filter
   Policies (Vol 6, Part B, 4.6, bit 7).

No vendor command is sent. A controller that needs vendor setup or
firmware before it answers these stops at the first one it fails, with
that command's status or a timeout.

After bring-up, `connect` sends LE Create Connection for one named peer
and waits for LE Connection Complete. If the peer is not heard within the
poll budget, LE Create Connection Cancel stops the attempt. `encrypt`
sends LE Enable Encryption with the LTK from `nonos_bt_smp` and waits for
Encryption Change. The LTK comes most significant octet first, as the
Security Manager hands it over, and goes on the wire least significant
octet first. Rand and EDIV are zero, as LE Secure Connections has them.
Events met during either wait stay queued for `next_event`. `update_link`
sends LE Connection Update with timing a peripheral asked for and this
host accepted; it waits only for Command Status, and LE Connection Update
Complete reaches the caller as any other event.

Scanning is passive only. The controller listens and sends no scan
request, so looking for devices puts nothing of ours on the air. The
filter policy (Vol 4, Part E, 7.8.10; Vol 6, Part B, 4.3.3) is 0x02 on a
controller with Extended Scanner Filter Policies and 0x00 on any other.
Under 0x02, directed advertising to a resolvable private address the
controller cannot resolve reaches the caller as an LE Directed
Advertising Report, and the caller decides whether that address is its
own.

Scans and connections are made as the public address until
`use_random_address` gives the controller a random one with LE Set Random
Address (Vol 4, Part E, 7.8.4); from then on Own_Address_Type is random in
both. The reserved kind and a random part Vol 6, Part B, 1.3.2.1 or
1.3.2.2 forbids are refused unsent, and an address the controller refuses
leaves the public one in use. The controller refuses a change while it
scans or connects, so the caller changes it between those.

Every wait is bounded. A command waits at most `POLL_BUDGET` polls with a
`pause` between empty ones (about two seconds in a capsule). The event
queue holds at most `QUEUE_MAX` events and drops the oldest past that,
counted in `dropped()`. An incoming packet over its bound is refused.

The proofs live in `nonos_bt_hci_proofs`. They drive the whole stack (host,
USB transport, joiner) against a simulated controller behind `UsbPipes`.
