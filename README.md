# minigrep - Rust grep parser tool

Based on the rust official tutorial from
[here](https://doc.rust-lang.org/book/ch12-00-an-io-project.html)

This version has additional enhancements:

- It highlights the query in the found lines.

- It accepts an extra parameter to control whether we ignore casing or not.
  Usage:

  `cargo run <search-query> <file> I`

  Example:

  `cargo run to poem.txt I`

  If the casing parameter is not present, `minigrep` will search for an
  environment variable called `CASE_INSENSITIVE`, the default behavior is to
  keep the casing on search.
