use std::fs;
use std::path::{Path, PathBuf};
use clap::{Parser, Subcommand};
use mdbook_tooltips::features::parser::TooltipParser;
use toml::{Table, Value};

#[derive(Parser, Debug)]
#[command(author, version, about = "mdBook tooltip preprocessor with tooltip parsing and installation tools", long_about = None)]
struct Args {
	#[command(subcommand)]
	command: Option<Command>,
}

#[derive(Subcommand, Debug)]
enum Command {
	/// Parse and replace tooltip patterns in a file
	Parse {
		/// Input file path containing text with tooltip patterns
		#[arg(value_name = "FILE")]
		input: PathBuf,

		/// Output file path (if not specified, prints to stdout)
		#[arg(short, long, value_name = "FILE")]
		output: Option<PathBuf>,
	},
    
	/// Install tooltip assets (JS and CSS) to mdBook directory
	Install {
		/// mdBook root directory (should contain book.toml)
		#[arg(short, long, value_name = "DIR", default_value = ".")]
		book_dir: PathBuf,
	},
}

pub fn run() {
	let args = Args::parse();

	match args.command {
		Some(Command::Parse { input, output }) => {
			parse_command(&input, &output);
		}
		Some(Command::Install { book_dir }) => {
			install_command(&book_dir);
		}
		None => {
			// Default: show help
			println!("mdBook Tooltips Preprocessor");
			println!("\nUsage: mdbook-tooltips <COMMAND>");
			println!("\nCommands:");
			println!("  parse    Parse and replace tooltip patterns in a file");
			println!("  install  Install tooltip assets to mdBook directory");
			println!("\nRun with --help for more information");
		}
	}
}

fn parse_command(input: &PathBuf, output: &Option<PathBuf>) {
	// Read the input file
	let content = fs::read_to_string(input)
		.expect(&format!("Failed to read input file: {}", input.display()));

	// Create parser and process the content
	let parser = TooltipParser::new();
	let processed_content = parser.parse(&content);

	// Output the result
	match output {
		Some(output_path) => {
			fs::write(output_path, &processed_content)
				.expect(&format!("Failed to write to output file: {}", output_path.display()));
			println!("✓ Successfully processed and wrote to: {}", output_path.display());
		}
		None => {
			println!("{}", processed_content);
		}
	}
}

fn install_command(book_dir: &Path) {
	println!("📦 Installing tooltip assets to mdBook...");

	// Verify book.toml exists
	let book_toml_path = book_dir.join("book.toml");
	if !book_toml_path.exists() {
		eprintln!("❌ Error: book.toml not found in {}", book_dir.display());
		eprintln!("   Please run this command from your mdBook directory or specify --book-dir");
		std::process::exit(1);
	}

	// Create theme directory structure
	let theme_dir = book_dir.join("theme");
	let css_dir = theme_dir.join("css");
	let js_dir = theme_dir.join("js");

	println!("📁 Creating theme directories...");
	if let Err(e) = fs::create_dir_all(&css_dir) {
		eprintln!("❌ Failed to create CSS directory: {}", e);
		std::process::exit(1);
	}

	if let Err(e) = fs::create_dir_all(&js_dir) {
		eprintln!("❌ Failed to create JS directory: {}", e);
		std::process::exit(1);
	}

	// Copy CSS files
	println!("📄 Copying CSS files...");

		// CSS content
		let css_content = r#".tooltip-word {
			position: relative;
			cursor: help;
			color: #0077cc;
			text-decoration: underline dotted;
		}

		.tooltip-word .tooltip-text {
			visibility: hidden;
			width: 250px;
			background-color:rgba(0, 0, 0, 0.8);
			color: #fff;
			font-size:x-small;
			text-align: center;
			border-radius: 6px;
			padding: 6px;
			position: absolute;
			z-index: 1;
			bottom: 125%; /* Position above the word */
			left: 50%;
			transform: translateX(-50%);
			opacity: 0;
			transition: opacity 0.3s;
			white-space: normal;
		}

		.tooltip-word:hover .tooltip-text {
			visibility: visible;
			opacity: 1;
		}
"#;

		// JS content
		let js_content = r#"const tooltip = document.getElementById('tooltipText');
	const target = document.getElementById('tooltipTarget');

	target.addEventListener('mouseenter', () => {
		tooltip.style.visibility = 'visible';
		tooltip.style.opacity = '1';
	});

	target.addEventListener('mouseleave', () => {
		tooltip.style.visibility = 'hidden';
		tooltip.style.opacity = '0';
	});
"#;

		// Write CSS file
		let css_path = css_dir.join("tooltip.css");
		if let Err(e) = fs::write(&css_path, css_content) {
				eprintln!("❌ Failed to write CSS file: {}", e);
				std::process::exit(1);
		}
		println!("  ✓ Created: tooltip.css");

		// Write JS file
		let js_path = js_dir.join("tooltip.js");
		if let Err(e) = fs::write(&js_path, js_content) {
				eprintln!("❌ Failed to write JS file: {}", e);
				std::process::exit(1);
	}
		println!("  ✓ Created: tooltip.js");

	// Update book.toml with preprocessor configuration
	println!("⚙️  Updating book.toml...");
	match update_book_toml(&book_toml_path) {
		Ok(_) => println!("✓ Successfully updated book.toml"),
		Err(e) => {
			eprintln!("✗ Failed to update book.toml: {}", e);
			std::process::exit(1);
		}
	}

	println!("\n✅ Installation complete!");
	println!("📋 Next steps:");
	println!("   1. The 'tooltips' preprocessor has been added to book.toml");
	println!("   2. Custom CSS is in: theme/css/");
	println!("   3. Custom JS is in: theme/js/");
	println!("   4. Run 'mdbook serve' or 'mdbook build' to test");
}

fn update_book_toml(book_toml_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
	// Read the current book.toml
	let content = fs::read_to_string(book_toml_path)?;
	let mut table: Table = content.parse()?;

	// Ensure preprocessor section exists
	if !table.contains_key("preprocessor") {
		table.insert("preprocessor".to_string(), Value::Table(Table::new()));
	}

	let preprocessor_table = table
		.get_mut("preprocessor")
		.and_then(|v| v.as_table_mut())
		.ok_or("Failed to get preprocessor table")?;

	// Add tooltips preprocessor configuration
	let mut tooltips_config = Table::new();
	tooltips_config.insert("command".to_string(), Value::String("mdbook-tooltips".to_string()));
    
	preprocessor_table.insert("tooltips".to_string(), Value::Table(tooltips_config));

	// Ensure output.html section exists and add additional-js / additional-css
	if !table.contains_key("output") {
		table.insert("output".to_string(), Value::Table(Table::new()));
	}

	let output_table = table
		.get_mut("output")
		.and_then(|v| v.as_table_mut())
		.ok_or("Failed to get output table")?;

	if !output_table.contains_key("html") {
		output_table.insert("html".to_string(), Value::Table(Table::new()));
	}

	let html_table = output_table
		.get_mut("html")
		.and_then(|v| v.as_table_mut())
		.ok_or("Failed to get output.html table")?;

	let js_path = "./theme/js/tooltip.js".to_string();
	let css_path = "./theme/css/tooltip.css".to_string();

	// helper to ensure an array contains a value
	let ensure_array_contains = |tbl: &mut Table, key: &str, val: String| {
		match tbl.get_mut(key) {
			Some(existing) => {
				if let Some(arr) = existing.as_array_mut() {
					// check whether value exists
					let exists = arr.iter().any(|v| v.as_str() == Some(&val));
					if !exists {
						arr.push(Value::String(val));
					}
				} else {
					// replace non-array value with array containing previous and new
					let prev = existing.as_str().map(|s| s.to_string());
					let mut new_arr = vec![];
					if let Some(p) = prev {
						new_arr.push(Value::String(p));
					}
					new_arr.push(Value::String(val));
					tbl.insert(key.to_string(), Value::Array(new_arr));
				}
			}
			None => {
				tbl.insert(key.to_string(), Value::Array(vec![Value::String(val)]));
			}
		}
	};

	ensure_array_contains(html_table, "additional-js", js_path);
	ensure_array_contains(html_table, "additional-css", css_path);

	// Write back the updated configuration
	let updated_content = toml::to_string_pretty(&table)?;
	fs::write(book_toml_path, updated_content)?;

	Ok(())
}
