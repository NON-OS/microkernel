## Runtime lifecycle: joins

A connect (`server/radio/join.rs`, `firmware/gen3/join`) runs inside the
request, as on the RTL8821CE, in this order:

1. Leave any current network. Map the transmit region on first use.
2. Hunt the network: stop a background sweep in flight (SCAN_ABORT_UMAC),
   run a passive sweep until a beacon or probe response names the network,
   then stop it. Not heard in a whole sweep: code -2.
3. Draw the handshake nonce and SAE secrets. The MLME reads the beacon and
   refuses here, before any firmware context or frame, an open, TKIP,
   Enterprise or HT-only network, a passphrase that cannot be one, and a
   network saved as WPA3 that now offers only WPA2.
4. Put the firmware contexts up in Linux v6.12's MLD order: PHY context on
   the channel and its RLC chains, link pointed at the PHY then activated with
   the ACK rates, the access point as station 0 (`mfp` 1 until authorized),
   its management (TID 15) and data (TID 0) queues, session protection
   (900 ms), waited for until the firmware is on the channel.
5. Exchange frames through the MLME: Open System or SAE (hash-to-element or
   hunting and pecking) authentication, association, then the four-way
   handshake, an unanswered frame resent at each 300 ms of silence up to six
   times within twenty-five waits (7.5 s, so the hunt and the exchange fit the
   client's 20 s connect timeout). On association the MAC context, station entry
   and link take the AID and ERP state; a session that ends early is asked
   for again.
6. Install the pairwise key (index 0, with MFP when negotiated) and the group
   key (its own index), authorize the station entry, cancel the session, and
   open the port.

Power save follows Linux v6.12's balanced scheme (`mvm/power.c`). At boot
the device power command lets the card sleep (`POWER_SAVE_ENA`); the shadow
registers keep the driver's register writes safe while it does. On
association the station's power command (MAC_PM_POWER_TABLE) goes out with
its keep-alive (three DTIM periods, at least 25 s); once the keys are in, it
goes again with power management on and 100 ms receive and transmit
timeouts, and a teardown sends it with power management off. Beacon
filtering stays off, since the link watch hears the access point by its
beacons. Each RLC chain set has one receive chain, as Linux has outside CAM.

Any end takes down what was put up, keys first, flushing the queues before
removing them. Management and handshake frames go at the lowest basic rate
and data at the highest basic rate, each with its rate in the command
(`IWL_TX_FLAGS_CMD_RATE`); no rate scaling table is configured. The station
protects every frame it sends (`Ccmp::SoftwareTxHwRx`) and the firmware sends
it as it is (`IWL_TX_FLAGS_ENCRYPT_DIS`); received frames the firmware
decrypted from its key table, and any it did not, both pass the station's
checks (from the BSS, protected, not a replay) before net_core sees them.

While the port is open the background scan rests and each pass of the loop
services the link: the access point's group rekeys are answered and the new
key installed (the old index removed), and a deauthentication or
disassociation from the access point (unprotected only without management
frame protection) closes the port and takes the contexts down. A disconnect
sends a deauthentication (protected under MFP) before the same teardown.
An access point unheard for 2 s is probed, and after three unanswered probes
500 ms apart the link is lost (`nonos_wifi_core::station::alive`).

Interrupts: the gen3 path polls. It unmasks the ALIVE cause for the boot (as
Linux does) and, once ALIVE is in, masks every device interrupt (CSR_INT_MASK
0, both MSI-X mask registers all ones); the cause registers still latch and the
error checks read them. The interrupt the setup bound is not serviced. To
service interrupts the driver would need its own MSI-X configuration
(`iwl_pcie_conf_msix_hw`: the IVAR tables routing each cause to a vector);
that is not done. The 7265 and 8265 families are MSI-only in Linux (no MSI-X),
so on a machine where their INTx line is not routed they end up polled.
