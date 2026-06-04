use crate::error::{AkError, AkResult};

pub fn replace_first(haystack: &str, from: &str, to: &str) -> String {
    if let Some(idx) = haystack.find(from) {
        let mut out = String::with_capacity(
            haystack
                .len()
                .saturating_add(to.len())
                .saturating_sub(from.len()),
        );
        out.push_str(&haystack[..idx]);
        out.push_str(to);
        out.push_str(&haystack[idx + from.len()..]);
        out
    } else {
        haystack.to_owned()
    }
}

pub fn render_template(template: &str, vars: &[(&str, String)]) -> AkResult<String> {
    let mut result = template.to_owned();
    for (key, value) in vars {
        let placeholder = format!("{{{{{key}}}}}");
        if !result.contains(&placeholder) {
            continue;
        }
        result = result.replace(&placeholder, value);
    }
    if result.contains("{{") && result.contains("}}") {
        return Err(AkError::Validation(
            "template contains unresolved placeholders",
        ));
    }
    Ok(result)
}

#[cfg(feature = "text")]
pub fn regex_match(value: &str, pattern: &str) -> AkResult<bool> {
    let re = regex::Regex::new(pattern).map_err(|err| AkError::Parse(err.to_string()))?;
    Ok(re.is_match(value))
}

#[cfg(feature = "text")]
pub fn regex_find(value: &str, pattern: &str) -> AkResult<Option<String>> {
    let re = regex::Regex::new(pattern).map_err(|err| AkError::Parse(err.to_string()))?;
    Ok(re.find(value).map(|m| m.as_str().to_owned()))
}

#[cfg(feature = "text")]
pub fn regex_replace(value: &str, pattern: &str, replacement: &str) -> AkResult<String> {
    let re = regex::Regex::new(pattern).map_err(|err| AkError::Parse(err.to_string()))?;
    Ok(re.replace_all(value, replacement).into_owned())
}

#[cfg(feature = "text")]
pub fn regex_split(value: &str, pattern: &str) -> AkResult<Vec<String>> {
    let re = regex::Regex::new(pattern).map_err(|err| AkError::Parse(err.to_string()))?;
    Ok(re.split(value).map(str::to_owned).collect())
}
