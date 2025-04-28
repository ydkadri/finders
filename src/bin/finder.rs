use clap::Parser;
use std::io::Error;

use finders::file_finder;
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

    if let Some(query) = cli.search_pattern.as_deref() {
        let case_insensitive = cli.case_insensitive;
        let searcher = searcher::Searcher::new(query, case_insensitive);

        search_files(searcher, paths, verbose)?;
    } else if let Some(pattern) = cli.regex_pattern.as_deref() {
        let re_searcher = searcher::ReSearcher::new(pattern);

        search_files(re_searcher, paths, verbose)?;
    } else {
        for path in paths {
            println!("{:?}", path);
        }
    }

    Ok(())
}
