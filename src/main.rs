use std::io;
use std::process;
use mdbook::preprocess::{CmdPreprocessor, Preprocessor};
use mdbook::book::Book;
use mdbook_tooltips::features::parser::TooltipParser;

mod cli;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    // Check if this is being run as an mdBook preprocessor
    // mdBook calls with "supports <renderer>" or provides input on stdin for preprocessing
    if args.len() > 1 && args[1] == "supports" {
        // Handle the supports check
        if args.len() > 2 && args[2] == "html" {
            // We support HTML renderer
            std::process::exit(0);
        } else {
            // We don't support other renderers
            std::process::exit(1);
        }
    } else if is_stdin_piped() {
        // Stdin is piped, so we're being called as a preprocessor
        if let Err(e) = handle_mdbook_preprocessor() {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    } else {
        // Fall back to CLI mode
        cli::run();
    }
}

#[cfg(unix)]
fn is_stdin_piped() -> bool {
    use std::os::unix::io::AsRawFd;
    unsafe { libc::isatty(io::stdin().as_raw_fd()) == 0 }
}

#[cfg(windows)]
fn is_stdin_piped() -> bool {
    // On Windows, assume piped if not in interactive console
    // A more robust approach would use the Windows Console API
    std::env::var("TERM").is_err() || std::env::var("PIPE").is_ok()
}

fn handle_mdbook_preprocessor() -> Result<(), Box<dyn std::error::Error>> {
    let (ctx, book) = CmdPreprocessor::parse_input(io::stdin())?;
    let mut book = book;

    if ctx.config.get_preprocessor("tooltips").is_some() {
        let preprocessor = TooltipsPreprocessor;
        book = preprocessor.run(&ctx, book)?;
    }

    serde_json::to_writer(io::stdout(), &book)?;
    Ok(())
}

struct TooltipsPreprocessor;

impl Preprocessor for TooltipsPreprocessor {
    fn name(&self) -> &str {
        "tooltips"
    }

    fn run(&self, _ctx: &mdbook::preprocess::PreprocessorContext, mut book: Book) -> Result<Book, mdbook::errors::Error> {
        let parser = TooltipParser::new();

        // Process each chapter
        book.for_each_mut(|item| {
            if let mdbook::book::BookItem::Chapter(ref mut chapter) = item {
                chapter.content = parser.parse(&chapter.content);
            }
        });

        Ok(book)
    }
}

