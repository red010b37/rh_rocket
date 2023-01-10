use regex::Regex;
use uuid::Uuid;

pub fn is_valid_guid(s: &str) -> bool {
    match Uuid::parse_str(s) {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub fn gen_slug(title: String, gen_id: i32) -> String {
    // Make the string lowercase
    let s_lower = title.to_lowercase();

    // Remove all characters except for letters, numbers, and spaces
    let s_filtered = Regex::new(r"[^a-z0-9\s]")
        .unwrap()
        .replace_all(&s_lower, "");

    // Replace multiple spaces with a single hyphen
    let s_hyphenated = Regex::new(r"\s+").unwrap().replace_all(&s_filtered, "-");

    // Convert to a String
    let mut s_final: String = s_hyphenated.into();

    s_final = format!("{}-{}", s_final, gen_id.to_string());
    println!("Modified string: {}", s_final);
    return s_final;
}
