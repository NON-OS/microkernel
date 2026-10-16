## Firmware

The capsule downloads the Realtek RTL8821CE firmware image into the device over
the H2C/DDMA path. Firmware bytes are linked into the capsule with
`include_bytes!`, so no filesystem authority is required at boot. The firmware
path validates the header, stages the sections into a brokered DMA window, drives
the download registers, and waits for the firmware-ready state before MAC init.

## Authority

The capsule may enumerate PCI devices, claim one supported Realtek Wi-Fi
function, map its BARs, and allocate broker-owned DMA rings. It binds no
interrupt: discovery does not look at the INTx line and the rings are polled.
It has no filesystem authority, no credential authority, and no network-stack
authority. Passphrases arrive only for the duration of a connect request and are
handed to the shared supplicant; the driver keeps no profile store.

## Privacy and persistence

The capsule stores no SSIDs, passphrases, scan history, peer MAC history, DHCP
leases, or IP state. Runtime state is limited to grant ids, PCI identity, the
station MAC drawn at random each boot (`src/station.rs`, locally administered
and unicast; with no randomness the radio stays down rather than fall back to
the efuse address), radio/link state, DMA ring metadata, the current channel,
and the session keys the supplicant installs for the active link.

Scanning is passive: the background sweep and the pre-join hunt listen for
beacons. The only frame sent before a join is a probe request for a network
the person marked hidden, naming that network alone
(`serve/connect/probe.rs`, proven in `connect_tests`). The scan list is aged
(a network unheard for three sweeps is dropped) and capped at sixteen.
