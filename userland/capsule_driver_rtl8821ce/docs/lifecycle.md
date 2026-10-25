## Runtime lifecycle

Startup discovers a supported Realtek Wi-Fi PCI function, claims it, maps its
BARs, allocates the TX/RX DMA rings, downloads firmware, reads the efuse
calibration, draws the per-boot station address and programs it into the MAC,
runs the MAC/BB/RF init tables, and serves IPC, polling the rings between
requests and sweeping channels 1 to 13 in the background.

A connect request hunts the target's beacon (probing only a hidden network),
then joins under the request's policy: SAE when the network offers it with
management frame protection capable, PSK otherwise, and never PSK for a
network the client saved as WPA3. The shared MLME runs authentication (SAE
commit and confirm, or open), association with the RSN element it chose, and
the four-way handshake, checking message 3's RSN element against the beacon;
the pairwise and group keys go into the hardware crypto engine. Received data
frames reach the stack only from the BSSID, protected, above the replay
counter; group rekeys are answered and installed; a deauthentication takes the
link down (unless PMF is on and it is unprotected). Teardown is handled by
process exit and broker revocation of the device, MMIO, and DMA grants.

## Failure model

Every setup phase rolls back prior broker grants on failure. Unsupported PCI
IDs, missing BARs, failed MMIO/DMA grants, firmware download
timeout, MAC/PHY init failure, association timeout, or a failed four-way prevent
the affected operation from reporting success. Packet transport above the link
layer stays in the upper network capsules.

With no RTL8821CE in the device list the capsule logs one line and exits
`EXIT_ABSENT` (2) before claiming anything. The claim and register map are
retried on the shared bounded schedule (`nonos_libc::bring_up`: seven tries,
sleeping between them, each failed try giving the claim back); running out
leaves the capsule serving with stage `NotClaimed`, so the panel can show why
instead of the service vanishing. Later stages (power, firmware, MAC) are not
retried: the capsule serves the stage it reached, as before.
