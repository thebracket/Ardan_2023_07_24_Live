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
}
