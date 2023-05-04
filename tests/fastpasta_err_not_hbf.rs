use crate::util::*;
mod util;

const MATCH_PATTERNS: [&str; 6] = [
    "(?i)Trigger Type.*0x6A03",
    "(?i)Trigger Type.*SOC",
    "(?i)RDH.*Version.*7",
    "(?i)Total.*RDHs.*2",
    "(?i)Total.*hbfs.*0",
    "(?i)((layers)|(staves)).*((layers)|(staves)).*L0_12",
];

#[test]
fn err_not_hbf_detect_page_error() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("fastpasta")?;
    let re = fancy_regex::Regex::new(r"(?i)ERrOR - 0xa0.*pages")?; // case insensitive
    let pred_one_match = predicate::function(|&x| re.find_iter(x).count() == 1);

    // Verifying the regex predicate
    let test_is_false = pred_one_match.eval(&"test");
    assert!(!test_is_false);
    let test_is_true = pred_one_match.eval(&"error - 0xa0 something pages something");
    assert!(test_is_true);

    cmd.arg(FILE_ERR_NOT_HBF).arg("check").arg("all");

    cmd.assert().success();

    // Take the output of stderr and convert it to string
    let res = cmd.output()?.stderr;
    let str_res = std::str::from_utf8(&res).expect("invalid utf-8 sequence");

    // Compare with regex predicate
    assert!(pred_one_match.eval(&str_res));

    Ok(())
}

#[test]
fn view_hbf() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("fastpasta")?;

    cmd.arg(FILE_ERR_NOT_HBF).arg("view").arg("hbf");

    use predicate::str::contains;
    cmd.assert().success().stdout(
        contains("RDH").count(2).and(
            contains("IHW").count(2).and(
                contains("TDH")
                    .count(2)
                    .and(contains("TDT").count(2).and(contains("DDW").count(0))),
            ),
        ),
    );

    Ok(())
}

#[test]
fn view_rdh() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("fastpasta")?;

    cmd.arg(FILE_ERR_NOT_HBF).arg("view").arg("rdh");

    cmd.assert()
        .success()
        .stdout(is_match(": .* (7|6) .* 64 .* (0|2)")?.count(2));

    Ok(())
}

#[test]
fn check_sanity() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("fastpasta")?;

    cmd.arg(FILE_ERR_NOT_HBF).arg("check").arg("sanity");
    cmd.assert().success();

    for pattern in MATCH_PATTERNS {
        assert!(match_on_output(&cmd.output()?.stdout, pattern, 1));
    }

    Ok(())
}

#[test]
fn check_sanity_debug_verbosity() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("fastpasta")?;

    cmd.arg(FILE_ERR_NOT_HBF)
        .arg("check")
        .arg("sanity")
        .arg("-v3");

    cmd.assert().success();

    assert!(match_on_output(&cmd.output()?.stderr, "(?i)loaded.*rdh", 2));

    Ok(())
}