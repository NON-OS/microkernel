//! The host's role table and revocation list, owned. Each is verified when
//! it is installed: the table under the key id the host pins and no older
//! than the serial it last accepted, the list under that table and no older
//! than the list it replaces. Every check opens both again from the bytes
//! the store owns, so no parsed view outlives what it was read from.

use crate::Roles;
use alloc::vec::Vec;
use core::cell::{Cell, RefCell};
use nsh_roles::{open_policy, open_revocations, Policy, PolicyError, Revocations};

pub struct RoleStore {
    table: Vec<u8>,
    anchor: [u8; 32],
    serial: u64,
    list: RefCell<Vec<u8>>,
    issued_ms: Cell<u64>,
}

impl RoleStore {
    /// `min_serial` is the highest table serial this host has installed.
    pub fn new(
        table: Vec<u8>,
        anchor: [u8; 32],
        min_serial: u64,
        list: Vec<u8>,
    ) -> Result<Self, PolicyError> {
        let p = open_policy(&table, &anchor, min_serial)?;
        let issued_ms = open_revocations(&list, &p, 0)?.list().issued_ms;
        let serial = p.serial;
        Ok(RoleStore {
            table,
            anchor,
            serial,
            list: RefCell::new(list),
            issued_ms: Cell::new(issued_ms),
        })
    }

    /// Install a newer revocation list. One older than the list held is
    /// refused, the same one may be installed again, and on any refusal the
    /// list held stays.
    pub fn install_list(&self, list: Vec<u8>) -> Result<(), PolicyError> {
        let p = open_policy(&self.table, &self.anchor, self.serial)?;
        let issued_ms = open_revocations(&list, &p, self.issued_ms.get())?.list().issued_ms;
        *self.list.borrow_mut() = list;
        self.issued_ms.set(issued_ms);
        Ok(())
    }

    /// The installed table's serial: the host keeps it, and passes it as
    /// `min_serial` when it next installs a table.
    pub fn serial(&self) -> u64 {
        self.serial
    }

    /// When the installed list was issued.
    pub fn issued_ms(&self) -> u64 {
        self.issued_ms.get()
    }
}

impl Roles for RoleStore {
    fn with<T>(
        &self,
        f: impl FnOnce(&Policy<'_>, &Revocations<'_>) -> T,
    ) -> Result<T, PolicyError> {
        let p = open_policy(&self.table, &self.anchor, self.serial)?;
        let list = self.list.borrow();
        let r = open_revocations(&list, &p, self.issued_ms.get())?;
        Ok(f(&p, &r))
    }
}
