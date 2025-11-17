use std::fmt::Write;

#[must_use]
#[cfg_attr(feature = "hotpath", hotpath::measure)]
pub fn get_desktop_info() -> String {
  // Retrieve the environment variables and handle Result types
  let desktop_env = std::env::var("XDG_CURRENT_DESKTOP");
  let display_backend = std::env::var("XDG_SESSION_TYPE");

  let desktop_str = match desktop_env {
    Err(_) => "Unknown",
    Ok(ref s) if s.starts_with("none+") => &s[5..],
    Ok(ref s) => s.as_str(),
  };

  let backend_str = match display_backend {
    Err(_) => "Unknown",
    Ok(ref s) if s.is_empty() => "Unknown",
    Ok(ref s) => s.as_str(),
  };

  // Pre-calculate capacity: desktop_len + " (" + backend_len + ")"
  // Capitalize first char needs temporary allocation only if backend exists
  let mut result =
    String::with_capacity(desktop_str.len() + backend_str.len() + 3);
  result.push_str(desktop_str);
  result.push_str(" (");

  // Capitalize first character of backend
  if let Some(first_char) = backend_str.chars().next() {
    let _ = write!(result, "{}", first_char.to_ascii_uppercase());
    result.push_str(&backend_str[first_char.len_utf8()..]);
  }

  result.push(')');
  result
}
