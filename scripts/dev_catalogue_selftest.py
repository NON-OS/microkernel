# NONOS Operating System
# Copyright (C) 2026 NONOS Contributors
#
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU Affero General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
# GNU Affero General Public License for more details.
#
# You should have received a copy of the GNU Affero General Public License
# along with this program. If not, see <https://www.gnu.org/licenses/>.
"""Each way a release could come to take the development catalogue, made on
purpose, and the check refusing it; the tree as it stands, accepted."""

import json

import dev_catalogue_scan as scan
from dev_catalogue_behaviour import behaviour
from dev_catalogue_content import SOURCE, catalogue, load
import dev_catalogue_chain as chain

SEAL = "tools/nonos_seal/dev_market.py"
FLIPS = [  # the seal's choice, inverted or widened
    ("return capsules.DEVELOPMENT and in_dev_tree()", "return not capsules.DEVELOPMENT and in_dev_tree()"),
    ("return capsules.DEVELOPMENT and in_dev_tree()", "return capsules.DEVELOPMENT or in_dev_tree()"),
    ("return capsules.DEVELOPMENT and in_dev_tree()", "return capsules.DEVELOPMENT"),
    (' and os.path.isdir(".git")', ""),
    ('open(checkout, "rb").read() == key', 'open(checkout, "rb").read() != key'),
]
NAMING = [  # a release path naming the catalogue
    ("mk/21-market.mk", "\t--linux-list tools/devnet/market/index.json \\"),
    ("tools/nix/capsules.nix", "    cp tools/devnet/market/index.bin target/market/index.bin"),
    ("userland/capsule_market/src/baseline.rs", 'include_bytes!("../../../tools/devnet/market/i.bin");'),
    ("tools/nonos_seal/chain.py", 'SOURCE = os.path.join("tools", "devnet", "market")'),
    ("tools/nonos_seal/__main__.py", "from . import dev_market"),
    ("nonos-mk/src/commands/sign.rs", '// reads "tools\\devnet\\market"'),
]
ALLOWED = [  # host proofs and file lists read nothing into an image
    ("nonos-mk/tests/dev_catalogue/signed.rs", 'include_str!("../../../tools/devnet/market/index.json")'),
    ("userland/market_proofs/src/x.rs", '"tools/devnet/market"'),
    ("tools/nix/inputs.json", '"tools/devnet/market/index.json",'),
    ("tools/nonos_market_catalogue/test_union.py", '(ROOT / "tools/devnet/market/index.json")'),
    (SEAL, 'SOURCE = "tools/devnet/market/index.json"'),
]


def self_test(root):
    files = {p: (root / p).read_text() for p, _ in scan.PINS}
    doc = json.loads((root / SOURCE).read_text())
    payee = dict(doc, entries=[dict(doc["entries"][1], commerce=dict(doc["entries"][1]["commerce"],
                                                                    payee="0x" + "ab" * 20))])
    cases = [(not scan.pinned(files), True), (not catalogue(doc), True), (not behaviour(*load(root)), True)]
    cases += [(not scan.pinned(dict(files, **{p: files[p].replace(want, "")})), False) for p, want in scan.PINS]
    cases += [(not scan.pinned(dict(files, **{p: files[p].replace(a, b)})), False) for p, a, b in (
        ("tools/nonos_seal/__main__.py", "= not cfg", "= cfg"), ("tools/nonos_seal/capsules.py", "= False", "= True"))]
    cases += [(not scan.pinned(dict(files, **{scan.GATE: files[scan.GATE].replace(
        "if dev_market.chosen():", "if True or dev_market.chosen():")})), False)]
    cases += [(not behaviour(*load(root, lambda s, a=a, b=b: s.replace(a, b))), False) for a, b in FLIPS]
    cases += [(not scan.named(p, line), False) for p, line in NAMING]
    cases += [(not scan.named(p, line), True) for p, line in ALLOWED]
    cases += [(not catalogue(payee), False),
              (not catalogue(dict(doc, entries=[e for e in doc["entries"] if e["listing_id"] != "devtest.free"])), False),
              (not catalogue(dict(doc, schema_version=2)), False)]
    cases += chain.cases(doc, json.loads((root / "nonos-data/market/index.json").read_text()))
    wrong = [i for i, (passed, ok) in enumerate(cases) if passed != ok]
    if wrong:
        print(f"dev-catalogue: self-test failed on cases {wrong}")
        return 1
    print(f"dev-catalogue: self-test passed, {sum(not ok for _, ok in cases)} bad inputs rejected "
          f"and {sum(ok for _, ok in cases)} good accepted")
    return 0
