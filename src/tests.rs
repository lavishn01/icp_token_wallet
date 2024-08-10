

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_send_tokens() {
        let result = send_tokens(TransferArgs {
            to: "recipient_address".to_string(),
            amount: 50,
        });
        assert!(result.is_ok());
    }

    #[test]
    fn test_receive_tokens() {
        let result = receive_tokens(100);
        assert!(result.is_ok());
        assert_eq!(get_balance(), 100);
    }

    #[test]
    fn test_balance() {
        assert_eq!(get_balance(), 100);
    }
}
