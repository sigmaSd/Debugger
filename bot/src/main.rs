use scolor::ColorExt;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    loop {
        // Wait for ok
        println!("{}", "Press Ok when ready".light_blue().italic());
        std::io::stdin().read_line(&mut String::new())?;

        // Build the target crate
        println!("{}", "Building the target crate.".green().italic());
        let status = std::process::Command::new("cargo")
            .arg("+nightly")
            .arg("b")
            .spawn()?
            .wait()?;

        if !status.success() {
            println!("{}", "Compiling failed".yellow().italic());
            println!();
            continue;
        }

        // Read instructions
        println!("{}", "Reading Bot instructions.".green().italic());
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
        println!("{}", "Writing gdb commands.".green().italic());
        let gdb_cmds_path = ::std::path::Path::new("/tmp/gdb_bot");
        ::std::fs::write(&gdb_cmds_path, format!("b {}:{}\nr", file, line))?;

        // run gdb
        println!("{}", "Spawning gdb.".green().italic());
        ::std::process::Command::new("rust-gdb")
            .args(&["-x", &gdb_cmds_path.display().to_string()])
            .arg(exe)
            .spawn()?
            .wait()?;
        println!();
    }
}
