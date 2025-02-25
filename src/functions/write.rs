/**
 * Imports from asm
 */
#[cfg(all(target_arch = "x86_64", target_os = "linux"))]
extern "C" { fn write_amdl(fd: u8, buf: *const u8, count: usize) -> isize; }

#[cfg(all(target_arch = "aarch64", target_os = "macos"))]
extern "C" { fn write_armx(fd: u8, buf: *const u8, count: usize) -> isize; }

#[cfg(all(target_arch = "aarch64", target_os = "linux"))]
extern "C" { fn write_arml(fd: u8, buf: *const u8, count: usize) -> isize; }

/**
 * Functions
 */

#[cfg(all(target_arch = "aarch64", target_os = "linux"))]
#[inline(always)]
pub fn write(text: &[u8]) { unsafe { write_arml(1, text.as_ptr(), text.len() as usize); } }

#[cfg(all(target_arch = "x86_64", target_os = "linux"))]
#[inline(always)]
pub fn write(text: &[u8]) { unsafe { write_amdl(1, text.as_ptr(), text.len() as usize); } }

#[cfg(all(target_arch = "aarch64", target_os = "macos"))]
#[inline(always)]
pub fn write(text: &[u8]) { unsafe { write_armx(1, text.as_ptr(), text.len() as usize); } }

#[macro_export]
macro_rules! format
{
    ($($arg:expr),*) =>
    {{
        let mut buffer = [0u8; 256];
        let mut index = 0;

        $(
            let bytes = $arg;
            if index + bytes.len() < buffer.len()
            {
                buffer[index..index + bytes.len()].copy_from_slice(bytes);
                index += bytes.len();
            }
        )*

        write(&buffer[..index]);
    }};
}