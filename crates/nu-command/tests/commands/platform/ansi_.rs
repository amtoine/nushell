use nu_test_support::{nu, pipeline};

#[test]
fn test_ansi_shows_error_on_escape() {
    let actual = nu!(pipeline(
        r#"
            ansi -e \
        "#
    ));

    assert!(actual.err.contains("no need for escape characters"))
}

#[test]
fn test_ansi_list_outputs_table() {
    let actual = nu!(pipeline(
        r#"
            ansi --list | length
        "#
    ));

    assert_eq!(actual.out, "424");
}
