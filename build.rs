fn main()
{
    println!("cargo:rerun-if-changed=build.rs");
    
    #[cfg(all(target_arch = "aarch64", target_os = "linux"))]
    cc::Build::new()
        .file("src/functions/core/exit_arml.s")
        .compile("exit_arml");

    #[cfg(all(target_arch = "x86_64", target_os = "linux"))]
    cc::Build::new()
        .file("src/functions/core/exit_amdl.s")
        .compile("exit_amdl");

    #[cfg(all(target_arch = "aarch64", target_os = "macos"))]
    cc::Build::new()
        .file("src/functions/core/exit_armx.s")
        .compile("exit_armx");
}