# League of Legends chat parser

**Brief Description:**  
`LoLCP` is a parser for **League of Legends** chat logs.  
At this stage, it focuses on extracting timestamps from text lines using a simple formal grammar implemented with `pest`.

---

***At the current stage, the parser only handles timestamp recognition.***

---

## Technical Description

The goal of this project is to eventually allow users to upload a plain text chat log and receive structured **JSON output** that contains (based on the provided log context):

- all **players** in the match,
- which **champions** they played,
- which **team** achieved specific objectives (like jungle monsters or towers),
- **kill** statistics (who killed whom and how many times, with and without gold streaks),
- and all **chat messages** grouped by player or team,

— all tied together by their corresponding **timestamps**.

---

## Parsing Process

Each chat line follows a semi-structured format, for example:
```txt
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
We focus on detecting the **time mark** at the beginning of the line.

---

## Grammar Rule

Defined using the `pest` parser generator:

```pest
time = { ASCII_DIGIT ~ ASCII_DIGIT ~ ":" ~ ASCII_DIGIT ~ ASCII_DIGIT }
```
Example valid timestamps:
```txt
00:42
02:24
18:32
```