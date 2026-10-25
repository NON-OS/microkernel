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

//! Proving and publishing a private payment or a withdrawal.

use shield_wire::Values;

use super::args::{coin, flag};
use super::prove::{cancel_token, proved, prover};
use super::publish::publish;
use super::{refuse, Answer};

pub fn send(f: &[&str]) -> Answer {
    let [c, to, amount, early] = f else { return refuse("Name the coin, payee, amount and wait.") };
    let w = prover()?;
    let t = w.send_private(
        coin(c)?,
        String::from(*to),
        String::from(*amount),
        flag(early),
        cancel_token()?,
    );
    proved();
    let t = t.map_err(|e| e.to_string())?;
    let mut v = Values::new();
    v.put("proved", "1");
    for weak in &t.weakened {
        v.put("weakened", weak);
    }
    publish(&mut v, &w);
    Ok(v)
}

pub fn withdraw(f: &[&str]) -> Answer {
    let [c, to, amount, early] = f else {
        return refuse("Name the coin, address, amount and wait.");
    };
    let w = prover()?;
    let t = w.withdraw(
        coin(c)?,
        String::from(*to),
        String::from(*amount),
        flag(early),
        cancel_token()?,
    );
    proved();
    let t = t.map_err(|e| e.to_string())?;
    let mut v = Values::new();
    v.put("proved", "1");
    for weak in &t.weakened {
        v.put("weakened", weak);
    }
    publish(&mut v, &w);
    Ok(v)
}
