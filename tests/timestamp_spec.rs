use anyhow::Result;
use lol_chat_parser::parse_timestamp;

#[test]
fn test_extracts_timestamp() -> Result<()> {
    let input = "17:34 [Team] kozakSyla (Lux): hi in team chat";
    let ts = parse_timestamp(input)?;
    assert_eq!(ts, "17:34");
    Ok(())
}