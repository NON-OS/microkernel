## Interface contract

Two request families arrive on the one service inbox: net_core's link
protocol (`nonos_wifi_core::netif`: link status, transmit, receive) and the
Wi-Fi control family (`nonos_wifi_client`, tag `0x57494649`):

| Operation | Input | Output |
|---|---|---|
| status (4) | none | bring-up stage, data-path counters, efuse and BAR diagnostics, receive refusals, rekeys |
| scan (3) | none | passes, raw frames, beacons, then the network list (signal, WPA2/WPA3 flags, SSID) |
| connect (1) | SSID, passphrase, optional flags octet (WPA3-only, hidden) | status code, handshake counters, state, AKM, AP status or reason code |
| disconnect (2) | none | teardown result |
| link (5) | none | associated, six zero bytes (no BSSID), SSID, AKM |

Unknown operations reply with an error status word, and a frame neither family
takes is answered rather than left without a reply. Every length in a connect
body is checked before use; the flags octet is optional, so an older client
joins as before.
