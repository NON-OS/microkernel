# capsule_driver_rtl8153: binding, framing and link

How the capsule takes an RTL8153 or RTL8153B, frames its traffic and
follows its link, and what it says on the log. The contract is in
[README.md](README.md), NNET in [PROTOCOL.md](PROTOCOL.md).

## Binding

1. The adapter by vendor:product, from r8152's `rtl8152_table` (`ids.rs`):
   0bda:8153 and 18 adapters and docks built on the chip.
2. The vendor configuration: the one whose first interface is class 0xFF,
   with bulk IN on endpoint 1 and bulk OUT on endpoint 2.
3. The chip version from PLA_TCR0; RTL_VER_03 to 06 (RTL8153) and 08, 09
   (RTL8153B) are taken, every other version is refused by name.
4. The bulk pipes go to driver.xhci0, then SET_CONFIGURATION.
5. The bring-up in Linux's order (`up/bring_up.rs`), with the run's drawn
   station address written to PLA_IDR where Linux `set_ethernet_addr`
   writes one, the config registers unlocked around it.
6. When the link comes up, TX and RX are enabled (`enable/`).

The chip's unicast filter (RCR_APM) matches the address in PLA_IDR. The
factory address in PLA_BACKUP, which Linux `determine_ethernet_addr` reads
first, is never read, so a chip with none there binds all the same. The
receive mode is Linux's with every multicast wanted: broadcast, frames to
its own address, and all multicast (PLA_MAR all ones), not every frame.

## Framing

TX: one frame per bulk transfer, behind an 8-byte tx_desc (TX_FS, TX_LS,
length). No padding, as r8152. RX: RX aggregation is turned off so each
frame comes in its own transfer under BULK_MAX; every transfer is still
walked as r8152's rx_bottom walks it (24-byte rx_desc, 8-byte steps, CRC
stripped), and its frames are handed up one per receive.

## Link

The link is read from PLA_PHYSTATUS at most once a second, after a request
is answered (`Nic::tick`), and kept: the chip's link interrupt comes on
endpoint 3, which driver.xhci0 does not configure beside the bulk pair. A
link that comes up starts traffic, as Linux `set_carrier` does; one that
goes down is only noted. A failed read counts toward the device being
taken as gone.

## Log

`log usbnet` shows `[usbnet rtl8153]` lines when Debug is granted:
`started, looking for a device`; each port tried and why it was not taken,
or `port N: bound` for the one taken;
a refused chip or failed step by name with its errno, the only line that
names the device's vendor:product; `chip RTL8153 (RTL_VER_0x)` and `port N
up, drawn station address programmed` when bound; `link up, 1000 Mb/s full
duplex` (or the speed and duplex read), `link up, traffic not started:`
and the step, and `link down`; and `device stopped answering, given back`.
Without entropy, `no entropy for a station address, no device bound until
there is` is said once. No line carries a station address.
