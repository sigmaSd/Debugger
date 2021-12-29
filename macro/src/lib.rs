#![feature(proc_macro_span)]
use proc_macro::{LineColumn, Span, TokenStream};
use std::io::Write;

#[proc_macro]
pub fn dbg(_item: TokenStream) -> TokenStream {
    (|| {
        let call_site = Span::call_site();

        let file = call_site.source_file().path().canonicalize().ok()?;
        let LineColumn { line, column: _ } = call_site.start();

        let exe_name = {
            let mut exe_path = file.clone();
            loop {
                let path = exe_path.parent()?;
                if path.file_name()? == "src" {
                    break path.parent()?.file_name()?.to_str()?.to_string();
                }
                exe_path = path.to_path_buf();
            }
        };

        let mut bot_stdin = std::fs::File::create(std::env::temp_dir().join("bot_stdin")).ok()?;
        writeln!(bot_stdin, "{}", file.display().to_string()).ok()?;
        writeln!(bot_stdin, "{}", line).ok()?;
        writeln!(bot_stdin, "{}", exe_name).ok()
    })(); // Ignore any failure, dbg-bot should figure it out on its own

    TokenStream::new()
}
