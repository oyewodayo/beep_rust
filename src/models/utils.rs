use regex::Regex;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

pub fn generateSlug(name: &str) -> String {
    let slug = name.to_lowercase().replace(" ", "-");
    let re = Regex::new(r"[^a-z0-9-]").unwrap();
    let slug = re.replace_all(&slug, "").to_string();
    let re = Regex::new(r"-+").unwrap();
    let slug = re.replace_all(&slug, "-").to_string();
    let slug = slug.trim_matches('-').to_string();
    
    if slug.is_empty() {
        let mut hasher = DefaultHasher::new();
        name.hash(&mut hasher);
        format!("slug-{}", hasher.finish())
    } else {
        slug
    }
}