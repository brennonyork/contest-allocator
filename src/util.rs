/// Converts String types to boolean types
///
/// # Examples
///
/// ```
/// let x: bool = str_to_bool("Y".to_string())
/// ```
pub fn str_to_bool(x: String) -> bool {
    match x.as_ref() {
        "Y" => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_str_to_bool() {
        assert_eq!(true, str_to_bool("Y".to_string()));
        assert_eq!(false, str_to_bool("N".to_string()));
        assert_eq!(false, str_to_bool("".to_string()));
        assert_eq!(false, str_to_bool("Hello, world!".to_string()));
    }
}
