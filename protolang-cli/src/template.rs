//! micro 'template' engine
//!
//! Substitute all instances of {{variable}} by the associated
//! value of 'variable' in a hashmap
use std::collections::HashMap;

pub fn template(template: &str, vars: &HashMap<String, String>) -> Result<String, String> {
    let mut out = String::new();
    let mut remaining = template.split("{{");

    let Some(first) = remaining.next() else {
        return Ok(out);
    };
    out.push_str(first);

    while let Some(chunk) = remaining.next() {
        // if no end pos, then we push the literal {{ and the remaining text
        let end_var = chunk.find("}}");
        let Some(end_pos) = end_var else {
            out.push_str("{{");
            out.push_str(chunk);
            continue;
        };

        let (before, after) = chunk.split_at(end_pos);
        // if the variable is not found, return an error with the name of the missing variable
        let Some(var_value) = vars.get(before) else {
            return Err(before.to_string());
        };
        out.push_str(var_value);
        let after = after.strip_prefix("}}").unwrap();
        out.push_str(after);
    }
    Ok(out)
}
