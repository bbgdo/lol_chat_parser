use pest::Parser;
use pest_derive::Parser;
use anyhow::{anyhow, Result};

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct LolChatParser;

pub fn parse_timestamp(line: &str) -> Result<String> {
    let mut pairs = LolChatParser::parse(Rule::line, line).map_err(|e| anyhow!("Parse error: {e}"))?;
    let line_pair = pairs
        .next()
        .ok_or_else(|| anyhow!("No match for line"))?;
    let mut inner = line_pair.into_inner();
    let time = inner
        .next()
        .ok_or_else(|| anyhow!("No timestamp found"))?;

    debug_assert_eq!(time.as_rule(), Rule::time);

    Ok(time.as_str().to_string())
}
