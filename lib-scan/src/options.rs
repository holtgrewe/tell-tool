use clap::ArgMatches;

/// Options for the "scan" command.
#[derive(Clone, Debug)]
pub struct ScanOptions {
    // Generic arguments
    /// Number of background threads to use for compression/decompression in I/O.
    pub io_threads: u32,

    /// Path to input BAM file.
    pub input_bam: String,
    /// Path to output discordant reads BAM file.
    pub output_bam: String,
    /// Path to output sites BCF file.
    pub output_bcf: String,
}

impl ScanOptions {
    /// Build new options from ArgMatches.
    pub fn new(matches: &ArgMatches) -> Self {
        Self {
            input_bam: matches.value_of("input_bam").unwrap().to_string(),
            output_bam: matches.value_of("output_bam").unwrap().to_string(),
            output_bcf: matches.value_of("output_bcf").unwrap().to_string(),
            io_threads: matches
                .value_of("io_threads")
                .unwrap()
                .parse::<u32>()
                .unwrap(),
        }
    }
}
