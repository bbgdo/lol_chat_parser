# League of Legends Chat Parser (LoLCP)

**Brief Description:**  
`LoLCP` is a parser for **League of Legends** chat logs.  
It uses a custom grammar written in `pest` to extract structured events such as chat messages, kill events, team achievements, purchases, map-objective interactions, and smart pings.

---

## 📌 Current Status

The parser is now capable of handling **most common LoL chat log structures** and extracting:

- timestamps
- players and their champions
- chat messages (All / Team / Party / Player)
- kill events (first blood, shutdown, bonus bounty)
- team achievements (e.g., Feat of Warfare)
- purchases
- targeting events (players and map objectives)
- map pings (on the way, missing, retreating, danger, etc.)

However, **LoL logs are complex and frequently updated (and not documented 🙁 )**, so
> **I do not guarantee 100% correctness across all client versions or rare system messages.**

Nevertheless, the included grammar and logic are designed to be modular and extensible.

---

## 📘 Technical Description

The goal of this project is to take a plain-text League of Legends log and transform it into structured **JSON**, containing:

- all **players** mentioned in the match
- which **champions** they played
- game **events** such as kills, shutdowns, objectives, team achievements
- all chat messages (All / Team / Party / Player)
- miscellaneous gameplay events (targeting, purchases, map interactions)
- all tied to their corresponding **timestamps**

This allows further analysis such as:
- reconstructing kill timeline
- identifying communication patterns
- aggregating player interactions
- tracking objective control
- building match summaries

---

## 📜 Parsing Process Overview

Each chat line *(except the first one by system)* in the log typically follows this pattern:
```
<timestamp><event>
```
### Examples:
```
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
```
The parser breaks these into structured components using `pest` grammar rules.

---

## 🧩 Grammar (overview)

The full grammar is located in:
```
src/grammar.pest
```

Below is a shortened overview of the **most important rules**:

```pest
line = { time ~ " " ~ line_body }

time = @{ ASCII_DIGIT ~ ASCII_DIGIT ~ ":" ~ ASCII_DIGIT ~ ASCII_DIGIT }

player_with_champion = {
    player_name ~ " " ~ "(" ~ champion_name ~ ")"
}

chat_message = {
      channel_chat_message
    | bare_chat_message
}

kill_shutdown_event = {
    player_with_champion ~ " has shut down " ~ player_with_champion ~ "!" ~ " " ~ shutdown_bounty
}

target_objective_event = {
    player_with_champion ~ " has targeted " ~ name_phrase ~ " (" ~ percentage ~ "%)"
}

ping_on_the_way_event = {
    player_with_champion ~ " " ~ ping_phrase
}
```
***(This is only a simplified subset for demonstration—see the full grammar file for all details.)***

---

## 🚀 Usage (CLI)

The project includes a command-line interface with several commands:
### Show help
```
cargo run -- help
```
### Show credits
```
cargo run -- credits
```
### Parse a log file
```
cargo run -- parse lol_chat_example.txt
```
### 📤 Example Output
<details>
<summary>here is output json</summary>

``` json
{
  "players": [
    {
      "name": "BorysBulba",
      "champions": [
        "Tahm Kench"
      ]
    },
    {
      "name": "Golf4f",
      "champions": [
        "Mel"
      ]
    },
    {
      "name": "TheMiozl",
      "champions": [
        "Renekton"
      ]
    },
    {
      "name": "kozakSyla",
      "champions": [
        "Lux"
      ]
    },
    {
      "name": "piwkobb",
      "champions": [
        "Yone"
      ]
    },
    {
      "name": "uskin432",
      "champions": [
        "Warwick"
      ]
    }
  ],
  "kills": [
    {
      "time": "00:52",
      "killer": "kozakSyla",
      "killer_champion": "Lux",
      "victim": null,
      "victim_champion": null,
      "bounty": null,
      "is_shutdown": false,
      "is_first_blood": true
    },
    {
      "time": "13:05",
      "killer": "Golf4f",
      "killer_champion": "Mel",
      "victim": "BorysBulba",
      "victim_champion": "Tahm Kench",
      "bounty": 149,
      "is_shutdown": true,
      "is_first_blood": false
    }
  ],
  "events": [
    {
      "time": "00:42",
      "team": null,
      "description": "uskin432 (Warwick) is on the way"
    },
    {
      "time": "02:24",
      "team": "Enemy team",
      "description": "Enemy team has completed the Feat of Warfare!"
    },
    {
      "time": "13:10",
      "team": null,
      "description": "BorysBulba (Tahm Kench) purchased Control Ward"
    },
    {
      "time": "14:46",
      "team": null,
      "description": "kozakSyla (Lux) is on rampage!"
    },
    {
      "time": "15:40",
      "team": null,
      "description": "piwkobb (Yone) has targeted TheMiozl - (Renekton)"
    },
    {
      "time": "18:32",
      "team": null,
      "description": "kozakSyla (Lux) has targeted the Power Flower (33%)"
    }
  ],
  "messages": [
    {
      "time": "13:08",
      "channel": "party",
      "player": "piwkobb",
      "champion": "Yone",
      "text": "Cho'Gath Heartsteel - 20 charges"
    },
    {
      "time": "16:53",
      "channel": "all",
      "player": "piwkobb",
      "champion": "Yone",
      "text": "hello this is all chat msg"
    },
    {
      "time": "17:34",
      "channel": "team",
      "player": "kozakSyla",
      "champion": "Lux",
      "text": "hi in team chat"
    }
  ],
  "system": [
    {
      "text": "Type /help for a list of commands"
    }
  ]
}
```
</details>

---

## 📁 Project Structure
```
src/
 ├── grammar.pest      # Full grammar definition
 ├── lib.rs            # Core parsing logic
 ├── main.rs           # CLI interface
tests/
 ├── grammar_rules_spec.rs
 ├── json_integration_spec.rs
README.md
Cargo.toml
Cargo.lock
lol_chat_example.txt   # Example log file
.gitignore
```
---

## 📜 License
MIT License.
Project created as part of the National University of Kyiv-Mohyla Academy Rust course.

Created by Bohdan Tarverdiiev

---


```
  /\_/\      
 ( o.o )   -- hello there :)
  > ^ <      
```