#[test]
#[should_panic(expected = "Missing frontmatter")]
fn test_missing_frontmatter() {
  toml_frontmatter::parse::<()>("# Heading").unwrap();
}

#[test]
#[should_panic(expected = "Frontmatter not at beginning of data")]
fn test_frontmatter_not_at_start() {
  let sample = r#"
# Heading

---toml
date = "2023-01-01"
---

Text."#;

  toml_frontmatter::parse::<()>(sample).unwrap();
}
