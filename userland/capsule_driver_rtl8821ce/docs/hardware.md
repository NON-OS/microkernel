## Real hardware bring-up checklist

What only a boot on real silicon can confirm. The host proofs cover the parsers, the bring-up sequence against a model, and the bounded retry; these do not.

- On a machine without the device: the broker log shows no claim for it and the capsule exits at once with `EXIT_ABSENT` (2), after one `no controller present, not started` line.
- With the device present but failing (disabled in firmware, or a forced setup error): at most seven claim and release rounds in the broker log over about six seconds, then one `device present, bring-up failed` line naming the last cause (with the Debug capability), and exit `EXIT_GAVE_UP` (6), which the kernel names as `[EXIT] <service> status 6: device present, bring-up failed and was given up`. The capsule holds no core while it waits.
- The power-on, efuse, firmware download and MAC stages each report on the console, and the panel shows the stage reached.
- If the claim or register map is refused, it is retried on the bounded schedule and the panel then shows `NotClaimed` rather than the service vanishing.
- Address: `[rtl8821ce] phy configured` follows the efuse read; `[rtl8821ce] no entropy
 for station address` instead means the kernel gave no randomness and the radio stays
 down. A capture shows the station's source address with the locally administered bit
 set, different on every boot.
- Scan: the Settings Wi-Fi panel status reads Ready and its scan counters climb (passes,
 raw frames, beacons; raw without beacons means frames arrive but do not parse). A
 WPA2-only AP shows WPA2, a WPA3-only AP shows WPA3, a transition-mode AP shows both.
 Only 2.4 GHz networks are listed.
- Nothing named on the air: a monitor-mode capture sees no probe request from the laptop
 while scanning or joining a visible network, and for a network saved as hidden one
 directed probe per channel naming only it.
- Join each AP type and read the connect reply (the panel shows its text; the reply
 carries the AKM: 2 PSK, 6 PSK-SHA256, 8 SAE):
 - WPA2-Personal (CCMP): joins with AKM 2; a wrong passphrase reads "The handshake did
 not finish; check the passphrase".
 - WPA3-Personal, hash-to-element only (SAE PWE 1): joins with AKM 8; a wrong password
 reads "WPA3: the access point did not accept the password".
 - WPA3-Personal, hunting-and-pecking (PWE 0): joins with AKM 8.
 - WPA3 transition mode: joins with AKM 8 (SAE preferred) and the network is saved as
 WPA3; if the AP later drops SAE the join is refused with "Saved as WPA3, but the
 network now offers only WPA2; not joined". A WPA3 join there that does not finish is
 not tried again with WPA2.
 - Open, TKIP-only or Enterprise: refused with "The network's security is not supported
 (open, TKIP or Enterprise)".
- Data: after a join, net_core binds the link and obtains a DHCP lease; the status
 counters show TX and RX-eth rising and receive refusals near zero. Leave the link up
 past the AP's group rekey interval: the rekey count rises and traffic continues.
- Leave: disconnect, or deauthenticate from the AP; the link reads down and the
 background scan resumes.
- DMA: on a machine with more than 4 GiB of memory, bring-up still reaches Ready
 and data flows; no `DMA map above 4 GiB refused` line.
- A join does not hold a core busy: the system stays responsive through a connect.
- Card gone: removing or disabling the card while serving gives one `reads back all
 ones` line and the panel shows `The card's registers read back dead`.
- Power save, with `ps::switch::LPS` turned on in a test build only: the link held
 through sleep, no received frame lost across it, a group rekey and an SA Query
 answered while asleep. Until that run, the switch stays off.

## Explicit non-goals today

No 5 GHz channels, no HT/VHT rates, no open or TKIP networks, no WPA2-Enterprise or 802.1X,
no access point or mesh mode, no monitor mode, no Bluetooth function of the combo chip, no
802.11ac rate tuning beyond the init tables, power save built but off until the run above, no
roaming policy, no stored Wi-Fi
profiles, and no IP, DHCP, DNS or socket logic in this capsule.
