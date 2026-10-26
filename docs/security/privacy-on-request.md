# Privacy: What leaves only when you ask

Part of [Privacy](privacy.md).

Nothing on this page is dialled until you open the program and ask it to
connect. Which network carries it is your [network
choice](../using/privacy-network.md), except where a row says otherwise.

## Programs on the desktop

| Address | Who | When | How to turn it off |
|---|---|---|---|
| `html.duckduckgo.com` | Browser | You search from the address bar | Type an address instead |
| `fonts.googleapis.com` | Browser | A page you open asks for a Google web font | Do not open such pages |
| `neverssl.com`, `httpforever.com`, `info.cern.ch`, `example.com` | Browser | You click that home page shortcut | Do not click it |
| `ethereum-rpc.publicnode.com`, `mainnet.gateway.tenderly.co`, `rpc.mevblocker.io` | Wallet | You open the wallet on mainnet; never Direct | Do not open the wallet |
| `ethereum-sepolia-rpc.publicnode.com`, `sepolia.gateway.tenderly.co` | Wallet | The same on Sepolia | The same |
| `rpc.sepolia.ethpandaops.io` | Wallet | The same on Sepolia | The same |
| `1.rpc.thirdweb.com`, `eth.rpc.blxrbdn.com`, `eth-mainnet.public.blastapi.io` | Shield | You use the NOX Shield on mainnet; Nym or Anyone only | Do not use the Shield |
| `rpc.flashbots.net`, `sepolia.rpc.thirdweb.com` | Shield | The same | The same |
| `huggingface.co` | Model fetch | You download a Qwen model; Anyone unless you ask for Direct | Do not download |
| `dl-cdn.alpinelinux.org` | Linux installer | You install a Linux package; Anyone, or direct to a private address | Do not install |
| `tool.linux.net` | Terminal | You run, in the Terminal, a Linux program whose signed market policy grants NET_ANON. The personality asks the kernel for that program's network ticket and exits; the Terminal starts the same command once more as this role, whose traffic goes over Anyone only and only where the program's policy lets it reach. No other program, and no run started any other way, can ask for it | Do not run that program, or uninstall it |

The browser and the terminal go wherever you point them; this list covers only
the places their code names. The model catalogue carries the Qwen file URLs as
signed data, written at build time by `tools/nonos_qwen_tier/download.py`.

| Address | Who | When | How to turn it off |
|---|---|---|---|
| `etherscan.io`, `sepolia.etherscan.io` | Wallet | Never dialled: shown as text | Nothing to turn off |

## Hidden services

| Address | Who | When | How to turn it off |
|---|---|---|---|
| `iywfqrj6xyqey574vjtljswxhzeoeqfvlmpqfueva4stooiu3blo7sqd.anyone` | Shield | Pool landers, as Anyone onion services | Do not use the Shield |
| `5swdytwdtcspbntuofrknlqa7ls2nw2zol2o6tts2jv5hs67n4ee5nad.anyone` | Shield | The same | The same |
| `ywhgp672zlndlpjubaq23yhbuy526335jfgl2zr74emhgoepovh56lid.anyone` | Shield | The same | The same |
| `wusdfhdpumeamiyex3v625zled37m2lw24x4bxr3mngu75ryz7vurvad.anyone` | Shield | The same | The same |
| `dns-stage-1.anyone.anyone`, `hnsywhyh3zvvqzkmum7b3fxueii3bueeqjbwkfpngcqktxmubedrf5yd.anyone` | `net_anon` | A short `.anyone` name is looked up | Use full addresses |
| `dns-stage-2.anyone.anyone`, `xxfuq2xfwq7vxgadwywmtmfzeyk5j2oxhjhbn3onaq5h7yp7e3tpmkqd.anyone` | `net_anon` | The same | The same |
| `dns-stage-3.anyone.anyone`, `xvtw2foswsovdutimyjo66zy3k26uehfcwdgrakut43cw4fto2djo2qd.anyone` | `net_anon` | The same | The same |
| `dns-live-1.anyone.anyone`, `gadmrvl67444hgzrhsnhzknxaimfnzp6az3wq4d2j7hrf7th34elrrad.anyone` | `net_anon` | The same | The same |
| `dns-live-2.anyone.anyone`, `kjlkfrfxquevo64qv4gssl3t52tiuay2muj7u4rox4llxboj4c4ypcid.anyone` | `net_anon` | The same | The same |
| `dns-live-3.anyone.anyone`, `jntoblprbfgcpldwuzobmzsdjs6mtwtr3dtn3mtgdjnk6j7x2frcabad.anyone` | `net_anon` | The same | The same |

Hidden services never leave their network: this machine reaches them through a
circuit and never learns or dials their address.

## Only in builds a person chooses

| Address | Who | When | How to turn it off |
|---|---|---|---|
| `162.159.200.1` | `net_ntp` | Clock sync on UDP 123, in the `microkernel-net-ntp` build only | Do not build it |
| `1.1.1.1` | `net_dns` | The older stack's default resolver; the desktop uses the DHCP lease's | Not built with `net_core` |
| `10.0.2.2` | Linux installer | Alpine's packages from the host's relay, in a development image only on port 8080 (`dev-mirror`, [the development mirror](../build/dev-mirror.md)) | No release image has it |

| Build setting | Who | What it names | When it is dialled |
|---|---|---|---|
| `NONOS_PACKAGE_MIRROR` | Linux installer | The NONOS mirror for the in-tree tools | You install one |
| `NONOS_DEB_MIRROR`, `NONOS_DEB_HOST` | Linux installer | A Debian mirror; unset, none | You install a Debian package |
| `NONOS_PACMAN_MIRROR`, `NONOS_PACMAN_HOST` | Linux installer | A pacman mirror; unset, none | You install one |

`capsule_nox_settle` names the mainnet and Sepolia RPC hosts above and calls
them on its own, at start and every six hours; it is in no desktop build.
