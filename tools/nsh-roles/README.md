# nsh-roles

The fleet operator's tool for nsh/1 roles: the role table
(`nsh-policy/1`), the revocation list (`nsh-revoke/1`), and role grants
and their approvals (`nsh-grant/1`), as
[docs/nsh/nsh-v1-roles.md](../../docs/nsh/nsh-v1-roles.md) lays them out.
Run it on the operator's own machine. No image and no capsule builds it.

```
nsh-roles keygen --out DIR --name NAME
nsh-roles table --fleet HEX --serial N --max-age-hours N --authority F ... \
    [--approver F ...] [--roles F] --key F --out F
nsh-roles revoke --table F --anchor HEX [--id HEX ...] --key F --out F
nsh-roles grant --table F --anchor HEX --subject F --user NAME --role NAME \
    --cap NAME ... --hours N [--parent HEX] --key F --out F
nsh-roles approve --table F --anchor HEX --grant F --key F --out F
nsh-roles show --table F --anchor HEX [--list F | --grant F]
```

- **`keygen`** writes a hybrid key, Ed25519 and ML-DSA-65, in nsh-enrol's
  format: `NAME.secret` (mode 0600) and `NAME.public`. The same command
  makes a role authority's, an approver's and a user's key.
- **`table`** signs the table with `--key`, which must be one of the
  `--authority` keys. It prints the table's anchor, the key id every host
  pins. With no `--roles` the table carries the shipped roles. A roles file
  holds one role a line, `name digest|text approvals max-session-hours
  ceiling,... escalation,...|-`. Keys and roles are written in the order
  the format demands, each once.
- **`revoke`** issues a list now, naming the withdrawn grant ids, signed by
  a role authority of the table.
- **`grant`** issues a base grant from now for `--hours` to the user whose
  public key is `--subject`. With `--parent`, the base grant's id, it
  issues an escalation, which spans at most one hour. The grant id is 16
  fresh random bytes.
- **`approve`** adds one approver's approval to an escalation. The
  approver must be one the table lists, neither the issuer nor the
  subject, and not one who has approved already.
- **`show`** opens a table under its anchor, and a list or a grant under
  the table, and prints them.

Every file is opened or checked by `nsh_roles` exactly as a host does
before it is written, so the tool never hands out a table, list or grant a
host would refuse. No command writes over a file, and a secret key file is
wiped from memory once it is read.

The tests run the command line whole. A fleet made with it logs in through
`nsh_roles::login`, escalates through `nsh_roles::escalate`, and its list
withdraws the grant. Every refusal above writes nothing.
