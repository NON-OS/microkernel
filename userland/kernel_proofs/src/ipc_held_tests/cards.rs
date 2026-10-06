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

//! The network cards: each takes its stack, and no app.

use super::common::{owner, IPC, NETWORK, WIFI, WIRED};
use super::held::{callers_of, endpoint_admits, inbox_admits};

#[test]
fn a_card_refuses_an_app() {
    for card in WIRED.iter().chain(WIFI.iter()) {
        assert!(!endpoint_admits(card, owner(&[])), "{card} took a caller owning nothing");
        assert!(!endpoint_admits(card, owner(&["app.browser"])), "{card} took the browser");
        assert!(
            !endpoint_admits(card, owner(&["app.attack", "endpoint.app.attack.reply"])),
            "{card}"
        );
        // Network is not the key: the browser holds it and still drives no card.
        assert!(!inbox_admits([(*card, IPC)], IPC | NETWORK, owner(&["app.browser"])), "{card}");
    }
}

#[test]
fn each_card_takes_its_stack() {
    for card in WIRED {
        assert!(endpoint_admits(card, owner(&["net.core"])), "{card}");
        assert!(endpoint_admits(card, owner(&["net.l2"])), "{card}");
        assert!(!endpoint_admits(card, owner(&["app.settings"])), "{card} took Settings");
        assert!(!endpoint_admits(card, owner(&["app.setup_wizard"])), "{card} took the wizard");
    }
    for card in WIFI {
        for caller in
            ["net.core", "app.settings", "app.settings.1", "app.settings.2", "app.setup_wizard"]
        {
            let one: &'static [&'static str] = Box::leak(Box::new([caller]));
            assert!(endpoint_admits(card, owner(one)), "{card} refused {caller}");
        }
        assert!(
            !endpoint_admits(card, owner(&["net.l2"])),
            "{card} took net.l2, which binds only wired cards"
        );
        assert!(
            !endpoint_admits(card, owner(&["app.settings.3"])),
            "{card} took a window Settings has not got"
        );
    }
}

#[test]
fn a_name_close_to_a_service_is_not_it() {
    for caller in ["net.core.1", "net.cor", "Net.core", "app.setup_wizard.1", "app.settings.10", ""]
    {
        let one: &'static [&'static str] = Box::leak(Box::new([caller]));
        assert!(!endpoint_admits("driver.iwlwifi0", owner(one)), "{caller:?}");
    }
    for card in ["driver.e1000_1", "driver.e1000", "driver.iwlwifi", "Driver.iwlwifi0"] {
        assert!(callers_of(card).is_none(), "{card} is not a registered card name");
    }
}
