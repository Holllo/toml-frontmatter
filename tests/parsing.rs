use {
  anyhow::Result,
  insta::{assert_snapshot, assert_toml_snapshot},
  serde::{Deserialize, Serialize},
};

#[derive(Debug, Deserialize, Serialize)]
struct Frontmatter {
  date: String,
}

#[test]
fn test_parsing() -> Result<()> {
  let sample = r#"
---toml
date = "2023-01-01"
---

# Some Markdown

With text!"#;

  let (toml, markdown) = toml_frontmatter::parse::<Frontmatter>(sample.trim())?;

  assert_toml_snapshot!("toml", toml);
  assert_snapshot!("markdown", markdown);

  Ok(())
}
