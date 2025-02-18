use clap::Parser;

#[derive(Parser, Debug)]
#[clap(version,
    about = "ffplayout, Rust based 24/7 playout solution.",
    override_usage = "Run without any command to use config file only, or with commands to override parameters:\n\n    ffplayout [OPTIONS]",
    long_about = None)]
pub struct Args {
    #[clap(short, long, help = "File path to ffplayout.yml")]
    pub config: Option<String>,

    #[clap(short, long, help = "File path for logging")]
    pub log: Option<String>,

    #[clap(
        short,
        long,
        help = "Generate playlist for date. Date-range is possible, like: 2022-01-01 - 2022-01-10.",
        name = "YYYY-MM-DD",
        multiple_values=true
    )]
    pub generate: Option<Vec<String>>,

    #[clap(short = 'm', long, help = "Playing mode: folder, playlist")]
    pub play_mode: Option<String>,

    #[clap(short, long, help = "Play folder content")]
    pub folder: Option<String>,

    #[clap(short, long, help = "Path from playlist")]
    pub playlist: Option<String>,

    #[clap(
        short,
        long,
        help = "Start time in 'hh:mm:ss', 'now' for start with first"
    )]
    pub start: Option<String>,

    #[clap(
        short = 't',
        long,
        help = "Set length in 'hh:mm:ss', 'none' for no length check"
    )]
    pub length: Option<String>,

    #[clap(short, long, help = "Loop playlist infinitely")]
    pub infinit: bool,

    #[clap(short, long, help = "Set output mode: desktop, hls, stream")]
    pub output: Option<String>,

    #[clap(short, long, help = "Set audio volume")]
    pub volume: Option<f64>,
}

pub fn get_args() -> Args {
    let args = Args::parse();

    args
}
