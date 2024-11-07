# Grammar.rs 

## Constructor and Initialization

-    __init__: Initializes the Grammar object.

# Grammar Parsing and Generation

-    parse_from_string: Parses grammar rules from a string.
-    parse_from_file: Parses grammar from a file.
-    _include_from_string: Includes and parses grammar rules from a string.
-    _include_from_file: Includes and parses grammar rules from a file.
-    _parse_grammar_line: Parses a grammar rule line.
-    _parse_code_line: Parses a code line for generating code.
-    _remove_comments: Removes comments and trims the line.
-    _compute_interesting_indices: Computes interesting indices for variable types.

## Symbol and Rule Expansion

-    _generate: Generates a user-defined symbol.
-    _expand_rule: Expands a given rule.
-    _select_creator: Selects the creator for the given symbol based on probabilities.
-    _generate_root: Expands the root symbol.
-    _generate_symbol: Expands a symbol whose name is given as an argument.

## Type Generation

-    _generate_int: Generates integer types.
-    _generate_float: Generates floating point types.
-    _generate_char: Generates a single character.
-    _generate_string: Generates a random string.
-    _generate_html_string: Generates an HTML-safe string.
-    _generate_hex: Generates a single hex digit.
-    _generate_import: Expands a symbol from another (imported) grammar.
-    _generate_lines: Generates a given number of lines of code.
-    _generate_code: Generates a given number of lines of code.

## Context and Variable Management

-    _add_variable: Adds a variable to the context.
-    _get_variable_setters: Gets the variable setters for a given variable.
-    _get_any_var: Gets any variable from the context.

## Helper Functions

-    _string_to_int: Converts a string to an integer.
-    _exec_function: Executes user-defined Python code.
-    _fix_idents: Fixes indentation in user-defined functions.
-    _save_function: Saves a user-defined function.
-    _set_variable_format: Sets the variable format for programming language generation.
-    _set_line_guard: Sets a guard block for programming language generation.
-    _set_recursion_depth: Sets the maximum recursion depth.
-    _set_var_reuse_probability: Sets the variable reuse probability.
-    _set_extends: Sets the inheritance for a type.

## Command Handling

-    _command_handlers: Handles special commands in the grammar.
    *    varformat
    *    include
    *    import
    *    lineguard
    *    max_recursion
    *    var_reuse_prob
    *    extends

## Probability and CDF Management

-    _get_cdf: Computes a probability function for a given creator array.
-    _normalize_probabilities: Normalizes probabilities for production rules.

## Other

-    _parse_tag_and_attributes: Extracts tag name and attributes from a string.
-    _add_variable: Adds a variable to the context, including inheritance.
