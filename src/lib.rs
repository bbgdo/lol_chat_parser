use std::collections::{HashMap, HashSet};

use anyhow::{anyhow, Result};
use pest::Parser;
use pest_derive::Parser;
use serde::Serialize;

#[derive(Parser)]
#[grammar = "grammar.pest"]

pub struct LolChatParser;
#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ChatChannel {
    All,
    Team,
    Party,
    Player,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct ChatMessage {
    pub time: String,
    pub channel: ChatChannel,
    pub player: String,
    pub champion: String,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct KillEvent {
    pub time: String,
    pub killer: String,
    pub killer_champion: String,
    pub victim: Option<String>,
    pub victim_champion: Option<String>,
    pub bounty: Option<u32>,
    pub is_shutdown: bool,
    pub is_first_blood: bool,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct ObjectiveEvent {
    pub time: String,
    pub team: Option<String>,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct PlayerSummary {
    pub name: String,
    pub champions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct SystemLine {
    pub text: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct ParsedLog {
    pub players: Vec<PlayerSummary>,
    pub kills: Vec<KillEvent>,
    pub events: Vec<ObjectiveEvent>,
    pub messages: Vec<ChatMessage>,
    pub system: Vec<SystemLine>,
}

pub fn parse_timestamp(line: &str) -> Result<String> {
    let (time, _rest) = parse_time_and_rest(line)?;
    Ok(time)
}

fn parse_time_and_rest(line: &str) -> Result<(String, String)> {
    let mut pairs = LolChatParser::parse(Rule::line, line)
        .map_err(|e| anyhow!("Parse error for `line`: {e}"))?;
    let line_pair = pairs.next().ok_or_else(|| anyhow!("No match for `line`"))?;
    let mut inner = line_pair.into_inner();

    let time_pair = inner
        .next()
        .ok_or_else(|| anyhow!("No `time` pair inside `line`"))?;
    debug_assert_eq!(time_pair.as_rule(), Rule::time);

    let time = time_pair.as_str().to_string();

    let rest = if let Some(idx) = line.find(' ') {
        if idx + 1 < line.len() {
            line[idx + 1..].to_string()
        } else {
            String::new()
        }
    } else {
        String::new()
    };

    Ok((time, rest))
}

pub fn parse_log(input: &str) -> ParsedLog {
    let mut players: HashMap<String, HashSet<String>> = HashMap::new();
    let mut kills = Vec::new();
    let mut events = Vec::new();
    let mut messages = Vec::new();
    let mut system = Vec::new();

    for line in input.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        let (time, rest) = match parse_time_and_rest(trimmed) {
            Ok(tr) => tr,
            Err(_) => {
                system.push(SystemLine {
                    text: trimmed.to_string(),
                });
                continue;
            }
        };

        if let Some(chat) = parse_chat_message(&time, &rest) {
            add_player(&mut players, &chat.player, &chat.champion);
            messages.push(chat);
            continue;
        }

        if let Some(kill) = parse_kill_event(&time, &rest) {
            add_player(&mut players, &kill.killer, &kill.killer_champion);
            if let (Some(victim), Some(vchamp)) = (&kill.victim, &kill.victim_champion) {
                add_player(&mut players, victim, vchamp);
            }
            kills.push(kill);
            continue;
        }

        if let Some(obj) = parse_objective_event(&time, &rest) {
            if let Some((player, champ, _)) = parse_player_with_champion_prefix(&rest) {
                add_player(&mut players, &player, &champ);
            }

            if let Some((target_player, target_champion)) = parse_target_player(&rest) {
                add_player(&mut players, &target_player, &target_champion);
            }

            events.push(obj);
            continue;
        }

        if let Some((player, champ, _)) = parse_player_with_champion_prefix(&rest) {
            add_player(&mut players, &player, &champ);
        }
    }

    let mut players_vec: Vec<PlayerSummary> = players
        .into_iter()
        .map(|(name, champs)| {
            let mut list: Vec<String> = champs.into_iter().collect();
            list.sort();
            PlayerSummary {
                name,
                champions: list,
            }
        })
        .collect();

    players_vec.sort_by(|a, b| a.name.cmp(&b.name));

    ParsedLog {
        players: players_vec,
        kills,
        events,
        messages,
        system,
    }
}

fn add_player(players: &mut HashMap<String, HashSet<String>>, player: &str, champion: &str) {
    players
        .entry(player.to_string())
        .or_insert_with(HashSet::new)
        .insert(champion.to_string());
}

fn parse_chat_message(time: &str, rest: &str) -> Option<ChatMessage> {
    let trimmed = rest.trim_start();

    let (channel, after_channel) = if trimmed.starts_with('[') {
        let closing = trimmed.find(']')?;
        let tag = &trimmed[1..closing];
        let channel = match tag {
            "All" => ChatChannel::All,
            "Team" => ChatChannel::Team,
            "Party" => ChatChannel::Party,
            _ => return None,
        };
        let remainder = trimmed[closing + 1..].trim_start();
        (channel, remainder)
    } else {
        (ChatChannel::Player, trimmed)
    };

    let (player, champion, after_pc) = parse_player_with_champion_prefix(after_channel)?;
    let after_pc_trimmed = after_pc.trim_start();

    if !after_pc_trimmed.starts_with(':') {
        return None;
    }
    let message = after_pc_trimmed[1..].trim_start();

    Some(ChatMessage {
        time: time.to_string(),
        channel,
        player,
        champion,
        text: message.to_string(),
    })
}

fn parse_player_with_champion_prefix(text: &str) -> Option<(String, String, &str)> {
    let trimmed = text.trim_start();

    let open = trimmed.find('(')?;
    let close_rel = trimmed[open..].find(')')?;
    let close = open + close_rel;

    let player = trimmed[..open].trim();
    if player.is_empty() {
        return None;
    }

    let champion = trimmed[open + 1..close].trim();
    if champion.is_empty() {
        return None;
    }

    let remainder = &trimmed[close + 1..];
    Some((player.to_string(), champion.to_string(), remainder))
}

fn parse_target_player(rest: &str) -> Option<(String, String)> {
    let text = rest.trim();

    let idx = text.find(" has targeted ")?;
    let after = &text[idx + " has targeted ".len()..];

    let dash_idx = after.find(" - (")?;
    let target_name = after[..dash_idx].trim();
    if target_name.is_empty() {
        return None;
    }

    let after_dash = &after[dash_idx + " - (".len()..];
    let close_paren = after_dash.find(')')?;
    let champion = after_dash[..close_paren].trim();
    if champion.is_empty() {
        return None;
    }

    Some((target_name.to_string(), champion.to_string()))
}

fn parse_kill_event(time: &str, rest: &str) -> Option<KillEvent> {
    let text = rest.trim();

    if let Some(idx) = text.find(" has shut down ") {
        let (killer_part, tail) = text.split_at(idx);
        let tail = &tail[" has shut down ".len()..];

        let (killer, killer_champ, _) = parse_player_with_champion_prefix(killer_part)?;

        let exclam = tail.find('!')?;
        let victim_part = &tail[..exclam];
        let bonus_part = &tail[exclam + 1..];

        let (victim, victim_champ, _) = parse_player_with_champion_prefix(victim_part)?;

        let bonus_start = bonus_part.find("Bonus Bounty:")?;
        let bonus_text = &bonus_part[bonus_start + "Bonus Bounty:".len()..];
        let bonus_text = bonus_text.trim();
        let digits: String = bonus_text
            .chars()
            .take_while(|c| c.is_ascii_digit())
            .collect();
        let bounty = digits.parse::<u32>().ok();

        return Some(KillEvent {
            time: time.to_string(),
            killer,
            killer_champion: killer_champ,
            victim: Some(victim),
            victim_champion: Some(victim_champ),
            bounty,
            is_shutdown: true,
            is_first_blood: false,
        });
    }

    if let Some(idx) = text.find(" has drawn first blood!") {
        let (killer_part, _) = text.split_at(idx);
        let (killer, killer_champ, _) = parse_player_with_champion_prefix(killer_part)?;

        return Some(KillEvent {
            time: time.to_string(),
            killer,
            killer_champion: killer_champ,
            victim: None,
            victim_champion: None,
            bounty: None,
            is_shutdown: false,
            is_first_blood: true,
        });
    }

    None
}

fn parse_objective_event(time: &str, rest: &str) -> Option<ObjectiveEvent> {
    let text = rest.trim();

    if text.contains(" has completed the ") {
        let team_opt = if text.starts_with("Enemy team") {
            Some("Enemy team".to_string())
        } else if text.starts_with("Ally team") {
            Some("Ally team".to_string())
        } else if text.starts_with("Blue team") {
            Some("Blue team".to_string())
        } else if text.starts_with("Red team") {
            Some("Red team".to_string())
        } else {
            None
        };

        return Some(ObjectiveEvent {
            time: time.to_string(),
            team: team_opt,
            description: text.to_string(),
        });
    }

    if text.contains("has targeted")
        || text.contains("purchased ")
        || text.contains("is on rampage!")
        || text.contains(" is on the way")
        || text.contains(" is missing")
        || text.contains(" is retreating")
        || text.contains(" is in danger")
        || text.contains(" needs vision")
    {
        return Some(ObjectiveEvent {
            time: time.to_string(),
            team: None,
            description: text.to_string(),
        });
    }

    None
}
