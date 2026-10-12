# capsule_driver_rndis: binding, framing and link

How the capsule takes an RNDIS function, frames its traffic and follows its
link, and what it says on the log. The contract is in
[README.md](README.md), NNET in [PROTOCOL.md](PROTOCOL.md).

## Binding

1. The first configuration with a control interface of class 0x02/0x02/0xFF
   (an ACM descriptor with capabilities is a modem and is left), 0xE0/0x01/0x03
   or 0xEF/0x04/0x01, and its data interface: the Union's, else the call
   management descriptor's bDataInterface, else interface 1 for a control
   interface 0. QEMU's usb-net offers RNDIS first (bConfigurationValue 2).
2. The bulk pipes go to driver.xhci0, then SET_CONFIGURATION. No
   SET_INTERFACE unless the pipes sit on a nonzero alternate setting.
3. INITIALIZE (RNDIS 1.0, host MaxTransferSize 4096), then the completion's
   status, medium (802.3) and MaxTransferSize (a full frame, its header and
   a padding byte must fit).
4. SET OID_802_3_CURRENT_ADDRESS to the run's drawn station address. A
   device that refuses it, as QEMU's usb-net does, is bound all the same.
5. SET OID_GEN_CURRENT_PACKET_FILTER: directed, broadcast, all multicast and
   promiscuous, Linux's RNDIS_DEFAULT_FILTER, whether or not the device took
   the address; a device that took it may still filter on its own, and
   net.core drops unicast meant for another station. A refusal fails the
   bind.

The device's OID_802_3_PERMANENT_ADDRESS, which Linux generic_rndis_bind
reads, is never asked for: no QUERY is sent.

Each command is SEND_ENCAPSULATED_COMMAND, then GET_ENCAPSULATED_RESPONSE
polled every 10 ms for up to a second, matching the type and RequestID and
passing over INDICATE_STATUS messages. A failure after INITIALIZE sends
HALT.

## Framing

One PACKET_MSG per frame: a 44-byte header, DataOffset 36. A transfer that
fills its last packet gets one padding byte, counted in MessageLength. A
bulk IN of 4096 bytes may hold several messages; each is checked against
the transfer and its frame against the message, and the frames go up one
per receive. A malformed message is dropped.

## Link

The link reads up once bound: media state comes on the interrupt endpoint,
which driver.xhci0 does not configure beside bulk pipes.

## Log

`log usbnet` shows `[usbnet rndis]` lines when Debug is granted:
`started, looking for a device`; each port tried and why it was not taken,
or `port N: bound` for the one taken;
a failed bind step by name with its errno, the only line that names the
device's vendor:product; when bound, `port N up, drawn station address
programmed`, or `drawn station address, device filter opened` for a device
that refused the address; and `device stopped answering, given back`.
Without entropy, `no entropy for a station address, no device bound until
there is` is said once. No line carries a station address.
