# capsule_driver_ax88179: binding, framing and link

How the capsule takes an AX88179 or AX88178A, frames its traffic and
follows its link, and what it says on the log. The contract is in
[README.md](README.md), NNET in [PROTOCOL.md](PROTOCOL.md).

## Binding

1. The vendor:product is in Linux's products[] (13 entries, `products.rs`);
   the ASIX parts Linux gives cdc_ether (0b95:2790, 0b95:2791) are not.
2. The configuration Linux would choose (`usb_choose_configuration`) holds
   an interface of class ff/ff/00 with a bulk IN and a bulk OUT pipe.
3. Pipes to driver.xhci0, SET_CONFIGURATION, SET_INTERFACE (answer ignored).
4. `ax88179_reset` in order: PHY power and clocks (200 and 100 ms waits),
   auto detach when the EEPROM asks, the run's drawn station address
   written to AX_NODE_ID, bulk IN aggregation, pause levels, checksum
   offload off, AX_RX_CTL, monitor mode, default medium, EEE off,
   auto-negotiation restarted.

The drawn address goes where Linux `ax88179_set_mac_addr` writes one, so
the chip's own unicast filter (AX_RX_CTL_AP) matches it. The factory value
in AX_NODE_ID, which Linux `ax88179_get_mac_addr` reads, is never read, so
a chip without one binds all the same. AX_RX_CTL is Linux's bit for bit:
CRC errors dropped, the 2-byte IP alignment header, start, directed, all
multicast and broadcast. Checksum offload stays off, where Linux turns it
on: the stack checks every checksum itself. The LED setup from the EEPROM
is left out, so the chip's own LED defaults stay.

## Framing

TX: an 8-byte header (length, zero MSS word) before each frame; a transfer
whose length is a multiple of 512 or of the pipe's packet size gets one
padding byte and the header's padding flags, as Linux sends it.

RX: one bulk IN of at most 4096 bytes carries several frames, found from
the header at its tail; the aggregation size is cut to fit that, Linux's
timers kept. Every count, offset and length is checked; a malformed
transfer is dropped whole, and a frame with an error bit, a runt or one
past 1514 bytes is skipped. Frames are queued and handed up one per
receive; the pipe is polled only when the queue is empty.

## Link

No interrupt endpoint is configured, so the PHY is read on the clock, at
most once a second, after a request is answered (`Nic::tick`). `link_up`
answers from that reading. On a new link, or one the partner renegotiated,
the medium mode and bulk IN row are set as `ax88179_link_reset` sets them,
after the receiver is restarted until the USB TX FIFO reads empty (at most
ten looks across 100 ms). A reset that fails or does not settle leaves the
link down and is tried again no sooner than 5 s later. Three PHY reads
failed in a row and the link reads down; each failed read or refused reset
step counts toward the device being taken as gone.

## Log

`log usbnet` shows `[usbnet ax88179]` lines when Debug is granted:
`started, looking for a device`; each port tried and why it was not taken,
or `port N: bound` for the one taken;
a failed bind step by name with its errno, the only line that names the
device's vendor:product; `port N up, drawn station address programmed`
when bound; and `device stopped answering, given back`. Without entropy,
`no entropy for a station address, no device bound until there is` is said
once. No line carries a station address.
