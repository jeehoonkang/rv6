use crate::libc;
use crate::proc::{myproc, sleep, wakeup};
use crate::spinlock::Spinlock;

#[derive(Copy, Clone)]
pub struct Sleeplock {
    pub locked: u32,
    pub lk: Spinlock,
    pub name: *mut libc::c_char,
    pub pid: i32,
}

impl Sleeplock {
    // TODO: transient measure
    pub const fn zeroed() -> Self {
        Self {
            locked: 0,
            lk: Spinlock::zeroed(),
            name: 0 as *const libc::c_char as *mut libc::c_char,
            pid: 0,
        }
    }

    /// Sleeping locks
    pub unsafe fn new(name: *mut libc::c_char) -> Self {
        let mut lk = Self::zeroed();

        lk.lk
            .initlock(b"sleep lock\x00" as *const u8 as *const libc::c_char as *mut libc::c_char);
        lk.name = name;
        lk.locked = 0 as u32;
        lk.pid = 0 as i32;

        lk
    }
}

pub unsafe fn initsleeplock(mut lk: *mut Sleeplock, mut name: *mut libc::c_char) {
    (*lk)
        .lk
        .initlock(b"sleep lock\x00" as *const u8 as *const libc::c_char as *mut libc::c_char);
    (*lk).name = name;
    (*lk).locked = 0 as u32;
    (*lk).pid = 0 as i32;
}

pub unsafe fn acquiresleep(mut lk: *mut Sleeplock) {
    (*lk).lk.acquire();
    while (*lk).locked != 0 {
        sleep(lk as *mut libc::c_void, &mut (*lk).lk);
    }
    (*lk).locked = 1 as u32;
    (*lk).pid = (*myproc()).pid;
    (*lk).lk.release();
}

pub unsafe fn releasesleep(mut lk: *mut Sleeplock) {
    (*lk).lk.acquire();
    (*lk).locked = 0 as u32;
    (*lk).pid = 0 as i32;
    wakeup(lk as *mut libc::c_void);
    (*lk).lk.release();
}

pub unsafe fn holdingsleep(mut lk: *mut Sleeplock) -> i32 {
    let mut r: i32 = 0;
    (*lk).lk.acquire();
    r = ((*lk).locked != 0 && (*lk).pid == (*myproc()).pid) as i32;
    (*lk).lk.release();
    r
}
