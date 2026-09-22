use crate::transport::TransportHeaders;

#[test]
fn get_returns_value_for_known_key() {
    let headers = TransportHeaders::from(vec![("if-range".to_string(), "sha256:abc".to_string())]);

    assert_eq!(headers.get("if-range"), Some(&"sha256:abc".to_string()));
}

#[test]
fn get_returns_none_for_unknown_key() {
    let headers = TransportHeaders::from(vec![("if-range".to_string(), "sha256:abc".to_string())]);

    assert_eq!(headers.get("missing"), None);
}

#[test]
fn set_appends_a_new_entry() {
    let mut headers: TransportHeaders<Vec<(String, String)>> = TransportHeaders::from(vec![]);

    headers.set("if-range", "sha256:abc");

    assert_eq!(headers.get("if-range"), Some(&"sha256:abc".to_string()));
}

#[test]
fn has_returns_true_for_known_key() {
    let headers = TransportHeaders::from(vec![("if-range".to_string(), "sha256:abc".to_string())]);

    assert!(headers.has("if-range"));
}

#[test]
fn has_returns_false_for_unknown_key() {
    let headers = TransportHeaders::from(vec![("if-range".to_string(), "sha256:abc".to_string())]);

    assert!(!headers.has("missing"));
}
