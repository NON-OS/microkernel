## Station address

Every boot draws a new locally administered unicast address from kernel randomness. The address in the efuse is never used, because access points log the source of every probe (`userland/capsule_driver_rtl8821ce/src/station.rs:17-31`, `draw`). With no randomness the radio stays dark at `NoStationAddress` instead of falling back.

## Scanning

The serve loop scans in the background while no network is joined, so a scan request is answered at once from the list (`userland/capsule_driver_rtl8821ce/src/serve/mod.rs:124-133`, `scanner`).

- Channels 1 to 13 on 2.4 GHz only (`userland/capsule_driver_rtl8821ce/src/serve/tuning.rs`, `SCAN_CHANNELS`). The world-safe domain both drivers keep to lets the radio probe on 1 to 11, while on 12 and 13 it listens and sends only to a network whose beacon it heard there (`userland/nonos_wifi_core/src/regdom/world.rs`, `world`).
- 300 ms on each channel (`userland/capsule_driver_rtl8821ce/src/serve/scanner.rs:33-38`, `DWELL_MS`).
- The scan is passive: the driver listens for beacons and sends nothing.

Before a join the driver hunts the network's beacon: two sweeps at 250 ms a channel (`userland/capsule_driver_rtl8821ce/src/serve/connect/hunt/dwell.rs`, `BEACON_HUNT_DWELL_MS`, `HUNT_SWEEPS`). It sends a probe request only for a network saved as hidden, only on channels 1 to 11, and that probe names only that network (`userland/capsule_driver_rtl8821ce/src/serve/connect/probe.rs`, `hunt_probe`). A hidden network on 12 or 13 is therefore not probed for.

The join channel is the one the beacon names when the domain allows it and it is within four channels of where the beacon was heard, else the channel it was heard on; a beacon heard where the domain allows nothing is passed over (`userland/nonos_wifi_core/src/regdom/channel.rs`, `join_channel`, called from `userland/capsule_driver_rtl8821ce/src/serve/connect/hunt/find.rs`, `find_beacon`).

Every rate is sent at one power index, 0x24 in 0.5 dB steps (about 18 dBm): the domain's 20 dBm EIRP less an assumed 2 dBi antenna gain, set once at bring-up (`userland/capsule_driver_rtl8821ce/src/phy/txpower.rs`, `band_index`). The efuse power-by-rate calibration is not ported.

## Joining

A connect runs the whole join inside the request (`userland/capsule_driver_rtl8821ce/src/serve/connect.rs:17-29`, `JoinPolicy`). The policy is WPA3-SAE whenever offered and WPA2 otherwise, and only SAE for a network saved as WPA3 (`userland/capsule_driver_rtl8821ce/src/serve/connect.rs:139-143`, `JoinPolicy::WPA3_ONLY`). The negotiation itself is described on the [Wi-Fi overview](../overview/security.md#security-a-join-accepts).

- The join gets 8 s (`userland/capsule_driver_rtl8821ce/src/serve/connect.rs:61-68`, `JOIN_BUDGET_MS`).
- On a network that offers both, a WPA3 join that did not finish is tried again once with WPA2, with its own 8 s (`userland/capsule_driver_rtl8821ce/src/serve/connect.rs:69-72`, `PSK_BUDGET_MS`). That second try is skipped when the network was saved as WPA3 and when the access point's SAE confirm said the password is wrong (`userland/capsule_driver_rtl8821ce/src/serve/connect.rs:250-257`, `retry_with_psk`).
- The pairwise and group keys go into the chip's key table, with 4 entries for group keys by key index (`userland/capsule_driver_rtl8821ce/src/serve/connect.rs:57-74`, `GROUP_KEY_SLOTS`).
- The reply carries a status code and the AKM that ran: 2 for WPA2-PSK, 6 for PSK-SHA256, 8 for WPA3-SAE (`userland/capsule_driver_rtl8821ce/src/serve/connect/result.rs:51-68`, `ConnectResult`).

The status codes are defined beside `failure_code`, which maps each way a join can fail to one of them (`userland/capsule_driver_rtl8821ce/src/serve/connect/result.rs:28-49`, `CODE_NO_ENTROPY`; `userland/capsule_driver_rtl8821ce/src/serve/connect/result.rs:77-97`, `failure_code`):

| Ends with | Code |
|---|---|
| Malformed request, radio down | -1 |
| Network not heard | -2 |
| Access point refused authentication or association | -5 |
| Handshake did not finish, the usual sign of a wrong WPA2 passphrase | -6 |
| Saved as WPA3, now offers only WPA2 | -7 |
| Open, TKIP, Enterprise or 802.11n-only network | -8 |
| Passphrase not 8 to 63 characters or 64 hex digits | -9 |
| WPA3 confirm failed, the sign of a wrong password | -10 |
| Message 3 did not match the beacon | -11 |
| No randomness for the join | -12 |

## Carrying traffic

The one service inbox takes two request families: `net.core`'s link protocol, handled by `netif::serve`, and the Wi-Fi control family (`userland/capsule_driver_rtl8821ce/src/serve/mod.rs:100-122`, `netif::serve`). A frame neither family takes is answered with -22 rather than left without a reply (`userland/capsule_driver_rtl8821ce/src/serve/refuse.rs:24-25`, `STATUS_REFUSED`).

While joined, a new group key from the access point is installed and a deauthentication ends the session, so the link reads down and the background scan resumes (`userland/capsule_driver_rtl8821ce/src/serve/mod.rs:137-150`, `after_receive`). The driver does not rejoin by itself; see [autojoin](../overview/stages.md#autojoin-and-losing-the-link).

## Firmware

The capsule links `rtw8821c_fw.bin` from `nonos-bootloader/firmware/realtek/` with `include_bytes!` (`userland/capsule_driver_rtl8821ce/src/fwload.rs:38`, `include_bytes`). The file is 139,472 bytes in this tree. The note beside the driver, [firmware/README.md](../../../../userland/capsule_driver_rtl8821ce/firmware/README.md), gives its origin: linux-firmware, `rtw88/rtw8821c_fw.bin`, version 24.11.0, copyright Realtek Semiconductor Corp. It is not AGPL code. Its terms are in [the Realtek licence](../../../../nonos-bootloader/firmware/realtek/LICENSE): binary redistribution without modification, the notice kept, no reverse engineering.
