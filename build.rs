fn main()
{
    println!("cargo:rerun-if-changed=build.rs");

    cc::Build::new()
        //.archiver("llvm-ar")
        .file("src/functions/asm/write_arm_linux.s")
        .compile("write_arm_linux");
}