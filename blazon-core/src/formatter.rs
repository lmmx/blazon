/// Format Rust source code with canonical style.
pub fn format_source(source: &str) -> String {
    let mut output = String::with_capacity(source.len());
    output
}

/// Write indentation to buffer
pub(crate) fn write_indent(buf: &mut String, indent: usize) {
    for _ in 0..indent {
        buf.push(' ');
    }
}
