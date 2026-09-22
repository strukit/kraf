use crate::transport::TransportParams;

#[test]
fn get_returns_value_for_known_key() {
    let params = TransportParams::from(vec![("path".to_string(), "/tmp/a".to_string())]);

    assert_eq!(params.get("path"), Some(&"/tmp/a".to_string()));
}

#[test]
fn get_returns_none_for_unknown_key() {
    let params = TransportParams::from(vec![("path".to_string(), "/tmp/a".to_string())]);

    assert_eq!(params.get("missing"), None);
}

#[test]
fn set_appends_a_new_entry() {
    let mut params: TransportParams<Vec<(String, String)>> = TransportParams::from(vec![]);

    params.set("path", "/tmp/a");

    assert_eq!(params.get("path"), Some(&"/tmp/a".to_string()));
}

#[test]
fn has_returns_true_for_known_key() {
    let params = TransportParams::from(vec![("path".to_string(), "/tmp/a".to_string())]);

    assert!(params.has("path"));
}

#[test]
fn has_returns_false_for_unknown_key() {
    let params = TransportParams::from(vec![("path".to_string(), "/tmp/a".to_string())]);

    assert!(!params.has("missing"));
}
