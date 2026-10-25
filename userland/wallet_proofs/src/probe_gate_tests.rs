// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

/*
 * A source gate for the network probe: every probe is traffic an observer can
 * see and time, so it opens the one exchange the status needs and nothing
 * else, and never on a route the wallet refuses.
 */

fn wallet(path: &str) -> String {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    std::fs::read_to_string(root.join("../capsule_wallet_nonos/src/wallet").join(path)).unwrap()
}

fn probe() -> String {
    ["net/step/probe.rs", "net/step/probe_end.rs", "net/step/probe_report.rs"].map(wallet).concat()
}

#[test]
fn the_probe_opens_one_exchange_and_nothing_else() {
    let code = probe();
    assert_eq!(code.matches("Exchange::begin(").count(), 1, "one stream per probe");
    for opens in
        ["health(", "lookup(", "mk_service_lookup", "mk_ipc", "client_flight", "self_check"]
    {
        assert!(!code.contains(opens), "the probe must not reach {opens}");
    }
}

#[test]
fn the_stream_opens_only_on_a_route_the_wallet_takes() {
    let code = wallet("net/step/probe.rs");
    let refused = code.find("refused(self.route)").unwrap();
    let begin = code.find("Exchange::begin(").unwrap();
    assert!(refused < begin, "the route is judged before anything is opened");
    assert!(
        code.contains("self.route = WalletRoute::for_wallet()"),
        "the wallet's route, never Direct"
    );
}

#[test]
fn the_unread_self_checks_are_gone() {
    let root =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../capsule_wallet_nonos/src/wallet");
    for gone in ["net/health.rs", "net/lookup.rs", "net/constants.rs", "rpc/self_check.rs"] {
        assert!(!root.join(gone).exists(), "{gone} came back");
    }
}
