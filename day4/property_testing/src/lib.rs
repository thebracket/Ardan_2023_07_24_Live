pub fn is_email_valid(email: &str) -> bool {
    !email.is_empty() && email.contains('@')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_email() {
        use fake::faker::internet::en::SafeEmail;
        use fake::Fake;

        let email: String = SafeEmail().fake();
        println!("{email}");
        assert!(is_email_valid(&email));
    }

    // Both `Clone` and `Debug` are required by `quickcheck`
    #[derive(Debug, Clone)]
    struct ValidEmailFixture(pub String);

    use fake::faker::internet::en::SafeEmail;
    use fake::Fake;

    impl quickcheck::Arbitrary for ValidEmailFixture {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            let email = SafeEmail().fake();
            Self(email)
        }
    }

    #[quickcheck_macros::quickcheck]
    fn valid_emails_are_parsed_successfully(valid_email: ValidEmailFixture) -> bool {
        is_email_valid(&valid_email.0)
    }
}
