# capsule_driver_cdc_ecm: binding, framing and link

How the capsule takes an ECM function, frames its traffic and follows its
link, and what it says on the log. The contract is in
[README.md](README.md), NNET in [PROTOCOL.md](PROTOCOL.md).

## Binding

1. The first configuration with a communications interface of subclass
   0x06 whose Union it masters and names a CDC Data interface of the same
   configuration, an Ethernet Networking functional descriptor of 13 bytes,
   and a data alternate 1 with exactly one bulk IN and one bulk OUT. QEMU's
   usb-net offers RNDIS first and ECM second; ECM is taken.
2. wMaxSegmentSize of 0 or under 1514 is refused; one above the 2048-byte
   receive buffer is clamped to it (no larger frames are turned on).
3. Every interface of the configuration is held with the host's claim
   table; one another driver holds stops the bind, never taken over.
4. The bulk pipes go to the host, then SET_CONFIGURATION, then
   SET_INTERFACE to alternate 1.
5. SET_ETHERNET_PACKET_FILTER: directed, broadcast, all multicast and
   promiscuous.

The station address is the run's drawn one. ECM has no request that gives
the device an address, and the iMACAddress string is never read, so a
device that filters on its own address would drop every frame sent to the
drawn one unless its filter is open; net.core drops unicast meant for
another station. A device that does not filter answers the filter request
with a STALL; the bind goes on and its line says the address is not
confirmed.

RTL8153 adapters (0bda:8153, 17ef:721e) are bound in their ECM
configuration like any other device: the vendor-mode driver for them is
quarantined (`quarantine/rtl8153`), and their ECM configuration is the
class this driver is written from.

## Framing

One frame per bulk transfer. A frame whose length is a multiple of 64 or of
the packet size gets one padding byte, as the bus cannot send a zero-length
packet: the host may move the pipe in 512 or 1024-byte packets, both
multiples of 64. A bulk IN is 2048 bytes; a runt under 14 bytes is
dropped, a 1515-byte transfer is a full frame and its padding byte, anything
longer (two frames run together, or junk) is dropped rather than cut, and a
transfer that fills the buffer without ending is let go up to its short or
zero-length end; each is counted.

## Link

The link reads up once bound and follows NETWORK_CONNECTION from the
notification endpoint, looked at every eighth request. A notification must
name the communications interface and be as long as its wLength says
(NETWORK_CONNECTION none, CONNECTION_SPEED_CHANGE eight); another is
dropped and counted. A device without a notification endpoint keeps the
link up.

## Log

`log usbnet` shows `[usbnet cdc-ecm]` lines when Debug is granted:
`started, looking for a device`; each port tried and why it was not taken,
or `port N: bound` for the one taken;
a failed bind step by name with its errno, the only line that names the
device's vendor:product; when bound, `port N up, drawn station address,
device filter opened`, or after a STALL `drawn station address not
confirmed: device filter would not open`; and `device stopped answering,
given back`. Without entropy, `no entropy for a station address, no device
bound until there is` is said once. No line carries a station address.
