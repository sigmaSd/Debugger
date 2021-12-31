type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    loop {
        // Wait for ok
        println!("Press Ok when ready");
        std::io::stdin().read_line(&mut String::new())?;

        // Build the target crate
        std::process::Command::new("cargo")
            .arg("+nightly")
            .arg("b")
            .spawn()?
            .wait()?;

        // Read instructions
        let contents = std::fs::read_to_string(std::env::temp_dir().join("/tmp/bot_stdin"))?;
        let mut contents = contents.lines();
        let file = contents.next().ok_or("Missing file")?;
        let line = contents.next().ok_or("Missing line")?;
        let exe_name = contents.next().ok_or("Missing exe name")?;

        // exe path
        let exe = {
            let target = std::env::var("CARGO_TARGET_DIR").unwrap_or_else(|_| "target".to_owned());
            std::path::Path::new(&target).join("debug").join(exe_name)
        };

        // gdb cmds
        let gdb_cmds_path = ::std::path::Path::new("/tmp/gdb_bot");
        ::std::fs::write(&gdb_cmds_path, format!("b {}:{}\nr", file, line))?;

        // run gdb
        ::std::process::Command::new("rust-gdb")
            .args(&["-x", &gdb_cmds_path.display().to_string()])
            .arg(exe)
            .spawn()?
            .wait()?;
    }
}
