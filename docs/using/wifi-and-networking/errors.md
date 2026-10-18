## Join errors

The `Last join` row shows the driver's answer:

| Line | Code | What it means |
|---|---:|---|
| `Joined` | 0 | Associated, keys installed. DHCP starts next. |
| `The radio is down or the request was malformed` | -1 | The driver could not run the join at all. |
| `The network was not heard on any channel` | -2 | The driver hunted for the network and heard no beacon from it. Scan again. |
| `The keys could not be installed in the card` | -3, -4 | The handshake finished and the card did not take the session keys. |
| `The access point refused the association` | -5 | The access point refused authentication or association, with its own status code. |
| `The handshake did not finish; check the passphrase` | -6 | A wrong WPA2 passphrase ends here: the access point drops the handshake without saying why. |
| `Saved as WPA3, but the network now offers only WPA2; not joined` | -7 | A possible downgrade. Forget the network with `F` only if you know why it changed. |
| `The network's security is not supported (open, TKIP or Enterprise)` | -8 | See the list of refused networks above. |
| `A passphrase is 8 to 63 characters, or 64 hex digits` | -9 | Fix the passphrase length. |
| `WPA3: the access point did not accept the password` | -10 | A wrong WPA3 password. |
| `The handshake did not match the network's beacon; not joined` | -11 | The access point's security changed during the handshake, a possible downgrade attempt. |
| `No randomness for the handshake; not joined` | -12 | The kernel gave the driver no random bytes. |
| `This driver cannot join networks yet` | -38 | The Intel driver cannot run a join on this card, or has no randomness for one. |
| `The driver did not answer` | | No reply came from the driver. |

The panel also says why it did not act:

- `Wi-Fi is off; W turns it on`
- `No Wi-Fi driver is running`
- `C joins a network from the scan list; Enter scans`
- `Still waiting for the driver's last answer; try again when it comes`

When a network cannot be remembered, the reason is one of: `This boot keeps nothing across reboots`, `No TPM to seal the passphrase with`, `Sealed under a different boot state`, `The TPM did not give the sealing key`, `Sealed elsewhere or altered on disk`, `The saved-networks record is damaged` or `No NONOS store on this boot's disk`.
