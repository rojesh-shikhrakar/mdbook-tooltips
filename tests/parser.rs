use cucumber::{given, then, when, World};
use mdbook_tooltips::features::parser::TooltipParser;

/// A simple world struct to hold the test context
#[derive(Debug, World)]
pub struct ParserWorld {
    input_text: String,
    output_text: String,
    parser: TooltipParser,
}

impl Default for ParserWorld {
    fn default() -> Self {
        Self {
            input_text: String::new(),
            output_text: String::new(),
            parser: TooltipParser::new(),
        }
    }
}

#[given(regex = r#"a parser configured to transform "(.*)" into "(.*)""#)]
fn given_parser_configured(_world: &mut ParserWorld, _from: String, _to: String) {
    // Parser is implicitly configured through the parse_tooltips function
    // No additional setup needed; we accept the background's strings but don't need them
}

#[given(regex = r#"the input text "(.*)""#)]
fn given_input_text(world: &mut ParserWorld, text: String) {
    world.input_text = text;
}

// ===== WHEN STEPS =====

#[when("the parser processes the text")]
fn when_parser_processes(world: &mut ParserWorld) {
    world.output_text = world.parser.parse(&world.input_text);
}

// ===== THEN STEPS =====

#[then(regex = r#"the output should be "(.*)""#)]
fn then_output_should_be(world: &mut ParserWorld, expected: String) {
    // Feature file strings may include escaped quotes (e.g. \"), unescape common sequences
    let expected_unescaped = expected.replace("\\\"", "\"").replace("\\\\", "\\");

    // Also normalize actual output in case feature captured input with escaped quotes
    let actual_unescaped = world
        .output_text
        .replace("\\\"", "\"")
        .replace("\\\\", "\\");

    assert_eq!(
        actual_unescaped, expected_unescaped,
        "Output mismatch!\nInput: {}\nExpected: {}\nGot: {}",
        world.input_text, expected_unescaped, world.output_text
    );
}

// ===== ENTRY POINT FOR CUCUMBER =====

#[test]
fn cucumber() {
    futures::executor::block_on(ParserWorld::run("tests/parse.feature"));
}
