//! Process management syscalls
use crate::{
    config::MAX_SYSCALL_NUM,
    task::{
        change_program_brk, exit_current_and_run_next, suspend_current_and_run_next, TaskStatus, current_user_token, set_task_info, mmap, munmap
    },
    timer::get_time_ms,
    mm::{
        translate_ptr, VirtPageNum, VirtAddr
    },
};

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// Task information
#[allow(dead_code)]
pub struct TaskInfo {
    /// Task status in it's life cycle
    pub status: TaskStatus,
    /// The numbers of syscall called by task
    pub syscall_times: [u32; MAX_SYSCALL_NUM],
    /// Total running time of task
    pub time: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(_exit_code: i32) -> ! {
    trace!("kernel: sys_exit");
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// YOUR JOB: get time with second and microsecond
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TimeVal`] is splitted by two pages ?
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let ms = get_time_ms(); 
    let ktime = translate_ptr(current_user_token(), ts);
    unsafe {
        *ktime = TimeVal {
            sec: ms / 1_000,
            usec: ms * 1_000,
        };
    }
    0
}

/// YOUR JOB: Finish sys_task_info to pass testcases
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TaskInfo`] is splitted by two pages ?
pub fn sys_task_info(ti: *mut TaskInfo) -> isize {
    trace!("kernel: sys_task_info NOT IMPLEMENTED YET!");
    let kti = translate_ptr(current_user_token(), ti);
    set_task_info(kti);
    0
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(start: usize, len: usize, port: usize) -> isize {
    trace!("kernel: sys_mmap NOT IMPLEMENTED YET!");
    let start_vaddr: VirtAddr = start.into();

    if !start_vaddr.aligned() {
        return -1;
    }

    if port & !0x7 != 0 || port & 0x7 == 0 {
        return -1;
    }
    
    if len == 0 {
        return 0;
    }

    let end_vaddr: VirtAddr = (start + len).into();
    let start_vpn: VirtPageNum = start_vaddr.into();
    let end_vpn: VirtPageNum = end_vaddr.ceil();

    mmap(start_vpn, end_vpn, port)
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(start: usize, len: usize) -> isize {
    trace!("kernel: sys_munmap NOT IMPLEMENTED YET!");
    let start_vaddr: VirtAddr = start.into();

    if !start_vaddr.aligned() {
        return -1;
    }

    if len == 0 {
        return 0;
    }

    let end_vaddr: VirtAddr = (start + len).into();
    let start_vpn: VirtPageNum = start_vaddr.into();
    let end_vpn: VirtPageNum = end_vaddr.ceil();

    munmap(start_vpn, end_vpn)
}
/// change data segment size
pub fn sys_sbrk(size: i32) -> isize {
    trace!("kernel: sys_sbrk");
    if let Some(old_brk) = change_program_brk(size) {
        old_brk as isize
    } else {
        -1
    }
}
