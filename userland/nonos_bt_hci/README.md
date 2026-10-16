# nonos_bt_hci

The host side of the Bluetooth Host Controller Interface for an LE-only host
(Decision 21: LE only, HID over GATT). Written from the Bluetooth Core
Specification 5.4. Every fact in the source cites its volume, part and
section. It builds on `nonos_bt_wire` for the packet formats.

| Module | What it holds |
|---|---|
| `transport` | `Transport`: whole HCI packets each way, polled, never waited on. |
| `usb` | HCI over USB (Vol 4, Part B). Finds the Bluetooth interface (class E0/01/01) and its three endpoints in a configuration descriptor. Joins interrupt and bulk transfers into whole packets. `UsbHci` implements `Transport` over `UsbPipes`, which a capsule backs with its USB controller driver. |
| `uart` | HCI over a UART, for controllers wired to a serial port. `h4` is the UART transport (Vol 4, Part A): a packet indicator, then the packet, cut by its own header's length; an octet that is no indicator ends the stream. `h5` is the Three-wire UART (Vol 4, Part D): SLIP framing, the four-octet header with its checksum, the optional 16-bit CRC, link establishment (SYNC, CONFIG and their responses), and reliable packets with sequence numbers, a sliding window and acknowledgement. Both implement `Transport` over `UartPort`, which a capsule backs with its serial driver. No vendor command and no firmware patch: no public specification covers either. |
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
9. LE Read Maximum Data Length (7.8.46), only when the LE features have
   LE Data Packet Length Extension (bit 5). A refusal, or an answer
   outside 27 to 251 octets and 328 to 17040 µs, leaves every link at the
   default 27 octets.

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
Once a link is up, a controller that answered step 9 is asked with LE Set
Data Length (7.8.33) for the smaller of its maximum and 251 octets in
2120 µs, the time 251 octets take on the 1M PHY, so an ATT exchange of 247
octets fits one PDU. A refusal leaves the link as it is. Events met during
either wait stay queued for `next_event`. `update_link`
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

Over the Three-wire UART, `establish` runs link establishment for at most
a given number of looks. A reliable packet is sent again every `retx_ms`
until acknowledged, and the link counts as lost when one has waited
`LOST_MS` (one second) with nothing acknowledged, or when the controller
starts the link over under a host using it. Either is `HciError::NoLink`.

Every wait is bounded. A command waits at most `POLL_BUDGET` polls with a
`pause` between empty ones (about two seconds in a capsule). The event
queue holds at most `QUEUE_MAX` events and drops the oldest past that,
counted in `dropped()`. An incoming packet over its bound is refused.

A controller is not taken at its word:
- **ACL buffers are held per link.** Number of Completed Packets gives
  back no more than the link it names holds, so a count for packets never
  sent frees nothing. A link that ends gives back everything it held,
  with Disconnection Complete or with `link_gone`, when Disconnect finds
  the link already gone.
- **A lost controller stops the host.** A Hardware Error (7.7.16), or a
  completed Reset (7.3.2) this host did not send, stops it with
  `HciError::Reset` wherever it is met; the caller brings the controller
  up again. While its own Reset is waited for, both pass. Events from
  before that Reset are cleared.
- **`link_known` asks whether the controller still has a link,** with
  Read RSSI (7.5.4). Unknown Connection Identifier means it does not.

The proofs live in `nonos_bt_hci_proofs`. They drive the whole stack (host,
USB transport, joiner) against a simulated controller behind `UsbPipes`.
