/*!
# toml-frontmatter

> **TOML frontmatter parser.**

See the [`parse`] documentation for an example.

## License

Distributed under the [Apache License 2.0](https://spdx.org/licenses/Apache-2.0.html) and [MIT](https://spdx.org/licenses/MIT.html) licenses, see [LICENSE-Apache](https://git.bauke.xyz/Holllo/toml-frontmatter/src/branch/main/LICENSE-Apache) and [LICENSE-MIT](https://git.bauke.xyz/Holllo/toml-frontmatter/src/branch/main/LICENSE-MIT) for more information.
*/

#![forbid(unsafe_code)]
#![warn(missing_docs, clippy::missing_docs_in_private_items)]

use {
  anyhow::{anyhow, Result},
  serde::Deserialize,
};

/**
Parse a struct that implements [`serde::Deserialize`] from frontmatter and
return the remaining contents of the string.

## Errors

This function will return an [`Err`] when:

- the data doesn't have a frontmatter section,
- the frontmatter isn't at the beginning of the data,
- the TOML fails to parse.

## Example

```rust
#[derive(serde::Deserialize)]
struct Frontmatter {
  date: String,
}

let sample = r#"
---toml
date = "2023-01-01"
---

Some **Markdown**. Or something else!
"#.trim();

let (frontmatter, markdown) = toml_frontmatter::parse::<Frontmatter>(sample).unwrap();
```
*/
pub fn parse<'a, D: Deserialize<'a>>(data: &'a str) -> Result<(D, &'a str)> {
  let start_marker = "---toml\n";
  let end_marker = "\n---\n";

  let (start, end) = match (data.find(start_marker), data.find(end_marker)) {
    (Some(start), Some(end)) => (start, end),
    _ => return Err(anyhow!("Missing frontmatter")),
  };

  if start != 0 {
    return Err(anyhow!("Frontmatter not at beginning of data"));
  }

  let start = start + start_marker.len();
  let frontmatter = &data[start..end];

  let end = end + end_marker.len();
  let extra = &data[end..];

  Ok((toml::from_str::<D>(frontmatter)?, extra.trim_start()))
}
