pub mod assign;
pub mod ast_utils;
pub mod fmt;
pub mod fs_tree;
pub mod naming;
pub mod spec;
pub mod tables;

/// Convert a CamelCase string to `snake_case`.
#[must_use]
pub fn camel_to_snake(camel: &str) -> String {
    let mut snake = String::new();
    for (i, c) in camel.chars().enumerate() {
        if c.is_uppercase() {
            if i > 0 {
                snake.push('_');
            }
            snake.push(c.to_ascii_lowercase());
        } else if c == '-' {
            snake.push('_');
        } else {
            snake.push(c);
        }
    }
    snake
}

/// Convert a `snake_case` string to CamelCase.
#[must_use]
pub fn snake_to_camel(snake: &str) -> String {
    let mut camel = String::new();
    let mut next_cap = true;
    for c in snake.chars() {
        if c == '_' {
            next_cap = true;
        } else if next_cap {
            camel.push(c.to_ascii_uppercase());
            next_cap = false;
        } else {
            camel.push(c);
        }
    }
    camel
}
