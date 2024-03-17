#[cfg(test)]
mod tests {
    use dsa::lcs::*;

    #[test]
    fn test_lcs() {
        assert_eq!("a", lcs("a", "a"));
        assert_eq!("", lcs("", ""));
        assert_eq!("", lcs("", "a"));
        assert_eq!("b", lcs("ab", "ba"));
        assert_eq!("baac", lcs("abbaac", "bacbacba"));
    }

    #[test]
    fn test_lcs_length() {
        assert_eq!(1, lcs_length("a", "a"));
        assert_eq!(0, lcs_length("", ""));
        assert_eq!(0, lcs_length("", "a"));
        assert_eq!(1, lcs_length("ab", "ba"));
        assert_eq!(4, lcs_length("abbaac", "bacbacba"));
    }
}
