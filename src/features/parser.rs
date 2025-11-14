use regex::Regex;
use once_cell::sync::Lazy;

/// A parser that transforms markdown-style tooltip links into HTML tooltip tags.
///
/// Pattern: [Label](~Tooltip Text) -> <span class="tooltip-word">Label<span class="tooltip-text">Tooltip Text</span></span>
#[derive(Debug, Clone, Copy)]
pub struct TooltipParser;

// Compile the regex once for the whole program to avoid repeated compilation costs.
static TOOLTIP_RE: Lazy<Regex> = Lazy::new(|| {
	Regex::new(r"\[(.*?)\]\(~(.*?)\)").expect("Failed to compile tooltip regex")
});

impl TooltipParser {
	/// Create a new parser. The underlying regex is static and shared, so this is cheap.
	pub fn new() -> Self {
		TooltipParser
	}

	/// Process the input text and replace tooltip patterns with HTML tags.
	pub fn parse(&self, input: &str) -> String {
		TOOLTIP_RE
			.replace_all(input, |caps: &regex::Captures| {
				let label = caps.get(1).map(|m| m.as_str()).unwrap_or("");
				let tooltip_text = caps.get(2).map(|m| m.as_str().trim()).unwrap_or("");

				// If tooltip text is empty after trimming, preserve original text
				if tooltip_text.is_empty() {
					return format!("[{}](~{})", label, caps.get(2).map(|m| m.as_str()).unwrap_or(""));
				}

				format!(
					r#"<span class="tooltip-word">{}<span class="tooltip-text">{}</span></span>"#,
					label,
					tooltip_text
				)
			})
			.to_string()
	}
}

impl Default for TooltipParser {
	fn default() -> Self {
		Self::new()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_replace_single_occurrence() {
		let parser = TooltipParser::new();
		let input = "This is a [Term](~A helpful hint) in a sentence.";
		let expected = "This is a <span class=\"tooltip-word\">Term<span class=\"tooltip-text\">A helpful hint</span></span> in a sentence.";
		assert_eq!(parser.parse(input), expected);
	}

	#[test]
	fn test_replace_multiple_occurrences_in_same_line() {
		let parser = TooltipParser::new();
		let input = "Compare [One](~First) and [Two](~Second).";
		let expected = "Compare <span class=\"tooltip-word\">One<span class=\"tooltip-text\">First</span></span> and <span class=\"tooltip-word\">Two<span class=\"tooltip-text\">Second</span></span>.";
		assert_eq!(parser.parse(input), expected);
	}

	#[test]
	fn test_ignore_malformed_tooltip_patterns() {
		let parser = TooltipParser::new();
		let input = "Broken [X](~) and [Y](~    ) should be treated as plain text.";
		let expected = "Broken [X](~) and [Y](~    ) should be treated as plain text.";
		assert_eq!(parser.parse(input), expected);
	}
}

