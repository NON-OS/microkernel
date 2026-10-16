## Interface contract

Three request families arrive on the one service inbox: the Wi-Fi control
family, net_core's link protocol, and this driver's own.

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

The driver's own `NIWF` protocol (version 1):

| Operation | Input | Output |
|---|---|---|
| `OP_HEALTHCHECK` | none | status |
| `OP_DEVICE_INFO` | none | PCI id, hw revision, family |
| `OP_FIRMWARE_INFO` | none | selected firmware name and size |
| `OP_RF_STATE` | none | RF-kill / init flags |
| `OP_DMA_STATE` | none | brokered DMA staging grant metadata |
| `OP_FIRMWARE_STAGE` | none | staged firmware section counts and bytes |
| `OP_ALIVE_WAIT` | none | alive notification status and interrupt word |

It also routes `OP_FIRMWARE_LOAD`, `OP_RX_POLL`, and the older helpers from
before the shared core ran the join (`OP_MGMT_BUILD`, `OP_BEACON_PARSE`,
`OP_HCMD_ISSUE`; `src/server/dispatch.rs`). Nothing in the tree speaks NIWF to
them. The key helpers that took or returned key material (`0x000D` to
`0x0018`: the PTK, the handshake, CCMP, key unwrap and the raw data path),
with the private copies of the WPA2 code behind them, are gone; their numbers
are never reused and now reply `E_INVAL` (`E_BAD_OP` with no body).

Unknown operations reply `E_BAD_OP`. Non-empty bodies on fixed-width requests
reply `E_INVAL`. Once the gen3 radio owns the card, the ops that write it
(`OP_FIRMWARE_LOAD`, `OP_ALIVE_WAIT`, `OP_HCMD_ISSUE`) reply `E_BUSY` (-16).
