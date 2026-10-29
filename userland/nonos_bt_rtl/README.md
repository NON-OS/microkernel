# nonos_bt_rtl

The Realtek RTL8821C Bluetooth patch loader that `capsule_driver_bt_usb`
runs before standard bring-up. `prepare` takes the controller's USB ids,
the patch file and the config file, and:

1. sends nothing unless the id is Realtek's (vendor 0x0bda with the
   Bluetooth class, or 0x13d3:0x3529, as Linux v6.12 `btusb.c` lists them);
2. reads Read Local Version, and goes on only for the RTL8821C's ROM
   (subversion 0x8821, revision 0x000c, HCI 8): a controller already
   running a patch reports the patch's version and is left as it is;
3. reads the ROM version (vendor command 0xfc6d, one octet);
4. picks the patch whose chip id is ROM version + 1 from the "Realtech"
   epatch file, checks its project id (10, the RTL8821C's), replaces its
   last four octets by the file's firmware version, and appends the config
   (signature 0x8723ab55, entries that fill it exactly);
5. sends the whole by vendor command 0xfc20: length / 252 + 1 fragments of
   252 octets, the last holding the rest (none, for a multiple of 252),
   numbered 0 to 0x7f then 1 to 0x7f again, the last with bit 7 set.

Every reply must be Command Complete with status 0 and one octet, or the
load stops. The host's bring-up then starts with HCI Reset.

The format and commands are hardware facts read from Linux v6.12
`drivers/bluetooth/btrtl.c` and `btrtl.h` (GPL-2.0-only), cited in each
file; the code is original. The files themselves are linux-firmware's,
beside the capsule (`capsule_driver_bt_usb/firmware/README.md`).

Proofs: `userland/nonos_bt_rtl_proofs` (the real files, held to their
pins, against a modelled ROM that checks each fragment), and the end to
end in `userland/bt_usb_proofs/src/realtek`.
