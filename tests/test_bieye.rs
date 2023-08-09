/// Testing: bieye.rs

#[allow(dead_code)]
#[cfg(test)]
#[path = "../src/bieye.rs"]
mod bieye;

mod tests_bieye {
    use super::*;

    #[test]
    fn test_color_text() {
        // Check if the TERM environment variable is set to "dumb"
        println!("TERM={}", std::env::var("TERM").unwrap_or_default());
        if std::env::var("TERM").unwrap_or_default() == "dumb" {
            eprintln!("WARNING: Skipping color test due to TERM=dumb terminal");
            return;
        }
        // TERM=tmux-256color

        let mut be = bieye::Bieye {
            text_input: "TESTING".to_string(),
            is_colored: true,
            is_dimmed: false,
            ..Default::default()
        };
        be.process_text();

        assert_eq!(be.text_output, "\u{1b}[1;33mTES\u{1b}[0mTING");
    }

    // #[test]
    // fn test_dim_text() {
    //     let mut be = bieye::Bieye {
    //         text_input: "TESTING".to_string(),
    //         is_colored: false,
    //         is_dimmed: true,
    //         ..Default::default()
    //     };
    //     be.process_text();
    //
    //     assert_eq!(
    //         be.text_output,
    //         "\u{1b}[1mTES\u{1b}[0m\u{1b}[2mTING\u{1b}[0m"
    //     );
    // }
    //
    // #[test]
    // fn test_color_and_dim_text() {
    //     // let mut bieye = bieye::Bieye::default();
    //     let mut be = bieye::Bieye {
    //         text_input: "TESTING".to_string(),
    //         is_colored: true,
    //         is_dimmed: true,
    //         ..Default::default()
    //     };
    //     be.process_text();
    //
    //     assert_eq!(
    //         be.text_output,
    //         "\u{1b}[1;33mTES\u{1b}[0m\u{1b}[2mTING\u{1b}[0m"
    //     );
    // }
}
