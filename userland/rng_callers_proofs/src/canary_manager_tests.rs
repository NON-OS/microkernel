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

// No canary with neither source, and the manager that starts the canary,
// included into the canary's proofs.

#[test]
fn no_canary_is_made_with_neither_source() {
    arch::cpu_answers(None);
    rng::refuse();
    assert_eq!(StackCanary::new().err(), Some(NONE));
    assert_eq!(rng::weak(), 0);
}

#[test]
fn the_manager_is_not_made_or_started_with_no_randomness() {
    arch::cpu_answers(None);
    rng::refuse();
    assert!(AdvancedSecurityManager::new(SecurityConfig::default()).is_err());
    rng::fail_from(None);
    let m = AdvancedSecurityManager::new(SecurityConfig::default()).expect("manager");
    rng::refuse();
    assert_eq!(m.init(), Err(NONE));
    assert_eq!(seen(&m.stack_canary).2, 0);
    assert!(!m.cfi.enabled.load(Ordering::SeqCst));
    rng::fail_from(None);
    assert_eq!(m.init(), Ok(()));
    assert_eq!(seen(&m.stack_canary).2, 1);
    assert!(m.cfi.enabled.load(Ordering::SeqCst));
}

/// The one test that starts the global manager.
#[test]
fn advanced_security_refuses_to_start_with_no_randomness() {
    arch::cpu_answers(None);
    for (from, only) in rng::cases(2) {
        rng::fail_at(Some(from), only);
        assert_eq!(init_advanced_security(), Err(NONE));
    }
    rng::fail_from(None);
    assert_eq!(init_advanced_security(), Ok(()));
}
