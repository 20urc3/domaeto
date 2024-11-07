use std::ops::Range;
use std::fs;
use std::io;

mod grammar;

// Equivalent to _INT_RANGES
pub const INT_RANGES: &[(&str, Range<i64>)] = &[
    ("int", -2147483648..2147483647),
    ("int32", -2147483648..2147483647),
    ("uint32", 0..4294967295),
    ("int8", -128..127),
    ("uint8", 0..255),
    ("int16", -32768..32767),
    ("uint16", 0..65535),
    ("int64", i64::MIN..i64::MAX),
    ("uint64", 0..u64::MAX as i64),
];

// Equivalent to _INT_FORMATS
pub const INT_FORMATS: &[(&str, &str)] = &[
    ("int", "i"),
    ("int32", "i"),
    ("uint32", "I"),
    ("int8", "b"),
    ("uint8", "B"),
    ("int16", "h"),
    ("uint16", "H"),
    ("int64", "q"),
    ("uint64", "Q"),
];

// Equivalent to _NONINTERESTING_TYPES
pub const NONINTERESTING_TYPES: &[&str] = &[
    "short",
    "long",
    "DOMString",
    "boolean",
    "float",
    "double",
];

fn import_grammar(filename: &str);

fn parse_from_file(filename: &str) ->  String {
    let file_contents = fs::read_to_string(filename)?;
    file_contents
}

/*
-    __init__: Initializes the Grammar object.
-    parse_from_string: Parses grammar rules from a string.
-    parse_from_file: Parses grammar from a file. = DONE
-    _include_from_string: Includes and parses grammar rules from a string.
-    _include_from_file: Includes and parses grammar rules from a file.
-    _parse_grammar_line: Parses a grammar rule line.
-    _parse_code_line: Parses a code line for generating code.
-    _remove_comments: Removes comments and trims the line.
-    _compute_interesting_indices: Computes interesting indices for variable types.
-    _generate: Generates a user-defined symbol.
-    _expand_rule: Expands a given rule.
-    _select_creator: Selects the creator for the given symbol based on probabilities.
-    _generate_root: Expands the root symbol.
-    _generate_symbol: Expands a symbol whose name is given as an argument.
-    _generate_int: Generates integer types.
-    _generate_float: Generates floating point types.
-    _generate_char: Generates a single character.
-    _generate_string: Generates a random string.
-    _generate_html_string: Generates an HTML-safe string.
-    _generate_hex: Generates a single hex digit.
-    _generate_import: Expands a symbol from another (imported) grammar.
-    _generate_lines: Generates a given number of lines of code.
-    _generate_code: Generates a given number of lines of code.
-    _add_variable: Adds a variable to the context.
-    _get_variable_setters: Gets the variable setters for a given variable.
-    _get_any_var: Gets any variable from the context.
-    _string_to_int: Converts a string to an integer.
-    _exec_function: Executes user-defined Python code.
-    _fix_idents: Fixes indentation in user-defined functions.
-    _save_function: Saves a user-defined function.
-    _set_variable_format: Sets the variable format for programming language generation.
-    _set_line_guard: Sets a guard block for programming language generation.
-    _set_recursion_depth: Sets the maximum recursion depth.
-    _set_var_reuse_probability: Sets the variable reuse probability.
-    _set_extends: Sets the inheritance for a type.
-    _command_handlers: Handles special commands in the grammar.
    *    varformat
    *    include
    *    import
    *    lineguard
    *    max_recursion
    *    var_reuse_prob
    *    extends
-    _get_cdf: Computes a probability function for a given creator array.
-    _normalize_probabilities: Normalizes probabilities for production rules.
-    _parse_tag_and_attributes: Extracts tag name and attributes from a string.
-    _add_variable: Adds a variable to the context, including inheritance.
*/