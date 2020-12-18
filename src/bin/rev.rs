use clap::Clap;
use std::io::{Read, stdout, BufWriter, Write, stdin, BufRead, BufReader, ErrorKind};
use std::fs::File;

#[derive(Clap)]
#[clap(version = "0.1", author = "Raphael Peters <raphael.r.peters@gmail.com>")]
struct Opts {
    files: Vec<String>,
}

fn reverse_stream(reader: impl Read, mut writer: impl Write) -> std::io::Result<()> {
    let mut reader = BufReader::new(reader);

    for line in reader.lines() {
        let line = line?;
        let reverse: String = line.chars().rev().collect();
        writer.write_all(reverse.as_bytes())?;
        writer.write_all(b"\n")?;
    }

    Ok(())
}

fn reverse_files(files: Vec<String>) -> std::io::Result<()> {
    let stdout = stdout();
    let mut stdout = BufWriter::new(stdout);

    if files.is_empty() {
        let stdin = stdin();
        let mut stdin = stdin;

        reverse_stream(&mut stdin, &mut stdout)?;
    } else {
        for path in files {
            let mut reader = File::open(&path).map_err(|err| match err.kind() {
                ErrorKind::NotFound => std::io::Error::new(err.kind(), format!("{}: No such file or directory", &path)),
                _ => err
            })?;
            reverse_stream(&mut reader, &mut stdout)?;
        }
    }

    stdout.flush()?;

    Ok(())
}

fn main() {
    let opts: Opts = Opts::parse();

    if let Err(e) = reverse_files(opts.files) {
        eprintln!("zrev: {}", e);
        std::process::exit(1);
    }
}