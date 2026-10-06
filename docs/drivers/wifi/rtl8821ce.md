# Realtek RTL8821CE

The RTL8821CE driver [capsule](../../overview/glossary.md#capsule) runs the Wi-Fi function of the Realtek RTL8821CE PCIe card on 2.4 GHz: it scans, joins WPA2 and WPA3 networks and carries traffic for `net.core`.

## What works

Wi-Fi on Realtek RTL8821CE (scan, join, DHCP, DNS, browser traffic). Works on an x86_64 laptop (Intel Gemini Lake, 8 GB), maintainer hardware report, 6 October 2026; the image commit was not recorded.

The rest of this page is read from the code and from the host tests in `rtl8821ce_proofs` and `nonos_wifi_core_proofs`. Those cover WPA3-SAE, PSK-SHA256, group rekeys, deauthentication and the hidden-network probe. The hardware report does not say which security its join used.
