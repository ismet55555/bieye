/// Testing: CLI as a whole end to end
use color_eyre::eyre::Result;

use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs

#[cfg(test)]
mod tests_cli {
    use super::*;

    #[test]
    fn test_cli_help() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("bieye")?;

        cmd.arg("--help");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("This CLI tool"));

        Ok(())
    }

    #[test]
    fn test_words() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("bieye")?;

        cmd.arg("'hello there'");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("hello there"));

        Ok(())
    }

    #[test]
    fn test_non_utf() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("bieye")?;

        cmd.arg("'Ḇ𝖢𝕯٤ḞԍНǏ𝙅ƘԸⲘ𝙉০Ρ𝗤Ɍ𝓢ȚЦ𝒱Ѡ𝓧ƳȤѧᖯ'");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("Ḇ𝖢𝕯٤ḞԍНǏ𝙅ƘԸⲘ𝙉০Ρ𝗤Ɍ𝓢ȚЦ𝒱Ѡ𝓧ƳȤѧᖯ"));

        Ok(())
    }
}
