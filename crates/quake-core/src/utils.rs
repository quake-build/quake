#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(unix)]
    fn test_get_user_uid() {
        let env_uid = std::env::var("UID")
            .expect("env var UID not set")
            .parse()
            .expect("failed to parse UID env var");
        assert_eq!(get_user_uid(), env_uid);
    }
}
