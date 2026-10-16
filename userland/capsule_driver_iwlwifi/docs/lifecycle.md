## Runtime lifecycle

Startup (`setup`) discovers a supported Intel Wi-Fi PCI function, claims it,
maps BAR0, binds INTx if the line is routed, else one MSI-X vector, else runs
polled, maps the 64-page staging grant, requests MAC access and waits for the
clock, and reads the hardware revision.

Then, before the first request, the gen3 radio comes up once
(`server/radio`): check the register window, take the NIC (prepare, software
reset, APM init), read CSR_HW_REV and CSR_HW_RF_ID, select the firmware, map
the grants, lay out the context info, peripheral scratch, rings, image loader
and firmware sections, kick the self-load, start the CPU (UREG_CPU_INIT_RUN
through the UMAC window), wait up to 2 s for ALIVE (status 0xCAFE), ring the
PNVM doorbell when the part reports a SKU id, then run INIT_EXTENDED_CFG,
NVM_ACCESS_COMPLETE, INIT_COMPLETE, NVM_GET_INFO, antennas, BT, SoC latency,
power, MCC "ZZ", scan config, and the MLD MAC and link contexts, each bounded by
a 2 s reply wait.

The serving loop then waits at most 50 ms for a request and, between
requests, pumps a background passive UMAC scan over the NVM's channels: a
sweep is started, every received beacon or probe response goes through
`nonos_wifi_core` into the shared scan list, the completion carrying this
scan's uid ends it, and the next sweep starts after a 3 s rest. A sweep that
runs past its budget (150 ms per channel plus 2 s) is given up and counted.
Teardown is handled by process exit and broker revocation of the device, MMIO,
IRQ, and DMA grants.
