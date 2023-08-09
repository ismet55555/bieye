/// Testing: cli_args.rs

#[allow(dead_code)]
#[cfg(test)]
#[path = "../src/cli_args.rs"]
mod cli_args;

mod tests_cli_args {
    use super::*;

    #[test]
    fn test_cli_args_parsing() {
        let args = cli_args::CliArgs {
            text: Some("hello".to_string()),
            color: true,
            dim: true,
        };

        assert_eq!(args.text, Some("hello".to_string()));
        assert!(args.color);
        assert!(args.dim);
    }
}
