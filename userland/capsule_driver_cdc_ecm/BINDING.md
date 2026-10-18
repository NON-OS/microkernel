# capsule_driver_cdc_ecm: binding, framing and link

How the capsule takes an ECM function, frames its traffic and follows its
link, and what it says on the log. The contract is in
[README.md](README.md), NNET in [PROTOCOL.md](PROTOCOL.md).

## Binding

1. The first configuration with a communications interface of subclass
   0x06, a Union naming its data interface, an Ethernet Networking
   functional descriptor, and a data interface setting with a bulk pair.
   QEMU's usb-net offers RNDIS first and ECM second; ECM is taken.
2. The bulk pipes go to driver.xhci0, then SET_CONFIGURATION, then
   SET_INTERFACE to the data interface's alternate setting with the pipes.
3. SET_ETHERNET_PACKET_FILTER: directed, broadcast, all multicast and
   promiscuous.

The station address is the run's drawn one. ECM has no request that gives
the device an address, and the iMACAddress string is never read, so a
device that filters on its own address would drop every frame sent to the
drawn one unless its filter is open; net.core drops unicast meant for
another station. A device that does not filter answers the filter request
with a STALL, which Linux does not look at either; the bind goes on and
its line says the address is not confirmed.

RTL8153 adapters (0bda:8153, 17ef:721e) are left to driver_rtl8153, as Linux
leaves them to r8152.

## Framing

One frame per bulk transfer. A frame whose length is a multiple of 64 or of
the packet size gets one padding byte (Linux `usbnet_start_xmit`): the
controller driver may move the pipe in 512 or 1024-byte packets, both
multiples of 64. A bulk IN is 2048 bytes; a runt under 14 bytes is dropped,
a 1515-byte transfer is a full frame and its padding byte, and anything
longer (two frames run together, or junk) is dropped rather than cut.

## Link

The link reads up once bound: the network notifications arrive on an
interrupt endpoint, and driver.xhci0 does not configure one beside bulk pipes.

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
