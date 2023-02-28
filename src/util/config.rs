use std::path::PathBuf;
use structopt::StructOpt;
/// StructOpt is a library that allows parsing command line arguments
#[derive(StructOpt, Debug)]
#[structopt(
    name = "fastPASTA - fast Protocol Analysis Scanning Tool for ALICE",
    about = "A tool to scan and verify the CRU protocol of the ALICE readout system"
)]
pub struct Opt {
    /// Dump RDHs to stdout or file
    #[structopt(short, long = "dump-rhds")]
    dump_rhds: bool,

    /// Activate sanity checks
    #[structopt(short = "s", long = "sanity-checks")]
    sanity_checks: bool,

    /// Verbosity level
    #[structopt(short = "v", long = "verbosity", default_value = "0")]
    verbosity: u8,

    /// Max tolerate errors before ending processing
    /// if set to 0 -> no limit to errors
    #[structopt(short = "e", long = "tolerate-max-errors", default_value = "50")]
    max_tolerate_errors: u32,

    /// links to filter
    #[structopt(short = "f", long)]
    filter_link: Option<u8>,

    /// File to process
    #[structopt(name = "FILE", parse(from_os_str))]
    file: PathBuf,

    /// Output file
    #[structopt(short, long, parse(from_os_str))]
    output: Option<PathBuf>,
}

impl Opt {
    pub fn dump_rhds(&self) -> bool {
        self.dump_rhds
    }
    pub fn sanity_checks(&self) -> bool {
        self.sanity_checks
    }
    pub fn filter_link(&self) -> Option<u8> {
        self.filter_link
    }
    pub fn file(&self) -> &PathBuf {
        &self.file
    }
    pub fn output(&self) -> &Option<PathBuf> {
        &self.output
    }
    pub fn verbosity(&self) -> u8 {
        self.verbosity
    }
    pub fn max_tolerate_errors(&self) -> u32 {
        self.max_tolerate_errors
    }
}
