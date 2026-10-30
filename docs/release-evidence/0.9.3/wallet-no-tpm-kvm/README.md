# Creating a wallet on a machine with no TPM, under KVM

The sealed 0.9.3 standard image under KVM with no TPM, at the desktop after setup. The Wallet was opened
from the dock and "Create a wallet" was pressed.

What happened, seen on two separate runs:

- The wallet showed `the keyring did not answer: try again in a moment` and no wallet was made
  ([the screen](wallet-after-create.jpg)).
- The serial log ([serial.log](serial.log)) has, in order, a trap in the keyring's process and an unanswered
  call, and then the keyring starting again:

```
[TRAP UD] cpl=3 rip=0x000000001b7a3500 ... pid=0x0000000000000003
[DIAG] ipc.call unanswered keyring err=-3
[SPAWN] name=keyring pid=0x000000000000004E caps=0x0000000010000038 ...
[KEYRING] restarted after it ended
```

The log also says why there is nothing to seal under: `[WARN] enforce: TPM not available` and
`[WARN] proofs: TPM OFF: no TPM 2.0 measured boot`.

What it should do: the wallet has a status line for a machine with no machine key
(`no machine key to seal under, so this wallet is gone at reboot: write down the phrase`). On a TPM-less
machine the keyring should reach that answer and not trap. This is a bug in 0.9.3 and is listed in the
[known issues](../../../release/0.9.3-known-issues.md). A machine with a TPM was not tried in this run.
