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
}
