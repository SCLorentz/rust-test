use crate::write;

#[cfg(all(target_arch = "aarch64", target_os = "linux"))]
extern "C" { fn read_arml(fd: u8, buf: *const u8, count: usize) -> isize; }

#[cfg(all(target_arch = "aarch64", target_os = "linux"))]
#[inline(always)]
pub fn read(text: &[u8]) -> [u8; 64]
{
    write(text);

    let mut input = [0u8; 64];
    unsafe { read_arml(0, input.as_mut_ptr(), input.len()); }
    input
}