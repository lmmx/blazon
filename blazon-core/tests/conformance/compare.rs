use crate::helpers::load_fixture;
use blazon_core::format_source;
use rust_format::{Formatter, RustFmt};

#[test]
fn compare_with_rustfmt_app_state() {
    let code = load_fixture("asterism/app_state");

    let blazon_output = format_source(&code);
    let rustfmt_output = RustFmt::default()
        .format_str(&code)
        .expect("rustfmt failed");

    insta::assert_snapshot!(
        "blazon_vs_rustfmt_diff",
        format!(
            "=== BLAZON ===\n{}\n\n=== RUSTFMT ===\n{}",
            blazon_output, rustfmt_output
        )
    );
}
