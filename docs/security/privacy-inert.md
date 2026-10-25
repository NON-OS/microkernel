# Privacy: Names in the code that dial nothing

Part of [Privacy](privacy.md).

The audit finds every address-shaped value in a networked capsule's source.
These are such values that never become a connection, each with what it is.

| Literal | Where | What it is |
|---|---|---|
| `0.0.0.0` | Browser | The unspecified address, which the address parser refuses |
| `127.0.0.1` | Linux personality | Loopback, answered inside the machine |
| `12.8.4.0` | Terminal | The bit shifts that print an IPv6 group as four hex digits |
| `www.ibm.com` | Browser | A document type name the page parser matches for quirks mode |
| `www.w3.org` | Browser | The SVG namespace written into a serialised tag |
| `mforujillfk4w5h2zqgxdzendn3qb2zes4r2h57ownojshhmtcdzgdyd.onion` | Shield | The host build's pool landers; not compiled into NONOS |
| `lfpw5uoslfiqmwsc7o2d2qts3grmfnixrhae3hkgkv6x4dlxpvafp4yd.onion` | Shield | The same |
| `7ywwruseitmycfjwh2upbecetm6edej7pkxpp4xtzmtioes4ubs63kad.onion` | Shield | The same |
| `g7uyxmffgim7sneprrkdp4ecshupvd3kuaadho6f7e2gcd3gazimriyd.onion` | Shield | The same |
| `ewjq3ue43pclh6qyx3triup7org64nykbjgy7tmderlzzx27kstfdiqd.onion` | Shield | The same |

The `.onion` rows are behind `cfg(not(target_vendor = "nonos"))` in
`userland/shield_core/src/net/pools.rs`; no network NONOS runs reaches a Tor
onion.
