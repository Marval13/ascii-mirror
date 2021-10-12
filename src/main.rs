use mir::mirror_text;
use std::fs::File;
use std::io::{BufRead, BufReader};
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(author)]
/// mir is a small utility for mirroring ascii art.
///
/// By default output is printed on stdout,
/// unless an output file is given via the -output option.
///
struct Opt {
    /// Input file
    input: std::path::PathBuf,

    /// Output file
    #[structopt(short = "o", long = "output")]
    output: Option<std::path::PathBuf>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();

    let input = File::open(opt.input)?;
    let buf = BufReader::new(input);

    let mirrored = mirror_text(buf.lines().filter_map(|l| l.ok()).collect()).join("\n");

    if let Some(path) = opt.output {
        //let mut output = File::create(path)?;
        std::fs::write(path, mirrored)?;
    } else {
        print!("{}", mirrored);
    }

    Ok(())
}
