fn main() {
    // read env variables that were set in build script
    let bios_path = env!("BIOS_PATH");
    println!("{}", bios_path);

    let mut cmd = std::process::Command::new("qemu-system-x86_64");
    cmd.arg("-drive")
        .arg(format!("format=raw,file={bios_path}"));
    // .arg("-device")
    // .arg("isa-debug-exit,iobase=0xf4,iosize=0x04")
    // .arg("-serial")
    // .arg("stdio")
    // .arg("-display")
    // .arg("none");

    let mut child = cmd.spawn().unwrap();
    child.wait().unwrap();
}
