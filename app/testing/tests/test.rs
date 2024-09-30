mod helpers;

#[test]
fn test_mod_helper() {
    assert_eq!(helpers::common_setup(), "Integration tests".to_string());
}
