//! Process management syscalls
use crate::{
    task::{exit_current_and_run_next, suspend_current_and_run_next},
    timer::get_time_us,
};

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> ! {
    trace!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// get time with second and microsecond
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let us = get_time_us();
    unsafe {
        *ts = TimeVal {
            sec: us / 1_000_000,
            usec: us % 1_000_000,
        };
    }
    0
}

///handle systrace
pub fn sys_trace(_trace_request: usize, _id: usize, _data: usize) -> isize {
    match _trace_request {
        0 => {
            let id_ptr = _id as *const u8; // 将 id 视作指针
            unsafe {
                // 读取指针地址处的值
                *id_ptr as isize
            }
        }

        1 => {
            let id_ptr = _id as *mut u8; // 将 id 视作可变指针
            unsafe {
                // 写入 data 的最低字节到指针地址处
                *id_ptr = _data as u8;
            }
            0
        }

        2 => {
            use crate::task::get_current_syscall_count; // 导入获取系统调用计数的函数
            let syscall_counts = get_current_syscall_count();
            syscall_counts[_id] as isize // 返回编号为 id 的系统调用次数
        }

        _ => {
            panic!("Unsupported trace_request: {}", _trace_request);
        }
    }
}
