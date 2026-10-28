# capsule_driver_rtl8153: binding, framing and link

How the capsule takes an adapter, brings the chip up, frames its traffic
and follows its link, and what it says on the log. The contract is in
[README.md](README.md), NNET in [PROTOCOL.md](PROTOCOL.md). Behaviour is
read from Linux v6.12 `drivers/net/usb/r8152.c`; where this driver does
less than Linux, it says so.

## Binding

1. The device's ids are one of sixteen, each in Linux v6.12's r8152 table:
   Realtek 0bda:8153; Lenovo 17ef:3062, 3069, 3082, 7205, 720c, 7214,
   721e, a387; Microsoft 045e:07ab, 07c6, 0927; TP-Link 2357:0601; Linksys
   13b1:0041; Samsung 04e8:a101; Nvidia 0955:09ff (`src/rtl/adapters.rs`).
   Any other device is not this driver's and is not spoken to.
2. The first configuration whose first interface is of the vendor class
   (FFh) at alternate 0, with a bulk IN and a bulk OUT, as Linux chooses
   it. An adapter without one is left to cdc_ecm.
3. The chip's version, read before configuring as Linux reads it: the PLA
   dword at 0xe610, upper half masked by 0x7cf0, up to three times. Codes
   5c00, 5c10, 5c20, 5c30 are the RTL8153 and 6000, 6010 the RTL8153B; an
   RTL8152, RTL8153C, RTL8156, RTL8156B or unknown code is refused by name
   with ENODEV and the device never configured.
4. Every interface of the configuration held with the host; one another
   driver holds stops the bind. The bulk pipes go to the host, then
   SET_CONFIGURATION.
5. Wake (`src/rtl/wake`): U1/U2 and U2P3 off, the chip's settings loaded
   (bounded wait), the PHY out of power-down and at LAN-on (bounded),
   power cut, UPS and runtime-suspend wake off, receive aggregation off
   so a transfer holds one frame, and the version-specific settings Linux
   makes for each code.
6. The drawn station address written into the address register inside
   its unlock window.
7. Autonegotiation: 10/100 half and full, pause, 1000 full; the PHY reset.
8. Start (`src/rtl/start.rs`): ALDPS off, receive gated, every frame
   refused, the MAC and buffer manager reset, the buffer link list
   rebuilt, VLAN tags kept in frames, 1522-byte frames, the FIFO
   thresholds.
9. The link word read once.

Not done, though Linux does it: firmware patches, the PHY's analog and
EEE tuning, ALDPS, U1/U2, U2P3 and MAC clock slow-down turned back on,
the dynamic burst setting of the 5c20 and 5c30, and the link-change poll
bits. An adapter that also offers CDC-ECM may be taken by cdc_ecm first,
when it reaches the port before this driver; either serves it.

## Framing

Receive: one bulk IN of 4096 bytes at a time. Each packet is a 24-byte
descriptor whose first dword's bits 14:0 give the packet's length with its
FCS, then the packet; the next descriptor sits at the next multiple of 8.
A length under 60 or running past the bytes that came ends the walk of
that transfer; a frame over 1514 bytes is dropped; a transfer that fills
the buffer is let go with its rest. Each is counted. The chip's own
early-size limit keeps any transfer it gathers inside the buffer too.

Send: one frame per bulk OUT behind an 8-byte descriptor, its first dword
the length with bits 31 and 30 (first and last segment), its second 0.

## Link

The link word (PLA 0xe908) is read one request in 16. Link up turns
receive and transmit on, sets the inter-frame gap for the speed, and sets
the filter to unicast to the drawn address, broadcast and every multicast
group; only then does the link read up. Link down reads down at once, then
refuses every frame, gates receive, lets the FIFOs drain (bounded) and
resets the MAC and buffer manager.

## Log

`log usbnet` (or `log rtl8153`) shows `[usbnet rtl8153]` lines when Debug
is granted: `started, looking for a device`; each port tried; a failed
bind step by name with its errno, the only line that names the device's
vendor:product (`chip is an RTL8156, not this driver's errno -19`, `PHY
never came on errno -110`); `port N up, drawn station address
programmed`; and `device stopped answering, given back`. No line carries
a station address or a serial number.
