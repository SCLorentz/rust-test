fn main()
{
    println!("cargo:rerun-if-changed=build.rs");
    
    #[cfg(target_arch = "aarch64")]
    cc::Build::new()
        .archiver("llvm-ar") // Se necessário, especifique um arquivador
        .file("src/functions/write_arm_linux.s")
        .compile("write_arm_linux");
}