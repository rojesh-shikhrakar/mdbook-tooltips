Feature: Parse and replace markdown-style tooltip links with HTML tooltip tags
  As a content parser
  I want occurrences of [KeyWord](~Tooltip Text) to be replaced with
  <span class="tooltip-word">KeyWord<span class="tooltip-text">Tooltip</span></span>
  So that rendered HTML shows the keyword and an attached tooltip element.

  Background:
    Given a parser configured to transform "[Label](~Tooltip Text)" into "<span class=\"tooltip-word\">Label<span class=\"tooltip-text\">Tooltip Text</span></span>"

  Scenario: Replace a single occurrence
    Given the input text "This is a [Term](~A helpful hint) in a sentence."
    When the parser processes the text
    Then the output should be "This is a <span class=\"tooltip-word\">Term<span class=\"tooltip-text\">A helpful hint</span></span> in a sentence."

  Scenario: Replace multiple occurrences in the same line
    Given the input text "Compare [One](~First) and [Two](~Second)."
    When the parser processes the text
    Then the output should be "Compare <span class=\"tooltip-word\">One<span class=\"tooltip-text\">First</span></span> and <span class=\"tooltip-word\">Two<span class=\"tooltip-text\">Second</span></span>."

  Scenario: Replace occurrences separated by punctuation
    Given the input text "List: [A](~a), [B](~b); and [C](~c)."
    When the parser processes the text
    Then the output should be "List: <span class=\"tooltip-word\">A<span class=\"tooltip-text\">a</span></span>, <span class=\"tooltip-word\">B<span class=\"tooltip-text\">b</span></span>; and <span class=\"tooltip-word\">C<span class=\"tooltip-text\">c</span></span>."

  Scenario: Preserve other brackets and parentheses that don't match pattern
    Given the input text "Function call foo(bar) and [NotTooltip](no-tilde) remain unchanged."
    When the parser processes the text
    Then the output should be "Function call foo(bar) and [NotTooltip](no-tilde) remain unchanged."

  Scenario: Ignore malformed tooltip patterns
    Given the input text "Broken [X](~) and [Y](~    ) should be treated as plain text."
    When the parser processes the text
    Then the output should be "Broken [X](~) and [Y](~    ) should be treated as plain text."

  Scenario: Trim tooltip whitespace in replacement
    Given the input text "See [Trimmed](~   leading and trailing   )."
    When the parser processes the text
    Then the output should be "See <span class=\"tooltip-word\">Trimmed<span class=\"tooltip-text\">leading and trailing</span></span>."

  Scenario: Handle tooltip text containing special characters
    Given the input text "Note [Special](~Contains <>&\"' characters)."
    When the parser processes the text
    Then the output should be "Note <span class=\"tooltip-word\">Special<span class=\"tooltip-text\">Contains <>&\"' characters</span></span>."

  Scenario: Do not replace when label contains spaces
    Given the input text "Bad [Two Words](~too word tooltip) should not match."
    When the parser processes the text
    Then the output should be "Bad [Two Words](~tooltip) should not match."

  Scenario Outline: Multiple example inputs and expected outputs
    Given the input text "<input>"
    When the parser processes the text
    Then the output should be "<expected>"

    Examples:
      | input                                           | expected                                                                 |
      | "A [X](~t)."                                    | "A <span class=\"tooltip-word\">X<span class=\"tooltip-text\">t</span></span>."                                        |
      | "[Start](~s) middle [End](~e)"                  | "<span class=\"tooltip-word\">Start<span class=\"tooltip-text\">s</span></span> middle <span class=\"tooltip-word\">End<span class=\"tooltip-text\">e</span></span>" |
      | "No match here"                                 | "No match here"                                                          |
