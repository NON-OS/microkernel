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

//! The typed-data custody tests: what the wallet admits, hashes, checks and
//! shows for a typed-data signing request, held to one shape.

#[path = "typed_admit_tests.rs"]
mod typed_admit_tests;
#[path = "typed_check_tests.rs"]
mod typed_check_tests;
#[path = "typed_config_tests.rs"]
mod typed_config_tests;
#[path = "typed_fixture.rs"]
mod typed_fixture;
#[path = "typed_hash_tests.rs"]
mod typed_hash_tests;
#[path = "typed_live_tests.rs"]
mod typed_live_tests;
#[path = "typed_request_tests.rs"]
mod typed_request_tests;
#[path = "typed_shape_tests.rs"]
mod typed_shape_tests;
#[path = "typed_spec_tests.rs"]
mod typed_spec_tests;
#[path = "typed_word_tests.rs"]
mod typed_word_tests;
