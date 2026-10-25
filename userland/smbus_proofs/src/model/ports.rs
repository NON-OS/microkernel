// NONOS Operating System (AGPL-3.0-or-later)
//! The model behind the driver's `Ports`: register reads and writes with
//! their side effects, and a clock the waits advance.

use super::host::Model;
use crate::ports::Ports;
use crate::regs::*;

impl Ports for Model {
    fn r8(&mut self, off: u16) -> Result<u8, i32> {
        Ok(match off {
            HSTSTS => self.read_status(),
            HSTCNT => {
                self.idx = 0;
                self.cnt
            }
            HSTDAT0 => self.dat0,
            HSTDAT1 => self.dat1,
            BLKDAT => self.next_block_byte(),
            AUXCTL => self.aux,
            _ => 0,
        })
    }

    fn w8(&mut self, off: u16, v: u8) -> Result<(), i32> {
        self.writes.push((off, v));
        match off {
            HSTSTS => self.sts &= !(v & (STS_FLAGS | STS_INUSE)),
            HSTCNT => self.control(v),
            HSTCMD => self.cmd = v,
            HSTADD => self.add = v,
            HSTDAT0 => self.dat0 = v,
            HSTDAT1 => self.dat1 = v,
            BLKDAT => self.put_block_byte(v),
            AUXCTL => self.aux = if self.e32b { v } else { v & !AUX_E32B },
            _ => {}
        }
        Ok(())
    }

    fn arm(&mut self, ms: u64) {
        self.deadline = self.now + ms;
    }

    fn expired(&mut self) -> bool {
        self.now >= self.deadline
    }

    fn pause(&mut self, ms: u64) {
        self.now += ms;
    }
}

impl Model {
    /// Reading the status runs the host on and takes the semaphore.
    fn read_status(&mut self) -> u8 {
        if let Some(bits) = self.result {
            if self.lag == 0 {
                (self.sts, self.result) = (self.sts | bits, None);
            } else {
                self.lag -= 1;
            }
        } else if self.sts & STS_HOST_BUSY != 0 && !self.stuck {
            if self.pending == 0 {
                self.finish();
            } else {
                self.pending -= 1;
            }
        }
        let s = self.sts;
        self.sts |= STS_INUSE;
        s
    }
}
