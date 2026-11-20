use std::{
    collections::BTreeSet,
    io::{self, BufWriter, Read, Write},
};

use clap::Parser;
use regex::bytes::RegexBuilder;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short = 'l', long)]
    show_lines: bool,

    #[arg(short = 'i', long)]
    ignore_case: bool,

    #[arg(short = 'b')]
    l_before: Option<usize>,

    #[arg(short = 'a')]
    l_after: Option<usize>,

    pattern: String,
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let re = RegexBuilder::new(&args.pattern)
        .case_insensitive(args.ignore_case)
        .build()
        .unwrap_or_else(|e| {
            eprintln!("Bad pattern: {e}");
            std::process::exit(2)
        });
    let mut data = Vec::new();
    io::stdin().lock().read_to_end(&mut data)?;
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());
    let lines: Vec<&[u8]> = data.split(|&b| b == b'\n').collect();
    let lines = if data.ends_with(b"\n") {
        &lines[..lines.len() - 1]
    } else {
        &lines[..]
    };
    let hits: Vec<usize> = lines
        .iter()
        .enumerate()
        .filter(|(_, l)| re.is_match(l))
        .map(|(i, _)| i)
        .collect();
    let mut display: BTreeSet<usize> = BTreeSet::new();
    let before = args.l_before.unwrap_or(0);
    let after = args.l_after.unwrap_or(0);
    let last = lines.len().saturating_sub(1);
    for l in hits {
        display.extend(l.saturating_sub(before)..=(l + after.min(last)));
    }
    for &l in &display {
        if args.show_lines {
            write!(out, "{}: ", l + 1)?;
        }
        out.write_all(lines[l])?;
        out.write_all(b"\n")?;
    }
    out.flush()?;
    Ok(())
}
