# The pack unit, booted: vim installed from packs

Two boots of a qemu-dev image, on 2026-10-26. The image was linux/pack (installer, reader,
vfs hold, nonos_linux_pack) on main 1bd589a16, through the host relay on 8080.

**Not as shipped:** one local commit, never pushed, made `dev_market.chosen()` false.
Since f6fc7351f a development seal lists only the four devtest entries, so `linux.vim` was
not in the market. The maintainers ruled the fix as b/dev-market-union. The commit changes which
catalogue the seal signs, nothing in the install or the run.

## What the boots show (booted)

- **One pack a package.** `market install linux.vim` installed six packages, each as one
  pack. The serial line each prints when its pack lands (install/place/land_said.rs):

  | package | files in its pack | pack bytes | store names |
  |---|---:|---:|---|
  | vim | 0 | 4096 | 231 -> 233 |
  | libncursesw | 2 | 343112 | 241 -> 243 |
  | musl | 2 | 654024 | 244 -> 246 |
  | ncurses-terminfo-base | 39 | 103488 | 247 -> 249 |
  | xxd | 2 | 22600 | 250 -> 252 |
  | vim-common | 1803 | 26395163 | 253 -> 255 |

  Each pack is two names, the pack and the trailer over its index. The whole install took
  the store from 231 to 255 names. As files, the same closure is 1940 names (the design
  note, section 2), and the boot of 2026-10-25 stopped at the 512-name owner budget.
  The names between packs are vim's own program (a market program held until its policy is
  bound stays a file, with PENDING beside it), the package records and the link table.
  vim's pack is empty because the package's one file is that program.
- **Memory.** The install heap peaked at 51158 KiB of 327680, unpacking vim-common.
- **A program that runs out of packs.** `linux vim --version` printed its feature list and
  exited 0 (vim-version.png, boot 1). Its loader, `ld-musl`, and `libncursesw` are both
  served from packs. `linux /usr/bin/xxd -v`, an ELF that is itself in a pack, printed
  "xxd 2024-05-10 by Juergen Weigert et al." and exited 0 (xxd-and-defaults.png, boot 2).
- **A data file read out of a pack.** `linux vim -es -c 1p -c q
  /usr/share/vim/vim91/defaults.vim` printed the file's first line, `" The default vimrc
  file.`, and exited 0. The file is one of vim-common's 1803.
- **No refusals.** Neither serial log has a pack that was not served or a file that was not
  its digest.

Boot 1's third step typed `+1p +qa!`, and the drive types neither `+` nor `!`. vim waited
on its input, and xxd never ran. Boot 2 runs xxd first and reads the file with `-c` alone.

## Not shown here

- An update or uninstall refused while a run holds the pack (EBUSY, IN_USE 38). The order
  is host-tested (capsule_linux_proofs, file/pack order), as is the vfs refusal (fs_proofs).
- A crash mid-swap: host-tested against a store that stops after every step.

## Files

- boot1.steps, boot1-serial.log, vim-version.png
- boot2.steps, boot2-serial.log, xxd-and-defaults.png
