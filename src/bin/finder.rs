use clap::Parser;
use std::io::Error;

use finders::file_finder;
use finders::output::{
    ColourMode, CountOutput, FilesOnlyOutput, JsonOutput, Outputs, StandardOutput,
};
use finders::search_files;
use finders::searcher;

const FINDERS: &str = r#"
___________.__            .___    __________  _________
\_   _____/|__| ____    __| _/____\______   \/   _____/
 |    __)  |  |/    \  / __ |/ __ \|       _/\_____  \
 |     \   |  |   |  \/ /_/ \  ___/|    |   \/        \
 \___  /   |__|___|  /\____ |\___  >____|_  /_______  /
     \/            \/      \/    \/       \/        \/ "#;

#[derive(Parser)]
#[command(arg_required_else_help = true)]
#[command(author, version, about, long_about = None, before_help = FINDERS)]
struct Cli {
    /// Optional path to operate on, defaults to CWD
    path: Option<String>,

    /// File pattern to filter results
    #[arg(short, long)]
    file_pattern: Option<String>,

    /// Search pattern to match in result files
    #[arg(short, long)]
    search_pattern: Option<String>,

    /// Regex pattern to match in result files
    #[arg(short, long)]
    regex_pattern: Option<String>,

    /// Flag for case insensitive search
    #[arg(short = 'i', long)]
    case_insensitive: bool,

    /// Verbose output details unreadable files
    #[arg(short, long)]
    verbose: bool,

    /// Enable coloured output (force on)
    #[arg(long, conflicts_with = "no_colour")]
    colour: bool,

    /// Disable coloured output (force off)
    #[arg(long, conflicts_with = "colour")]
    no_colour: bool,

    /// Output only file paths with matches (like grep -l)
    #[arg(short = 'l', long, conflicts_with = "count")]
    files_with_matches: bool,

    /// Output match count per file (like grep -c)
    #[arg(short = 'c', long, conflicts_with = "files_with_matches")]
    count: bool,

    /// Output results as JSON
    #[arg(long, conflicts_with_all = ["files_with_matches", "count"])]
    json: bool,
}

fn main() -> Result<(), Error> {
    let cli = Cli::parse();

    // Grab finder values from the command line
    let finder = file_finder::Finder::new(cli.path.as_deref())?;
    let file_pattern = cli.file_pattern.as_deref();

    // Get iterable paths
    let paths = finder.find(file_pattern);

    // Determine if verbose or not
    let verbose = cli.verbose;

    // Determine colour mode from flags and environment
    let colour_mode = ColourMode::from_env(cli.colour, cli.no_colour);

    // Create output handler based on output mode flags
    let mut output: Box<dyn Outputs> = if cli.json {
        Box::new(JsonOutput::new(colour_mode))
    } else if cli.files_with_matches {
        Box::new(FilesOnlyOutput::new(colour_mode))
    } else if cli.count {
        Box::new(CountOutput::new(colour_mode))
    } else {
        Box::new(StandardOutput::new(colour_mode))
    };

    if let Some(query) = cli.search_pattern.as_deref() {
        let case_insensitive = cli.case_insensitive;
        let searcher = searcher::Searcher::new(query, case_insensitive);

        search_files(searcher, paths, verbose, &mut *output)?;
    } else if let Some(pattern) = cli.regex_pattern.as_deref() {
        let re_searcher = match searcher::ReSearcher::new(pattern) {
            Ok(searcher) => searcher,
            Err(e) => {
                eprintln!("Error: Invalid regex pattern: {}", e);
                std::process::exit(1);
            }
        };

        search_files(re_searcher, paths, verbose, &mut *output)?;
    } else {
        // File-only mode (no search pattern)
        for path in paths {
            output.write_file(&path);
        }
    }

    Ok(())
}
