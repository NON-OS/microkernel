## Current implemented surface

- Realtek RTL8821CE PCI discovery and brokered device claim.
- Firmware download over the H2C/DDMA path with bounded DMA staging.
- A random per-boot station address programmed into the MAC id registers.
- MAC, baseband, and RF init tables with per-channel RF retune (2.4 GHz).
- TX and RX descriptor rings, polled; the FCS stripped and every descriptor
 length checked against its slot and the mapping.
- Hardware security engine with pairwise/group key install for CCMP, and the
 software CCMP path for frames the chip left encrypted.
- Passive scan with WPA2/WPA3 flags and signal; join with WPA3-SAE (H2E and
 hunting-and-pecking, PMF) or WPA2-PSK (PSK and PSK-SHA256) through the shared
 `nonos_wifi_core`; group rekey; deauthentication handling; protected leave.
- IPC status, scan, connect, link, and disconnect; net_core's link protocol.

## Wire format

Requests use the capsule header, version `1`, and the shared driver envelope.
Replies begin with a signed status word. All multi-byte integers are
little-endian.

## State ownership

`driver.rtl8821ce0` owns only hardware-facing Wi-Fi state: PCI identity, broker
grant ids, BAR mappings, DMA ring metadata, station MAC, radio and link state,
current channel, and the installed session keys. The Wi-Fi core owns scan
policy, association state machine, authentication, and key derivation;
`net.core` and above own addressing and transport.

## Operating rules

- Do not place DHCP, DNS, IP, or socket policy in this capsule.
- Do not import kernel driver or memory internals.
- Do not use inline PIO or architecture assembly.
- Do not persist Wi-Fi profiles, passphrases, or scan history.
- Firmware download must go through bounded DMA staging and explicit completion.
