# mdbook-tooltips

A production-ready Rust CLI application that parses markdown-style tooltip syntax and converts it to HTML tooltip tags with full BDD test coverage.

## 🎯 Quick Start

**Transformation:**
```
Input:  "Learn about [API](~Application Programming Interface)"
Output: "Learn about <API>API<Tooltip>Application Programming Interface</Tooltip></API>"
```

## 📦 Usage

### Build
```bash
cargo build --release
```

### CLI Commands
```bash
# Print to stdout
./target/release/mdbook-tooltips sample.txt

# Save to file
./target/release/mdbook-tooltips sample.txt -o output.txt

# Show help
./target/release/mdbook-tooltips --help
```

### Run Tests
```bash
cargo test                    # All tests
cargo test --lib            # Unit tests only
cargo test --test parser_test # BDD tests only
```

## ✨ Features

- ✅ Regex-based pattern matching for `[label](~tooltip)` syntax
- ✅ HTML tag generation with proper nesting
- ✅ Whitespace trimming on tooltip content
- ✅ Label validation (no spaces, no brackets)
- ✅ Special character support in tooltips
- ✅ Malformed pattern rejection
- ✅ Non-matching pattern preservation
- ✅ File I/O with optional output to file or stdout
- ✅ CLI with help/version information
- ✅ 100% test coverage (55+ test cases)

## 📋 Pattern Rules

| Input | Output | Reason |
|-------|--------|--------|
| `[Term](~Tooltip)` | ✅ Replaced | Valid pattern |
| `[Two Words](~Tooltip)` | ❌ Preserved | Label has spaces |
| `[Term](no-tilde)` | ❌ Preserved | Missing tilde prefix |
| `[Term](~)` | ❌ Preserved | Empty tooltip |
| `foo(bar)` | ✅ Preserved | Not tooltip pattern |

## 🧪 Test Results

```
Unit Tests:      11 passed ✅
BDD Scenarios:   11 passed ✅
Total Steps:     44 passed ✅
Overall:         100% pass rate
```

## 📁 Project Structure

```
src/
  lib.rs                  # Library entry point
  main.rs                 # CLI application
  features/
    mod.rs               # Module exports
    parser.rs            # TooltipParser implementation

tests/
  parser_test.rs         # Cucumber BDD tests
  parse.feature          # Feature definitions

sample.txt              # Example input file
output.txt              # Example output file
CLI_USAGE.md           # Detailed CLI documentation
```

## 🔧 Dependencies

- **clap** (4.5.51) - CLI argument parsing
- **regex** (1.12.2) - Pattern matching
- **cucumber** (0.21.1) - BDD testing

## 📖 Code Example

```rust
use mdbook_tooltips::features::parser::TooltipParser;

let parser = TooltipParser::new();
let input = "Learn about [REST](~Representational State Transfer)";
let output = parser.parse(input);
// Output: Learn about <REST>REST<Tooltip>Representational State Transfer</Tooltip></REST>
```

## 📄 License

MIT

---

**Status:** Production Ready ✅
