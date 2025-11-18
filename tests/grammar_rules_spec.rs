use anyhow::Result;
use pest::Parser;
use lol_chat_parser::{LolChatParser, Rule};

#[test]
fn rule_time_parses_timestamp() -> Result<()> {
    let mut pairs = LolChatParser::parse(Rule::time, "17:34")?;
    let p = pairs.next().unwrap();
    assert_eq!(p.as_rule(), Rule::time);
    assert_eq!(p.as_str(), "17:34");
    Ok(())
}

#[test]
fn rule_line_parses_full_line() -> Result<()> {
    let input = "17:34 [Team] kozakSyla (Lux): hi in team chat";
    let mut pairs = LolChatParser::parse(Rule::line, input)?;
    let p = pairs.next().unwrap();
    assert_eq!(p.as_rule(), Rule::line);
    assert_eq!(p.as_str(), input);
    Ok(())
}

#[test]
fn rule_line_body_parses_body_only() -> Result<()> {
    let input = "[All] piwkobb (Yone): hello this is all chat msg";
    let mut pairs = LolChatParser::parse(Rule::line_body, input)?;
    let p = pairs.next().unwrap();
    assert_eq!(p.as_rule(), Rule::line_body);
    let mut inner = p.into_inner();
    let first = inner.next().unwrap();
    assert_eq!(first.as_rule(), Rule::chat_message);
    Ok(())
}

#[test]
fn rule_player_name_parses_simple_name() -> Result<()> {
    let mut pairs = LolChatParser::parse(Rule::player_name, "uskin432")?;
    let p = pairs.next().unwrap();
    assert_eq!(p.as_rule(), Rule::player_name);
    assert_eq!(p.as_str(), "uskin432");
    Ok(())
}

#[test]
fn rule_word_parses_word_with_apostrophe() -> Result<()> {
    let mut pairs = LolChatParser::parse(Rule::word, "Cho'Gath")?;
    let p = pairs.next().unwrap();
    assert_eq!(p.as_rule(), Rule::word);
    assert_eq!(p.as_str(), "Cho'Gath");
    Ok(())
}

#[test]
fn rule_champion_name_single_word() -> Result<()> {
    let mut pairs = LolChatParser::parse(Rule::champion_name, "Yone")?;
    let p = pairs.next().unwrap();
    assert_eq!(p.as_rule(), Rule::champion_name);
    assert_eq!(p.as_str(), "Yone");
    Ok(())
}

#[test]
fn rule_champion_name_multi_word() -> Result<()> {
    let mut pairs = LolChatParser::parse(Rule::champion_name, "Tahm Kench")?;
    let p = pairs.next().unwrap();
    assert_eq!(p.as_rule(), Rule::champion_name);
    assert_eq!(p.as_str(), "Tahm Kench");
    Ok(())
}

#[test]
fn rule_player_with_champion_parses_pair() -> Result<()> {
    let input = "BorysBulba (Tahm Kench)";
    let mut pairs = LolChatParser::parse(Rule::player_with_champion, input)?;
    let p = pairs.next().unwrap();
    assert_eq!(p.as_rule(), Rule::player_with_champion);
    assert_eq!(p.as_str(), input);
    Ok(())
}

#[test]
fn rule_channel_tag_parses_all_team_party() -> Result<()> {
    for (rule_input, expected) in [
        ("[All]", "[All]"),
        ("[Team]", "[Team]"),
        ("[Party]", "[Party]"),
    ] {
        let mut pairs = LolChatParser::parse(Rule::channel_tag, rule_input)?;
        let p = pairs.next().unwrap();
        assert_eq!(p.as_rule(), Rule::channel_tag);
        assert_eq!(p.as_str(), expected);
    }
    Ok(())
}

#[test]
fn rule_number_parses_integer() -> Result<()> {
    let mut pairs = LolChatParser::parse(Rule::number, "149")?;
    let p = pairs.next().unwrap();
    assert_eq!(p.as_rule(), Rule::number);
    assert_eq!(p.as_str(), "149");
    Ok(())
}

#[test]
fn rule_percentage_parses_percentage_digits() -> Result<()> {
    let mut pairs = LolChatParser::parse(Rule::percentage, "33")?;
    let p = pairs.next().unwrap();
    assert_eq!(p.as_rule(), Rule::percentage);
    assert_eq!(p.as_str(), "33");
    Ok(())
}

#[test]
fn rule_name_phrase_parses_multi_word() -> Result<()> {
    let mut pairs = LolChatParser::parse(Rule::name_phrase, "Feat of Warfare")?;
    let p = pairs.next().unwrap();
    assert_eq!(p.as_rule(), Rule::name_phrase);
    assert_eq!(p.as_str(), "Feat of Warfare");
    Ok(())
}

#[test]
fn rule_channel_chat_message_parses_all_chat() -> Result<()> {
    let input = "[All] piwkobb (Yone): hello this is all chat msg";
    let mut pairs = LolChatParser::parse(Rule::channel_chat_message, input)?;
    let p = pairs.next().unwrap();
    assert_eq!(p.as_rule(), Rule::channel_chat_message);
    assert_eq!(p.as_str(), input);
    Ok(())
}

#[test]
fn rule_channel_chat_message_parses_team_chat() -> Result<()> {
    let input = "[Team] kozakSyla (Lux): hi in team chat";
    let mut pairs = LolChatParser::parse(Rule::channel_chat_message, input)?;
    let p = pairs.next().unwrap();
    assert_eq!(p.as_rule(), Rule::channel_chat_message);
    assert_eq!(p.as_str(), input);
    Ok(())
}

#[test]
fn rule_channel_chat_message_parses_party_chat() -> Result<()> {
    let input = "[Party] piwkobb (Yone): Cho'Gath Heartsteel - 20 charges";
    let mut pairs = LolChatParser::parse(Rule::channel_chat_message, input)?;
    let p = pairs.next().unwrap();
    assert_eq!(p.as_rule(), Rule::channel_chat_message);
    assert_eq!(p.as_str(), input);
    Ok(())
}

#[test]
fn rule_bare_chat_message_parses_player_chat() -> Result<()> {
    let input = "uskin432 (Warwick): ward top bush";
    let mut pairs = LolChatParser::parse(Rule::bare_chat_message, input)?;
    let p = pairs.next().unwrap();
    assert_eq!(p.as_rule(), Rule::bare_chat_message);
    assert_eq!(p.as_str(), input);
    Ok(())
}

#[test]
fn rule_chat_message_parses_any_chat() -> Result<()> {
    let input = "[All] piwkobb (Yone): hello this is all chat msg";
    let mut pairs = LolChatParser::parse(Rule::chat_message, input)?;
    let p = pairs.next().unwrap();
    assert_eq!(p.as_rule(), Rule::chat_message);
    let mut inner = p.into_inner();
    let first = inner.next().unwrap();
    assert_eq!(first.as_rule(), Rule::channel_chat_message);

    Ok(())
}

#[test]
fn rule_chat_text_parses_full_message() -> Result<()> {
    let input = "hello this is all chat msg";
    let mut pairs = LolChatParser::parse(Rule::chat_text, input)?;
    let p = pairs.next().unwrap();
    assert_eq!(p.as_rule(), Rule::chat_text);
    assert_eq!(p.as_str(), input);
    Ok(())
}

#[test]
fn rule_kill_first_blood_event_parses() -> Result<()> {
    let input = "kozakSyla (Lux) has drawn first blood!";
    let mut pairs = LolChatParser::parse(Rule::kill_first_blood_event, input)?;
    let p = pairs.next().unwrap();
    assert_eq!(p.as_rule(), Rule::kill_first_blood_event);
    assert_eq!(p.as_str(), input);
    Ok(())
}

#[test]
fn rule_shutdown_bounty_parses() -> Result<()> {
    let input = "(Bonus Bounty: 149G)";
    let mut pairs = LolChatParser::parse(Rule::shutdown_bounty, input)?;
    let p = pairs.next().unwrap();
    assert_eq!(p.as_rule(), Rule::shutdown_bounty);
    assert_eq!(p.as_str(), input);
    Ok(())
}

#[test]
fn rule_kill_shutdown_event_parses() -> Result<()> {
    let input = "Golf4f (Mel) has shut down BorysBulba (Tahm Kench)! (Bonus Bounty: 149G)";
    let mut pairs = LolChatParser::parse(Rule::kill_shutdown_event, input)?;
    let p = pairs.next().unwrap();
    assert_eq!(p.as_rule(), Rule::kill_shutdown_event);
    assert_eq!(p.as_str(), input);
    Ok(())
}

#[test]
fn rule_team_name_parses_enemy_team() -> Result<()> {
    let mut pairs = LolChatParser::parse(Rule::team_name, "Enemy team")?;
    let p = pairs.next().unwrap();
    assert_eq!(p.as_rule(), Rule::team_name);
    assert_eq!(p.as_str(), "Enemy team");
    Ok(())
}

#[test]
fn rule_team_feat_event_parses() -> Result<()> {
    let input = "Enemy team has completed the Feat of Warfare!";
    let mut pairs = LolChatParser::parse(Rule::team_feat_event, input)?;
    let p = pairs.next().unwrap();
    assert_eq!(p.as_rule(), Rule::team_feat_event);
    assert_eq!(p.as_str(), input);
    Ok(())
}

#[test]
fn rule_purchase_event_parses() -> Result<()> {
    let input = "BorysBulba (Tahm Kench) purchased Control Ward";
    let mut pairs = LolChatParser::parse(Rule::purchase_event, input)?;
    let p = pairs.next().unwrap();
    assert_eq!(p.as_rule(), Rule::purchase_event);
    assert_eq!(p.as_str(), input);
    Ok(())
}

#[test]
fn rule_target_player_event_parses() -> Result<()> {
    let input = "piwkobb (Yone) has targeted TheMiozl - (Renekton)";
    let mut pairs = LolChatParser::parse(Rule::target_player_event, input)?;
    let p = pairs.next().unwrap();
    assert_eq!(p.as_rule(), Rule::target_player_event);
    assert_eq!(p.as_str(), input);
    Ok(())
}

#[test]
fn rule_target_objective_event_parses() -> Result<()> {
    let input = "kozakSyla (Lux) has targeted the Power Flower (33%)";
    let mut pairs = LolChatParser::parse(Rule::target_objective_event, input)?;
    let p = pairs.next().unwrap();
    assert_eq!(p.as_rule(), Rule::target_objective_event);
    assert_eq!(p.as_str(), input);
    Ok(())
}

#[test]
fn rule_rampage_event_parses() -> Result<()> {
    let input = "kozakSyla (Lux) is on rampage!";
    let mut pairs = LolChatParser::parse(Rule::rampage_event, input)?;
    let p = pairs.next().unwrap();
    assert_eq!(p.as_rule(), Rule::rampage_event);
    assert_eq!(p.as_str(), input);
    Ok(())
}

#[test]
fn rule_ping_on_the_way_event_parses() -> Result<()> {
    let input = "uskin432 (Warwick) is on the way";
    let mut pairs = LolChatParser::parse(Rule::ping_on_the_way_event, input)?;
    let p = pairs.next().unwrap();
    assert_eq!(p.as_rule(), Rule::ping_on_the_way_event);
    assert_eq!(p.as_str(), input);
    Ok(())
}

#[test]
fn rule_generic_player_event_parses() -> Result<()> {
    let input = "piwkobb (Yone) did something weird";
    let mut pairs = LolChatParser::parse(Rule::generic_player_event, input)?;
    let p = pairs.next().unwrap();
    assert_eq!(p.as_rule(), Rule::generic_player_event);
    assert_eq!(p.as_str(), input);
    Ok(())
}

#[test]
fn rule_generic_tail_parses_rest() -> Result<()> {
    let input = "did something weird";
    let mut pairs = LolChatParser::parse(Rule::generic_tail, input)?;
    let p = pairs.next().unwrap();
    assert_eq!(p.as_rule(), Rule::generic_tail);
    assert_eq!(p.as_str(), input);
    Ok(())
}

#[test]
fn rule_generic_text_parses_any_text() -> Result<()> {
    let input = "some totally unknown text here";
    let mut pairs = LolChatParser::parse(Rule::generic_text, input)?;
    let p = pairs.next().unwrap();
    assert_eq!(p.as_rule(), Rule::generic_text);
    assert_eq!(p.as_str(), input);
    Ok(())
}
