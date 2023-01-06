# toml-frontmatter

> **TOML frontmatter parser.**

## API

For documentation see [docs.rs](https://docs.rs/toml-frontmatter).

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

## License

Distributed under the [Apache License 2.0](https://spdx.org/licenses/Apache-2.0.html) and [MIT](https://spdx.org/licenses/MIT.html) licenses, see [LICENSE-Apache](https://git.bauke.xyz/Holllo/toml-frontmatter/src/branch/main/LICENSE-Apache) and [LICENSE-MIT](https://git.bauke.xyz/Holllo/toml-frontmatter/src/branch/main/LICENSE-MIT) for more information.
