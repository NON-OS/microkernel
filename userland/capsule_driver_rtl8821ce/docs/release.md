## Release target

The hardware chain is:

```text
driver.rtl8821ce0 -> nonos_wifi_core -> net.core -> net.ip -> apps
```

## Release evidence

- `rtl8821ce_proofs`: 152 `#[test]` functions, host only (116 passed in the
 earlier main run at,
 ).
 The crate includes `firmware/rtw8821c_fw.bin` with `include_bytes!`; without that file the
 crate does not compile.
- `nonos_wifi_core_proofs`: 70 `#[test]` functions, host only (14 in the earlier main run at
 ), including SAE and CCMP test
 vectors and full handshakes against a simulated access point.
- Lean `Nonos.Wpa2Handshake` builds (`docs/evidence/lean/nonos.log:224`). It is a model, not
 tied to this capsule's code.
- No hardware log and no QEMU log for this driver are committed. row 24
 marks scan, association, WPA2 and DHCP on hardware as not yet backed.

## Release checklist

- `rtl8821ce_proofs` and `nonos_wifi_core_proofs` stay green with the firmware binary present.
- A committed hardware serial log from an RTL8821CE machine shows device claim, firmware ready,
 MAC/BB/RF init, scan results, association and a completed WPA2 four-way.
- The same run shows a DHCP lease obtained through `net.core` and above.
- Capsule exit is shown to revoke the device, MMIO and DMA grants.
- Static gate passes for this README and the capsule manifest.
