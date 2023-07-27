use mockall::*;

#[automock]
pub trait MyTrait {
    fn calculate(&self, x: u32) -> u32;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_my_mock() {
        // Mockall has added a constructor to build a mocked implementation
        let mut mock = MockMyTrait::new();
        // Tell the mocked trait that `calculate` is expected to return 42
        mock.expect_calculate().return_const(42u32);
        assert_eq!(mock.calculate(12), 42);
    }

    #[test]
fn test_my_mock_fn() {
    // Mockall has added a constructor to build a mocked implementation
    let mut mock = MockMyTrait::new();
    // Tell the mocked trait that `calculate` is expected to return 42
    mock.expect_calculate().returning(|x| x + 30);
    assert_eq!(mock.calculate(12), 42);
}
}