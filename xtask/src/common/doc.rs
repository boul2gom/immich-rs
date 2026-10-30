//! Rustdoc sanitization utilities for `OpenAPI` descriptions and Markdown tables.

/// Returns `true` if `word` has mixed internal `CamelCase` casing (at least 2 uppercase and 1 lowercase ASCII letters)
/// and is not a pluralized acronym such as `URLs`, `IDs`, `APIs`, or `DTOs`.
fn is_camel_case_ident(word: &str) -> bool {
    if let Some(stem) = word.strip_suffix('s')
        && !stem.is_empty()
        && stem.chars().all(|c| c.is_ascii_uppercase() || c.is_ascii_digit())
    {
        return false;
    }
    let upper = word.chars().filter(char::is_ascii_uppercase).count();
    let lower = word.chars().filter(char::is_ascii_lowercase).count();
    upper >= 2 && lower >= 1
}

/// Sanitizes a single line of prose before emitting it inside `#[doc = " ..."]`.
///
/// - Escapes bare `http://` and `https://` URLs with `<...>` unless already wrapped or in Markdown links.
/// - Escapes square-bracket expressions like `[1-5]` or `[ISO 8601]` that are not Markdown links
///   or backticked intra-doc links (`[`Foo`]`), preventing `rustdoc::broken_intra_doc_links`.
/// - Wraps unbackticked `snake_case` and `CamelCase` identifiers in backticks to satisfy `clippy::doc_markdown`.
#[must_use]
pub fn sanitize_doc_line(input: &str) -> String {
    let trimmed = input.trim_end_matches('\r');
    if trimmed.trim().is_empty() {
        return String::new();
    }

    let with_brackets = escape_non_link_brackets(trimmed);
    let with_urls = wrap_bare_urls(&with_brackets);
    backtick_code_terms(&with_urls)
}

/// Sanitizes a string for inclusion inside a Markdown table cell (`| ... |`).
#[must_use]
pub fn sanitize_table_cell(input: &str) -> String {
    let single_line = input
        .replace("\r\n", "<br>")
        .replace('\n', "<br>")
        .replace('\r', "")
        .replace('|', "\\|");
    sanitize_doc_line(single_line.trim())
}

fn count_leading_backticks(s: &str) -> usize {
    s.bytes().take_while(|&b| b == b'`').count()
}

fn escape_non_link_brackets(line: &str) -> String {
    let bytes = line.as_bytes();
    let mut out = String::with_capacity(line.len() + 8);
    let mut i = 0;
    let mut code_delim_len: usize = 0;

    while i < bytes.len() {
        let b = bytes[i];
        if b == b'`' {
            let run = count_leading_backticks(&line[i..]);
            for _ in 0..run {
                out.push('`');
            }
            if code_delim_len == 0 {
                code_delim_len = run;
            } else if run == code_delim_len {
                code_delim_len = 0;
            }
            i += run;
            continue;
        }
        if code_delim_len == 0 && b == b'\\' && i + 1 < bytes.len() {
            out.push('\\');
            out.push(bytes[i + 1] as char);
            i += 2;
            continue;
        }
        if code_delim_len == 0
            && b == b'['
            && let Some(rel_end) = find_closing_bracket(&line[i + 1..])
        {
            let close_idx = i + 1 + rel_end;
            let inner = &line[i + 1..close_idx];
            let after = bytes.get(close_idx + 1).copied();

            let is_markdown_link = matches!(after, Some(b'(' | b'['));
            let is_backtick_intradoc = inner.starts_with('`') && inner.ends_with('`') && inner.len() >= 2;

            if is_markdown_link || is_backtick_intradoc {
                out.push_str(&line[i..=close_idx]);
            } else {
                out.push_str("\\[");
                out.push_str(inner);
                out.push_str("\\]");
            }
            i = close_idx + 1;
            continue;
        }

        let ch = line[i..].chars().next().unwrap();
        out.push(ch);
        i += ch.len_utf8();
    }

    out
}

fn find_closing_bracket(rest: &str) -> Option<usize> {
    let mut code_delim_len: usize = 0;
    let mut idx = 0;
    while idx < rest.len() {
        let s = &rest[idx..];
        if s.starts_with('`') {
            let run = count_leading_backticks(s);
            if code_delim_len == 0 {
                code_delim_len = run;
            } else if run == code_delim_len {
                code_delim_len = 0;
            }
            idx += run;
            continue;
        }
        let ch = s.chars().next().unwrap();
        if code_delim_len == 0 {
            if ch == ']' {
                return Some(idx);
            } else if ch == '[' {
                return None;
            }
        }
        idx += ch.len_utf8();
    }
    None
}

fn wrap_bare_urls(line: &str) -> String {
    let mut out = String::with_capacity(line.len() + 8);
    let mut i = 0;
    let mut code_delim_len: usize = 0;

    while i < line.len() {
        let rest = &line[i..];
        if rest.starts_with('`') {
            let run = count_leading_backticks(rest);
            for _ in 0..run {
                out.push('`');
            }
            if code_delim_len == 0 {
                code_delim_len = run;
            } else if run == code_delim_len {
                code_delim_len = 0;
            }
            i += run;
            continue;
        }

        if code_delim_len == 0 && (rest.starts_with("https://") || rest.starts_with("http://")) {
            let prev = out.chars().next_back();
            let already_wrapped = matches!(prev, Some('<' | '(' | '"' | '\''));
            let end_offset = rest
                .find(|c: char| c.is_whitespace() || matches!(c, '>' | ')' | ']' | '"' | '\'' | ',' | ';'))
                .unwrap_or(rest.len());
            let mut url_part = &rest[..end_offset];
            let mut trailing_dot = false;
            if url_part.ends_with('.') {
                url_part = &url_part[..url_part.len() - 1];
                trailing_dot = true;
            }

            if already_wrapped || url_part.len() <= "https://".len() {
                out.push_str(url_part);
            } else {
                out.push('<');
                out.push_str(url_part);
                out.push('>');
            }
            if trailing_dot {
                out.push('.');
            }
            i += end_offset;
            continue;
        }

        let ch = rest.chars().next().unwrap();
        out.push(ch);
        i += ch.len_utf8();
    }

    out
}

fn backtick_code_terms(line: &str) -> String {
    let mut out = String::with_capacity(line.len() + 8);
    let mut i = 0;
    let mut code_delim_len: usize = 0;

    while i < line.len() {
        let rest = &line[i..];
        if rest.starts_with('`') {
            let run = count_leading_backticks(rest);
            for _ in 0..run {
                out.push('`');
            }
            if code_delim_len == 0 {
                code_delim_len = run;
            } else if run == code_delim_len {
                code_delim_len = 0;
            }
            i += run;
            continue;
        }

        if code_delim_len > 0 {
            let ch = rest.chars().next().unwrap();
            out.push(ch);
            i += ch.len_utf8();
            continue;
        }

        // Skip `<https://...>` / `<br>` HTML or autolink tags untouched
        if rest.starts_with('<')
            && let Some(end) = rest.find('>')
        {
            out.push_str(&rest[..=end]);
            i += end + 1;
            continue;
        }

        // Skip Markdown links `[label](target)` untouched
        if rest.starts_with('[')
            && !rest.starts_with("\\[")
            && let Some(close_bracket) = find_closing_bracket(&rest[1..])
        {
            let after_bracket = 1 + close_bracket + 1;
            if rest[after_bracket..].starts_with('(')
                && let Some(close_paren) = rest[after_bracket..].find(')')
            {
                let total = after_bracket + close_paren + 1;
                out.push_str(&rest[..total]);
                i += total;
                continue;
            }
        }

        let ch = rest.chars().next().unwrap();
        if ch.is_ascii_alphanumeric() {
            let prev = out.chars().next_back();
            let word_boundary_before =
                !prev.is_some_and(|c| c.is_ascii_alphanumeric() || matches!(c, '_' | '/' | ':' | '.'));
            let word_end = rest
                .find(|c: char| !(c.is_ascii_alphanumeric() || c == '_'))
                .unwrap_or(rest.len());
            let word = &rest[..word_end];
            let next = rest[word_end..].chars().next();
            let word_boundary_after = !next.is_some_and(|c| matches!(c, '/' | ':'));

            let needs_ticks = word_boundary_before
                && word_boundary_after
                && ((word.contains('_') && word.trim_matches('_').len() >= 2) || is_camel_case_ident(word));

            if needs_ticks {
                out.push('`');
                out.push_str(word);
                out.push('`');
            } else {
                out.push_str(word);
            }
            i += word_end;
            continue;
        }

        out.push(ch);
        i += ch.len_utf8();
    }

    out
}
