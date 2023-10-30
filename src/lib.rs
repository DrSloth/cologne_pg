use pgrx::prelude::*;

pgrx::pg_module_magic!();

#[pg_extern(sql = "CREATE FUNCTION cologne(VARIADIC args text[]) RETURNS TEXT STRICT LANGUAGE c AS 'MODULE_PATHNAME', 'cologne_wrapper';")]
fn cologne(texts: pgrx::Array<&str>) -> String {
    let mut outbuf = String::new();
    for text in texts.into_iter() {
        if let Some(s) = text {
            cologne_phonetics::utf8_to_cologne_phonetics_string(s.as_bytes(), &mut outbuf);
        }
        outbuf.push(' ');
    }
    outbuf.pop();
    outbuf
}

// #[cfg(any(test, feature = "pg_test"))]
// #[pg_schema]
// mod tests {
//     use pgrx::prelude::*;
// }

/// This module is required by `cargo pgrx test` invocations.
/// It must be visible at the root of your extension crate.
#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {
        // perform one-off initialization when the pg_test framework starts
    }

    pub fn postgresql_conf_options() -> Vec<&'static str> {
        // return any postgresql.conf settings that are required for your tests
        vec![]
    }
}
