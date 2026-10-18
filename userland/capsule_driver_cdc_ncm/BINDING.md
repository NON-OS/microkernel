# capsule_driver_cdc_ncm: binding, framing and link

How the capsule takes an NCM function, frames its traffic and follows its
link, and what it says on the log. The contract is in
[README.md](README.md), NNET in [PROTOCOL.md](PROTOCOL.md).

## Binding

1. The first configuration, of all the device offers, with a communications
   interface of subclass 0x0D and protocol 0, a Union naming its data
   interface, an Ethernet Networking and an NCM functional descriptor. A
   device with only ECM (subclass 0x06) is left to driver_cdc_ecm.
2. The bulk pipes go to driver.xhci0, then SET_CONFIGURATION, then
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
device that does not filter answers with a STALL, which Linux does not look
at either; the bind goes on and, without the address set, its line says the
address is not confirmed.

## Framing

Out: one frame per NTB16, an NTH16, an NDP16 with one entry and its zero
entry, and the datagram placed on the device's wNdpOutAlignment,
wNdpOutDivisor and wNdpOutPayloadRemainder. A block that ends on a packet
boundary gets one more byte; one past min_tx_pkt is padded to the full NTB
size (not for DisplayLink docks), as `cdc_ncm_fill_tx_frame` does.

In: one bulk IN of the negotiated NTB input size; every NDP16 in the chain
and every datagram in it is checked against the transfer, queued, and
handed up one per call. A malformed block, NDP or entry is dropped, as is
an NDP with CRCs and a datagram longer than a frame.

## Link

The link reads up once bound: NETWORK_CONNECTION arrives on an interrupt
endpoint, and driver.xhci0 does not configure one beside bulk pipes.

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
