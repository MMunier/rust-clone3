#![feature(asm)]
use libc::{self, pid_t};
use std::io;

// DAS IST V2 aber mein kernel is zu alt ...
#[repr(align(8))]
#[repr(C)]
struct clone_args {
    flags: u64,        /* Flags bit mask */
    pidfd: u64,        /* Where to store PID file descriptor (pid_t *) */
    child_tid: u64,    /* Where to store child TID:u64, in child's memory (pid_t *) */
    parent_tid: u64,   /* Where to store child TID, in parent's memory (int *) */
    exit_signal: u64,  /* Signal to deliver to parent on child termination */
    stack: u64,        /* Pointer to lowest byte of stack */
    stack_size: u64,   /* Size of stack */
    tls: u64,          /* Location of new TLS */
    set_tid: u64,      /* Pointer to a pid_t array */
    set_tid_size: u64, /* Number of elements in set_tid */
}


unsafe fn clone3(args: &clone_args) -> io::Result<Option<i32>> {
    let ret: pid_t;
    println!("{}", std::mem::size_of::<clone_args>());
    asm!(
        "syscall",
        in("rax") libc::SYS_clone3,
        in("rdi") args as *const _,
        in("rsi") std::mem::size_of::<clone_args>(),
        out("rdx") _,
        out("rcx") _,
        out("r11") _,
        lateout("rax") ret,
    );

    match ret {
        0 => Ok(None),
        x if x > 0 => Ok(Some(x)),
        x => {
            *libc::__errno_location() = - x;
            Err(io::Error::last_os_error())
        },
    }
}

fn main() {
    let tid_arr: [pid_t; 1] = [50000];
    let args = clone_args {
        flags: 0,
        exit_signal: libc::SIGCHLD as _,
        tls: 0,
        child_tid: 0,
        parent_tid: 0,
        pidfd: 0,
        stack: 0,
        stack_size: 0,
        set_tid: &tid_arr as *const _ as _,
        set_tid_size: 0,
    };
    let res = unsafe { clone3(&args) }; 
    println!("CLONE-RESULT: {:?}", res);
}
