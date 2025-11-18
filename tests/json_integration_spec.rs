use anyhow::Result;
use lol_chat_parser::{parse_log, ChatChannel, ParsedLog};
use serde_json::{to_value, Value};

#[test]
fn parses_sample_log_and_produces_expected_json_shape() -> Result<()> {
    let log = r#"
Type /help for a list of commands
00:42 uskin432 (Warwick) is on the way
00:52 kozakSyla (Lux) has drawn first blood!
02:24 Enemy team has completed the Feat of Warfare!
13:05 Golf4f (Mel) has shut down BorysBulba (Tahm Kench)! (Bonus Bounty: 149G)
13:08 [Party] piwkobb (Yone): Cho'Gath Heartsteel - 20 charges
13:10 BorysBulba (Tahm Kench) purchased Control Ward
14:46 kozakSyla (Lux) is on rampage!
15:40 piwkobb (Yone) has targeted TheMiozl - (Renekton)
16:53 [All] piwkobb (Yone): hello this is all chat msg
17:34 [Team] kozakSyla (Lux): hi in team chat
18:32 kozakSyla (Lux) has targeted the Power Flower (33%)
"#;

    let parsed: ParsedLog = parse_log(log);

    let mut players: Vec<_> = parsed.players.iter().map(|p| p.name.as_str()).collect();
    players.sort();
    assert!(players.contains(&"uskin432"));
    assert!(players.contains(&"kozakSyla"));
    assert!(players.contains(&"Golf4f"));
    assert!(players.contains(&"BorysBulba"));
    assert!(players.contains(&"piwkobb"));
    assert!(players.contains(&"TheMiozl"));

    assert!(parsed.kills.len() >= 2);
    assert!(parsed.kills.iter().any(|k| k.is_first_blood));
    assert!(parsed.kills.iter().any(|k| k.is_shutdown));

    assert!(!parsed.events.is_empty());
    assert!(parsed
        .events
        .iter()
        .any(|o| o.description.contains("Feat of Warfare")));
    assert!(parsed
        .events
        .iter()
        .any(|o| o.description.contains("Power Flower")));

    // 4. Чат-повідомлення — перевіряємо по каналах
    assert!(parsed.messages.iter().any(|m| m.channel == ChatChannel::Party));
    assert!(parsed.messages.iter().any(|m| m.channel == ChatChannel::All));
    assert!(parsed.messages.iter().any(|m| m.channel == ChatChannel::Team));

    let party_msgs: Vec<_> = parsed
        .messages
        .iter()
        .filter(|m| m.channel == ChatChannel::Party)
        .collect();
    assert_eq!(party_msgs.len(), 1);
    assert_eq!(party_msgs[0].player, "piwkobb");
    assert_eq!(party_msgs[0].champion, "Yone");

    let all_msgs: Vec<_> = parsed
        .messages
        .iter()
        .filter(|m| m.channel == ChatChannel::All)
        .collect();
    assert_eq!(all_msgs.len(), 1);
    assert_eq!(all_msgs[0].player, "piwkobb");
    assert_eq!(all_msgs[0].text, "hello this is all chat msg");

    let team_msgs: Vec<_> = parsed
        .messages
        .iter()
        .filter(|m| m.channel == ChatChannel::Team)
        .collect();
    assert_eq!(team_msgs.len(), 1);
    assert_eq!(team_msgs[0].player, "kozakSyla");

    assert_eq!(parsed.system.len(), 1);
    assert_eq!(
        parsed.system[0].text,
        "Type /help for a list of commands"
    );

    let json: Value = to_value(&parsed)?;
    assert!(json.get("players").is_some());
    assert!(json.get("kills").is_some());
    assert!(json.get("events").is_some());
    assert!(json.get("messages").is_some());
    assert!(json.get("system").is_some());

    Ok(())
}
