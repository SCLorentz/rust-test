use crate::functions::asm;

#[cfg(all(target_arch = "aarch64", target_os = "linux"))]
pub fn open_file(path: &[u8]) -> usize
{
    let path_ptr = path.as_ptr();
    let path_len = path.len() as usize;

    let syscall_ret: u64;
    unsafe {
        asm!(
            "mov x0, {path_ptr}",
            "mov x1, {path_len}",
            "mov x8, 2",                    // sys_open
            "svc 0x80",
            path_ptr = in(reg) path_ptr,
            path_len = in(reg) path_len,
            out("x0") syscall_ret,
            options(nostack, preserves_flags)
        );
    }

    syscall_ret as usize
}

#[cfg(all(target_arch = "x86_64", target_os = "linux"))]
pub fn open_file(path: &[u8]) -> usize { unimplemented!() }

#[cfg(all(target_arch = "aarch64", target_os = "macos"))]
pub fn open_file(path: &[u8]) -> usize { unimplemented!() }

#[cfg(all(target_arch = "aarch64", target_os = "linux"))]
#[allow(unused)]
pub fn read_file(file_descriptor: usize) -> &'static [u8]
{ 
    unimplemented!()
}