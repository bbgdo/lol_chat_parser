use pest::Parser;
use pest_derive::Parser;
use anyhow::{anyhow, Result};

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct LolChatParser;

pub fn parse_timestamp(line: &str) -> Result<String> {
    let pairs = LolChatParser::parse(Rule::line, line).map_err(|e| anyhow!("Parse error: {e}"))?;
    for pair in pairs {
        if pair.as_rule() == Rule::time {
            return Ok(pair.as_str().to_string());
        }
    }
    Err(anyhow!("No timestamp found"))
}
