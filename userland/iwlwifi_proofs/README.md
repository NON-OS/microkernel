# iwlwifi_proofs

Host proofs for the Intel Wi-Fi driver (`capsule_driver_iwlwifi`). The crate
includes the shipping driver source with `#[path]` and runs it against a
modelled device, links `nonos_wifi_core` as the capsule does, and puts calls
into `nonos_libc` through a shim (`libc_shim/`) that records them.

## What it proves

185 `#[test]` functions, among them:

- discovery finds every supported family whatever the INTx line says, and
 only a lost claim ends the interrupt ladder (`discover_tests`);
- a chip whose MAC clock never comes up gives back every grant and the claim
 (`grants_tests`);
- the firmware image's split into LMAC, UMAC and paged sections and their
 device addresses (`dram_map_tests`);
- the legacy FH firmware load writes the documented register sequence
 (`iwlwifi_tests`);
- the gen3 path: firmware selection, the TLV parse, every structure size
 against the Linux v6.12 headers, the DMA plan, the context info and
 peripheral scratch, and the whole bring-up to ALIVE and a passive scan
 against `gen3_model` with the bundled image (`gen3_parse_tests`,
 `gen3_layout_tests`, `gen3_image_tests`, `prph_scratch_tests`,
 `gen3_boot_tests`, `gen3_radio_tests`, `gen3_tests`);
- every join command's bytes and the transmit command and response
 (`station_cmd_tests`, `tx_path_tests`);
- WPA2 and WPA3-SAE joins with data both ways against a scripted access point,
 their refusals, rekeys and teardown (`join_tests`, `serve_tests`,
 `client_join_wire`);
- the host command and receive queues, every device index masked and every
 buffer id checked (`hcmd_tests`, `rx_tests`, `harden_tests`);
- the older WPA, EAPOL, CCMP, MLME and supplicant pieces behind the NIWF ops
 (`wpa_tests`, `eapol_tests`, `ccmp_tests`, `mlme_tests`,
 `supplicant_tests`, `dot11_tests`, `data_tests`);
- the answer to a request whose header the driver refuses
 (`request_refusal_tests`).

The layouts the join commands encode were also compiled against Linux v6.12's
own headers with gcc:, 133
sizes and offsets, all matching.

## What it does not prove

That the firmware on a given laptop accepts these commands in this order, or
that a join completes on the air. No hardware log for this driver is
committed.

## Run

```sh
cd userland/iwlwifi_proofs && cargo test --release
```

See [drivers](../../docs/handbook/drivers.md) and
[proofs](../../docs/handbook/verification/proofs.md).
