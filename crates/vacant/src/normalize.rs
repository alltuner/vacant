// ABOUTME: Canonicalize user input into a registrable-shaped, ASCII, lowercased name.
// ABOUTME: Tolerates pasted URLs, trailing slashes, FQDN dots, mixed case, and IDN labels.

/// Canonicalize a user-supplied string into the engine's domain form.
///
/// Returns `None` if the input has no recoverable host (empty, or a non-ASCII
/// label that fails IDNA encoding). The returned name is lowercase ASCII with
/// no scheme, userinfo, port, path, query, fragment, or trailing dot.
pub fn normalize_input(raw: &str) -> Option<String> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return None;
    }

    let after_scheme = match trimmed.find("://") {
        Some(i) => &trimmed[i + 3..],
        None => trimmed,
    };

    let after_userinfo = match after_scheme.rfind('@') {
        Some(i) => &after_scheme[i + 1..],
        None => after_scheme,
    };

    let host_with_port = after_userinfo
        .split(['/', '?', '#'])
        .next()
        .unwrap_or(after_userinfo);

    let host = strip_port(host_with_port);

    let host = host.trim_end_matches('.').trim();
    if host.is_empty() {
        return None;
    }

    if host.is_ascii() {
        return Some(host.to_ascii_lowercase());
    }

    idna::domain_to_ascii(host).ok()
}

/// Trim a trailing `:<port>` from a host. Leaves bracketed IPv6 literals alone.
fn strip_port(host: &str) -> &str {
    if host.starts_with('[') {
        return host;
    }
    match host.rfind(':') {
        Some(i) if host[i + 1..].chars().all(|c| c.is_ascii_digit()) => &host[..i],
        _ => host,
    }
}

#[cfg(test)]
mod tests {
    use super::normalize_input;

    #[test]
    fn lowercases_ascii() {
        assert_eq!(
            normalize_input("ALLTUNER.COM").as_deref(),
            Some("alltuner.com")
        );
    }

    #[test]
    fn trims_outer_whitespace() {
        assert_eq!(
            normalize_input("  alltuner.com  ").as_deref(),
            Some("alltuner.com")
        );
    }

    #[test]
    fn strips_trailing_dot() {
        assert_eq!(
            normalize_input("alltuner.com.").as_deref(),
            Some("alltuner.com")
        );
    }

    #[test]
    fn strips_url_scheme() {
        assert_eq!(
            normalize_input("https://alltuner.com").as_deref(),
            Some("alltuner.com")
        );
        assert_eq!(
            normalize_input("http://Alltuner.COM").as_deref(),
            Some("alltuner.com")
        );
        assert_eq!(
            normalize_input("ftp://alltuner.com").as_deref(),
            Some("alltuner.com")
        );
    }

    #[test]
    fn strips_path_query_fragment() {
        assert_eq!(
            normalize_input("https://alltuner.com/some/path").as_deref(),
            Some("alltuner.com")
        );
        assert_eq!(
            normalize_input("alltuner.com/path?x=1#frag").as_deref(),
            Some("alltuner.com")
        );
        assert_eq!(
            normalize_input("alltuner.com/").as_deref(),
            Some("alltuner.com")
        );
    }

    #[test]
    fn strips_userinfo() {
        assert_eq!(
            normalize_input("https://user:pass@alltuner.com/path").as_deref(),
            Some("alltuner.com")
        );
        assert_eq!(
            normalize_input("user@alltuner.com").as_deref(),
            Some("alltuner.com")
        );
    }

    #[test]
    fn strips_port() {
        assert_eq!(
            normalize_input("alltuner.com:8080").as_deref(),
            Some("alltuner.com")
        );
        assert_eq!(
            normalize_input("https://alltuner.com:443/path").as_deref(),
            Some("alltuner.com")
        );
    }

    #[test]
    fn idn_to_punycode() {
        assert_eq!(
            normalize_input("café.com").as_deref(),
            Some("xn--caf-dma.com")
        );
        assert_eq!(
            normalize_input("xn--caf-dma.com").as_deref(),
            Some("xn--caf-dma.com")
        );
    }

    #[test]
    fn empty_returns_none() {
        assert!(normalize_input("").is_none());
        assert!(normalize_input("   ").is_none());
        assert!(normalize_input("https://").is_none());
        assert!(normalize_input(".").is_none());
    }

    #[test]
    fn combined_messy_input() {
        assert_eq!(
            normalize_input("  HTTPS://User@CAFÉ.com:443/x?y=1  ").as_deref(),
            Some("xn--caf-dma.com")
        );
    }
}
