use fancy_regex::Regex;
use nu_ansi_term::{
    Color::{Red, White},
    Style,
};
use nu_protocol::{ShellError, Span, Value};

pub fn highlight_search_in_table(
    table: Vec<Value>, // list of records
    search_string: &str,
    searched_cols: &[&str],
    string_style: &Style,
) -> Result<Vec<Value>, ShellError> {
    let orig_search_string = search_string;
    let search_string = search_string.to_lowercase();
    let mut matches = vec![];

    for record in table {
        let (cols, mut vals, record_span) = if let Value::Record { cols, vals, span } = record {
            (cols, vals, span)
        } else {
            return Err(ShellError::NushellFailedSpanned {
                msg: "Expected record".to_string(),
                label: format!("got {}", record.get_type()),
                span: record.span()?,
            });
        };

        let has_match = cols.iter().zip(vals.iter_mut()).fold(
            Ok(false),
            |acc: Result<bool, ShellError>, (col, val)| {
                if searched_cols.contains(&col.as_str()) {
                    if let Value::String { val: s, span } = val {
                        if s.to_lowercase().contains(&search_string) {
                            *val = Value::String {
                                val: highlight_search_string(s, orig_search_string, string_style)?,
                                span: *span,
                            };
                            Ok(true)
                        } else {
                            // column does not contain the searched string
                            acc
                        }
                    } else {
                        // ignore non-string values
                        acc
                    }
                } else {
                    // don't search this column
                    acc
                }
            },
        )?;

        if has_match {
            matches.push(Value::Record {
                cols,
                vals,
                span: record_span,
            });
        }
    }

    Ok(matches)
}

// Highlight the search string using ANSI escape sequences and regular expressions.
pub fn highlight_search_string(
    haystack: &str,
    needle: &str,
    string_style: &Style,
) -> Result<String, ShellError> {
    let regex_string = format!("(?i){needle}");
    let regex = match Regex::new(&regex_string) {
        Ok(regex) => regex,
        Err(err) => {
            return Err(ShellError::GenericError(
                "Could not compile regex".into(),
                err.to_string(),
                Some(Span::test_data()),
                None,
                Vec::new(),
            ));
        }
    };
    // strip haystack to remove existing ansi style
    let stripped_haystack = nu_utils::strip_ansi_likely(haystack);
    let mut last_match_end = 0;
    let style = Style::new().fg(White).on(Red);
    let mut highlighted = String::new();

    for cap in regex.captures_iter(stripped_haystack.as_ref()) {
        match cap {
            Ok(capture) => {
                let start = match capture.get(0) {
                    Some(acap) => acap.start(),
                    None => 0,
                };
                let end = match capture.get(0) {
                    Some(acap) => acap.end(),
                    None => 0,
                };
                highlighted.push_str(
                    &string_style
                        .paint(&stripped_haystack[last_match_end..start])
                        .to_string(),
                );
                highlighted.push_str(&style.paint(&stripped_haystack[start..end]).to_string());
                last_match_end = end;
            }
            Err(e) => {
                return Err(ShellError::GenericError(
                    "Error with regular expression capture".into(),
                    e.to_string(),
                    None,
                    None,
                    Vec::new(),
                ));
            }
        }
    }

    highlighted.push_str(
        &string_style
            .paint(&stripped_haystack[last_match_end..])
            .to_string(),
    );
    Ok(highlighted)
}
