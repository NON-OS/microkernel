# capsule_driver_ax88179: binding, framing and link

How the capsule takes an AX88179 or AX88178A, frames its traffic and
follows its link, and what it says on the log. The contract is in
[README.md](README.md), NNET in [PROTOCOL.md](PROTOCOL.md).

ASIX publishes no programming reference for these chips. Every register,
value and framing rule here is as Linux v6.12's
`drivers/net/usb/ax88179_178a.c` uses it, read for behaviour only; the code
is written here, with its own structure and names (`src/ax/regs.rs` says
so where the numbers are). What is marked unverified below has not been
seen on a chip.

## Devices

The 13 vendor and product pairs Linux v6.12 binds (`src/ax/ids.rs`):
ASIX 0b95:1790 (AX88179) and 0b95:178a (AX88178A), Cypress GX3 04b4:3610,
D-Link DUB-1312 2001:4a00, Sitecom 0df6:0072, Samsung 04e8:a100, Lenovo
OneLinkDock 17ef:304b, Belkin B2B128 050d:0128, Toshiba 0930:0a13, Magic
Control Technology U3-A9003 0711:0179, and Allied Telesis AT-UMC2000,
AT-UMC200 and AT-UMC2000/SP (07c9:000e, 000f, 0010). Linux v6.12 lists no
Dell product id; a Dell adapter is taken only if it shows one of these. A
listed device is taken only in a configuration with an
interface of class FFh, subclass FFh, protocol 0 at alternate 0 with a
bulk pair; one that offers only a class configuration (the AX88179A and
B are said to offer CDC NCM, unverified) is left to cdc_ncm or cdc_ecm.

## Binding

1. Every interface of the configuration is held with the host's claim
   table; one another driver holds stops the bind, never taken over.
2. The bulk pipes go to the host, then SET_CONFIGURATION.
3. The bring-up (`src/ax/plan.rs`), each register a vendor request
   (bRequest 01h MAC, wValue the register, wIndex the width; 02h PHY,
   wValue the PHY address 3, wIndex the register): the PHY reset, then
   powered, 500 ms; the clocks on, 200 ms; the drawn station address into
   the station register (10h); the bulk-in queue (2Eh) set to aggregate at
   most 4 KiB, so one bulk IN of driver.xhci0's 4096 bytes holds a
   transfer; the pause thresholds; checksum offload off both ways; the
   multicast table open (every group, as the stack keeps no list); receive
   started for directed, broadcast and every group, with the 2-byte
   alignment pad; the medium gigabit full duplex until the PHY says; EEE
   off; autonegotiation restarted. 21 requests and 700 ms of fixed waits,
   inside the 15 s bind budget. A refusal names its step.

The factory address in the chip's station register is never read: the
drawn one replaces it before receive is started. Not done, as this driver
does not need them: wake-on-LAN, the LED and auto-detach settings Linux
reads from the EEPROM, and checksum or segmentation offload.

## Framing

Receive: one bulk IN of 4096 bytes. The transfer ends in a 32-bit trailer
(entry count, table offset); the table's entries give each frame's length
with its 2-byte pad and its CRC-error and drop marks; frames sit from the
start of the transfer, each padded to 8 bytes. The trailer is refused, and
the transfer dropped whole, when the table does not lie between the frames
and the trailer; an entry naming bytes past the start of the table ends the
table; a marked frame, one under 16 bytes with its pad, or one longer than
a full frame and a 4-byte FCS is counted and let go; one up to 4 bytes over
a full frame is cut to 1514 (whether the chip counts the FCS is
unverified). At most 64 frames are taken from one transfer. Frames go up
one a request; the next transfer is read once all of them have. A transfer
that fills the 4096 bytes has not ended and is let go with its rest.

Send: an 8-byte header, the frame's length and a second word, then the
frame. When the transfer would end on a multiple of 64 bytes or of the
packet size, one padding byte follows and the second word carries the mark
(bits 31 and 15) that says so, since the bus cannot send a zero-length
packet.

## Link

Down from bind. While down, the PHY's status register (11h) is read every
second request: link, duplex and speed. On a link, or a new speed or
duplex, the receiver is restarted until the chip's FIFO status (vendor
request 81h) reads drained, at most 4 rounds and never waited on, the
bulk-in queue set again, and the medium written for that speed; the link
is up only once all of it was taken. While up, the 8-byte report on the
interrupt endpoint (bit 16, the link) is read every eighth request, and the
PHY when a report says down, on one look in sixteen besides, or on every
look when the chip has no such endpoint. A PHY read that fails counts
toward the device being given back; a short report is counted and ignored.

## Log

`log usbnet` shows `[usbnet ax88179]` lines when Debug is granted: the
shared core's `started, looking for a device`; each port tried and why it
was not taken, or `port N: bound`; a failed bind step by name with its
errno, the only line that names the device's vendor:product; when bound,
`port N up, drawn station address programmed`; and `device stopped
answering, given back`. No line carries a station address or a serial
number: the driver reads neither.
