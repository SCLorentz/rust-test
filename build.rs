fn main()
{
    println!("cargo:rerun-if-changed=build.rs");
    
    #[cfg(target_arch = "aarch64")]
    cc::Build::new()
        .file("src/functions/asm/exit_arml.s")
        .compile("exit_arml");

    #[cfg(target_arch = "x86_64")]
    cc::Build::new()
        .file("src/functions/asm/exit_amdl.s")
        .compile("exit_amdl");
}