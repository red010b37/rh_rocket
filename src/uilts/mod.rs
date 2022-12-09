use uuid::Uuid;

pub fn is_valid_guid(s: &str) -> bool {
    match Uuid::parse_str(s) {
        Ok(_) => true,
        Err(_) => false,
    }
}
