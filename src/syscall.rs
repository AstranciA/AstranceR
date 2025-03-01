use core::arch::asm;

use num_enum::TryFromPrimitive;

#[derive(TryFromPrimitive)]
//#[num_enum(error_type(name = UnsupportSyscallError, constructor = UnsupportSyscallError::new))]
#[repr(usize)]
pub enum SyscallId {
    SyscallWrite = 64,
    SyscallExit = 93,
    SyscallYield = 124,
    SyscallGetTime = 169,
}

fn syscall(id: SyscallId, args: [usize; 3]) -> isize {
    let mut ret: isize;
    unsafe {
        asm!(
            "ecall",
            inlateout("x10") args[0] => ret,
            in("x11") args[1],
            in("x12") args[2],
            in("x17") id as usize
        );
    }

    ret
}

pub fn sys_write(fd: usize, buffer: &[u8]) -> isize {
    syscall(SyscallId::SyscallWrite, [fd, buffer.as_ptr() as usize, buffer.len()])
}

pub fn sys_exit(exit_code: i32) -> isize {
    syscall(SyscallId::SyscallExit, [exit_code as usize, 0, 0])
}

pub fn sys_yield() -> isize {
    syscall(SyscallId::SyscallYield, [0, 0, 0])
}

pub fn sys_get_time() -> isize {
    syscall(SyscallId::SyscallGetTime, [0, 0, 0])
}
