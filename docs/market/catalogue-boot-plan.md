# 05: the boot plan for the 17 catalogue apps (0.9.3)

For the boot session. What must hold on a booted, merged tree before the
catalogue ships, and how its logs become the apps' measured boxes.

## The image

- A debug build of the merged main, sealed with a throwaway operator key
  made for the run, never the production key, so that the market's
  baseline index carries the 17 listings (mk/21 and the seal pass
  `--catalogue` now). The log says which key it was. Re-signing with the
- QEMU with the IOMMU on, the Anyone network up (installs leave through it),
  and the serial log kept. Then the test-matrix hardware where it can run.

## For each app: one boot, one log

1. In the App Store's Linux tab, install `linux.<app>`. The log must show
   `[LINUX-INSTALL] started, in its release's box linux.<app>` and no
   `[LINUX] refused the market's policy` (I-C5 at work).
2. In the Terminal, run its workload below, as the person would use it.
3. Quit it; let the family seal; say yes to the proof; keep the log, which
   ends with `[PROVE] syscall buckets ok=<130 hex>`.

| App | Workload (home is /home/<app>) |
|---|---|
| helix | `hx --health`; `hx notes.txt`: type, `:w`, `:q` |
| micro | `micro notes.txt`: type, Ctrl-S, Ctrl-Q |
| kakoune | `kak -n -ui dummy -e 'edit a.txt; exec ihello<esc>; write; quit'`; then `kak a.txt` by hand |
| vim | `vim -es -c 'put =\"hello\"' -c wq a.txt`; then `vim a.txt` by hand |
| bat | `bat --paging=never /etc/passwd`; `bat --list-languages` |
| eza | `eza -la /`; `eza --tree /usr/share` |
| nnn | `nnn`: browse, open a file, quit |
| ncdu | `ncdu -o out.json /usr`; `ncdu /usr` by hand |
| fzf | `ls /usr/bin \| fzf --filter=sh`; `fzf` by hand |
| git | `git init r`, add, commit, branch, merge, `git log`, `git diff` |
| tig | in `r`: `tig`, browse the log, quit |
| lazygit | install `linux.git` first (Alpine's lazygit does not pull it); in `r`: stage, commit, quit |
| age | `age -p -o a.age notes.txt`, then `age -d -o back.txt a.age`; compare |
| minisign | `minisign -GW`; sign a file; verify it |
| flac | `head -c 16000 /dev/urandom > in.raw`; encode it raw (16-bit mono 8 kHz); decode |
| lame | `lame -r in.raw out.mp3`; `lame --decode out.mp3 back.wav` |
| geth | `geth --dev --datadir dev console`: `eth.blockNumber`, `exit` |
| profanity | `linux profanity`: `/connect` a throwaway account on a public server over Anyone, then over Nym; `/omemo gen`; send one OMEMO message to a second throwaway account and read it there; `/quit`. Then once with no network running: the connect must fail with ENETUNREACH on the [LINUX] line, never go direct |
| monero | `linux monero-wallet-cli --version`; then `linux monerod --version` must be refused at its installed path (a held program); with the network path, `linux monero-wallet-cli --generate-new-wallet w --daemon-address <a public node>:18089` over Anyone, then over Nym, `refresh`, `exit` |

## Three checks that must fail closed

- Install a listing whose release carries a policy naming another program;
  the install must end with the market's refusal, not a default-box install.
- Install `linux.vim`: the log must show vim installed, never gvim (the
  provider fix in install/index_find.rs).
- With `linux.age` installed, run `/usr/bin/age-keygen`: it must be refused, not run
  in the default box. A program of a market package that its policy does
  not name stays held (install/market/minted.rs). Then do the same with an install
  cut off part way: power off during `linux.flac`'s install, boot again, run
  `/usr/bin/flac`. It must be refused, or not found if the cut came before
  it landed, and uninstall then reinstall must work.

## The verdict, and the measured boxes

    PYTHONPATH=tools python3 -m nonos_market_catalogue.verdict --write \
        helix=helix.log micro=micro.log ... geth=geth.log

Every app must PASS. --write puts each app's measured set into its recipe.
The next seal lists it with that set, and the catalogue documents are
regenerated. Repeat the runs on the hardware matrix, adding each log to its
app (`helix=qemu.log helix=t480.log`): the sets unite. An app that fails
stays out of the release until its failure is fixed. It is never shipped
with a box nobody has seen it run in.


Re-sign market, linux and the kernel image (I-C5, the provider fix).
