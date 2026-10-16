## Real hardware bring-up checklist: joins

- WPA2-PSK, from the Settings panel or net_core's autojoin: within a few
  seconds `[iwlwifi] join: port open; sent 4 recv N eapol 2 state 4 refused
  0` (N at least 4), the panel reads Joined, the link op reports associated
  with AKM 2, and net_core binds `driver.iwlwifi0` and takes a DHCP lease;
  ping and name lookups work.
- WPA3-SAE, once with an H2E access point and once with hunting and pecking:
  `[iwlwifi] join: port open; sent 5 ...` (commit, confirm, association,
  messages 2 and 4), AKM 8, then a lease. A transition-mode access point is
  joined with SAE.
- A monitor-mode capture of the joins: every frame from a locally
  administered address that changes each boot; no probe request; Open System
  (algorithm 0) or SAE (algorithm 3) authentication, an association request
  with the RSNE (AKM 2 or 8, MFPC for SAE), EAPOL messages 2 and 4 in the
  clear, then only CCMP-protected data. The access point's log shows the
  station authorized.
- Refusals: a wrong WPA2 passphrase ends with code -6 (state 3, the access
  point's deauthentication with reason 15 or a timeout); a wrong WPA3 password
  with -10; a network saved as WPA3 that now offers only WPA2 with -7 and
  nothing on the air; a hidden network with -2.
- Group rekey: with the access point rekeying the group key every minute, the
  link stays up across several rekeys and the disconnect line counts them.
- Leaving: a disconnect prints `[iwlwifi] left the network: tx ... rekeys N
  dropped 0 stray 0 failed F`, the access point logs a deauthentication with
  reason 3 (protected under WPA3), and the scan counters climb again. Kicking
  the station from the access point prints `link: the access point ended the
  association, reason R`.
- A join that fails in the firmware prints `join: a firmware context did not
  go up: ...`, `join: command 0x... failed` or `join: the firmware raised its
  error cause`; any of these means a command layout the firmware read
  differently, and the line names the command.
