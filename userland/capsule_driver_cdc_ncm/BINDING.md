# capsule_driver_cdc_ncm: binding, framing and link

How the capsule takes an NCM function, frames its traffic and follows its
link, and what it says on the log. The contract is in
[README.md](README.md), NNET in [PROTOCOL.md](PROTOCOL.md).

## Binding

1. The first configuration, of all the device offers, with a communications
   interface of subclass 0x0D and protocol 0, a Union naming its data
   interface, an Ethernet Networking and an NCM functional descriptor. A
   device with only ECM (subclass 0x06) is left to driver_cdc_ecm.
2. The bulk pipes go to the host, then SET_CONFIGURATION, then
   SET_INTERFACE to the data interface's alternate 0.
3. GET_NTB_PARAMETERS (28 bytes, required); SET_CRC_MODE off when the device
   could add a CRC; SET_NTB_FORMAT 16-bit only when it also offers 32-bit.
4. SET_NET_ADDRESS with the run's drawn station address, still in alternate
   0 as NCM 1.0 asks, when bmNetworkCapabilities bit 1 offers it.
5. A 10 ms pause, then SET_INTERFACE to the alternate with the bulk pipes.
6. SET_NTB_INPUT_SIZE only when the device's dwNtbInMaxSize is past 4096
   bytes (one bulk transfer) or under 2048: the size asked is 4096 at most
   and 2048 at least, in the 8-byte form when bit 5 asks for it, and a
   refusal fails the bind by name. GET/SET_MAX_DATAGRAM_SIZE when bit 3 is
   set.
7. SET_ETHERNET_PACKET_FILTER: directed, broadcast, all multicast, and
   promiscuous too when the device did not take the address.

The iMACAddress string is never read. A device that took SET_NET_ADDRESS
filters on the drawn address; any other has its filter opened so that one
filtering on its own address still passes frames to the drawn one. A
device that does not filter answers with a STALL; the bind goes on and,
without the address set, its line says the address is not confirmed.

## Framing

Out: one frame per NTB16, an NTH16, an NDP16 with one entry and its zero
entry, and the datagram placed on the device's wNdpOutAlignment,
wNdpOutDivisor and wNdpOutPayloadRemainder. A block is at most the
device's dwNtbOutMaxSize, or one byte under a 4096-byte transfer when that
is larger. One that ends on a packet boundary below dwNtbOutMaxSize gets
one zero byte more, counted in wBlockLength, so a short packet ends it
(NCM 1.0, as recalled, unverified). A device whose largest block cannot
carry one full frame is not bound.

In: one bulk IN of the negotiated NTB input size. Its header must be its
format's own length, and the block, no longer than the input size, must
all have arrived; nothing past its block length is read. Every NDP in the
chain lies inside the block and is read once, and every datagram in it is
checked against the block, queued, and handed up one per call. A device
that offered 32-bit NTBs may send one (an NTH32 "ncmh", NDP32s "ncm0" with
32-bit indices, as recalled, unverified); it is read the same way, by the
same walk. At most 40 datagrams are taken from one block. A malformed
block, NDP or entry is dropped, as is an NDP with CRCs and a datagram
longer than a frame.

Out stays NTB16: every device takes it, the format is set to 16-bit where
32-bit is offered too, and a block is at most one 4096-byte transfer, far
inside 16-bit offsets.

## Link

The link reads up once bound, and from then on follows NETWORK_CONNECTION
on the communications interface's interrupt IN endpoint, which the host
configures beside the bulk pipes; it is looked at once every 64 receives.
A device without one, or whose host refuses it, reads up while bound.

## Log

`log usbnet` shows `[usbnet cdc-ncm]` lines when Debug is granted:
`started, looking for a device`; each port tried and why it was not taken,
or `port N: bound` for the one taken;
the bind step that failed with its errno, the only line that names the
device's vendor:product; when bound, `port N up, drawn station address
programmed`, or `drawn station address, device filter opened`, or `drawn
station address not confirmed: device filter would not open`; and `device
stopped answering, given back`. Without entropy, `no entropy for a station
address, no device bound until there is` is said once. No line carries a
station address.
