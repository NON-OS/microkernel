## Interface contract

Two request families arrive on the one service inbox: the Wi-Fi control
family and net_core's link protocol. Anything else gets no reply
(`server/control/route.rs`, `Route::Other`).

The Wi-Fi control family (`nonos_wifi_client`, tag `0x57494649`):

| Operation | Input | Output |
|---|---|---|
| status (4) | none | stage byte, then step, detail, CSR_HW_REV, CSR_HW_RF_ID, sweeps, stalls, frames, beacons |
| scan (3) | none, radio up | sweeps, frames, beacons, then the network list |
| connect (1) | `[ssid_len][ssid][pass_len][pass][flags]` | status code, sent, received, data, EAPOL, probe, deauthentications, to us, state, AKM, AP status (`server/join_wire.rs`) |
| disconnect (2) | none | 0 |
| link (5) | none | `[associated][bssid 6][ssid_len][ssid][akm]` |
| connect, disconnect, link when the radio cannot join | any | -38 |
| scan with the radio down, any other op | any | -38 |

The radio can join when it is up, a station address was drawn, and the
firmware runs every join command at the layout this driver encodes
(`gen3::station::ids`). The connect codes are the RTL8821CE's (-1 to -12,
named in `nonos_wifi_client::driver::join_text`); -1 also covers a join
command the firmware failed.

net_core's link protocol (`nonos_wifi_core::netif`, tag "NNET"): link status,
MAC address, transmit and receive one Ethernet frame. Until a port is open the
link reads down and holds no address.

The driver's own `NIWF` protocol is gone: nothing in the tree spoke it. Its
reports (health, device, firmware, RF and DMA state), the legacy firmware
stage, load and ALIVE wait, the receive poll, the host command and the older
frame helpers were deleted with the code behind them, and a request carrying
its tag gets no reply.
