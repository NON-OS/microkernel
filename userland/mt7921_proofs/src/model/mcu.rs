// NONOS Operating System (AGPL-3.0-or-later)

//! The MCU's state: every command it was sent, the patch semaphore, the
//! download in progress and the pieces received whole, whether its RAM
//! code was started, and the channel its manager holds (a granted request,
//! else the associated BSS's).

/// One command as the MCU read it.
#[derive(Clone, Debug)]
pub struct Seen {
    pub uni: bool,
    /// The command id (`cid`); for an extended command, 0xed.
    pub cid: u16,
    pub ext: u8,
    pub seq: u8,
    pub set_query: u8,
    pub body: Vec<u8>,
}

/// A download piece: the announcement and the bytes that followed it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Piece {
    pub cmd: u8,
    pub addr: u32,
    pub len: u32,
    pub mode: u32,
    pub bytes: Vec<u8>,
}

#[derive(Default)]
pub struct Mcu {
    pub seen: Vec<Seen>,
    /// The chip already holds its patch (a warm start).
    pub patched: bool,
    pub sem_held: bool,
    pub piece: Option<Piece>,
    pub pieces: Vec<Piece>,
    /// `FW_START_REQ`'s option and address.
    pub started: Option<(u32, u32)>,
    pub running: bool,
    pub roc: Option<u8>,
    pub bss: Option<u8>,
    /// Channel requests made.
    pub grants: u32,
}

impl Mcu {
    /// The channel the radio is on, if any.
    pub fn tuned(&self) -> Option<u8> {
        self.roc.or(self.bss)
    }

    pub fn scatter(&mut self, bytes: &[u8]) {
        let Some(p) = self.piece.as_mut() else { return };
        p.bytes.extend_from_slice(bytes);
        if p.bytes.len() >= p.len as usize {
            self.pieces.extend(self.piece.take());
        }
    }

    /// The unified commands with id `cid`, in order.
    pub fn uni(&self, cid: u16) -> Vec<&Seen> {
        self.seen.iter().filter(|s| s.uni && s.cid == cid).collect()
    }

    /// The other commands with id `cid` (0xed with `ext`), in order.
    pub fn plain(&self, cid: u16, ext: u8) -> Vec<&Seen> {
        self.seen.iter().filter(|s| !s.uni && s.cid == cid && s.ext == ext).collect()
    }
}
