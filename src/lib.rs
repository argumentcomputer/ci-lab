#[cfg(test)]
mod test {
    use uno::one;
    use two::two;
    use workspace_test::three;

    #[test]
    fn test_one_two() {
        assert_eq!(one() + two(), three());
    }

    #[ignore]
    #[test]
    fn test_solidity_compatibility_ipa() {
        //panic!()
    }
}
