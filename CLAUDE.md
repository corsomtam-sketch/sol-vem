# VeM — Solana MEV Bot

## Project Overview
MEV (Maximal Extractable Value) bot for the Solana network. Forked from an upstream GitHub repo.

## ⚠️ BEFORE STARTING THE BOT — MANDATORY
**ALWAYS remind the user to turn on Proton VPN before running the bot.**
Never skip this. If the user is about to run the bot, the first thing to say is:
> "Have you turned on Proton VPN?"

---

## Standing Instructions

### GitHub Push Reminder
**Every 20 minutes: prompt the user to commit and push all changes to GitHub.**
- Remind with: "20 minutes have passed — please commit and push to GitHub."

### Conversation Compact Reminder
**Every 20 minutes: prompt the user to run `/compact` to compress the conversation.**
- Remind with: "20 minutes have passed — please run /compact to compress the conversation."

### Memory Logging
- Every significant discovery, change, iteration, decision, or incident must be logged in `/Users/danhurt/.claude/projects/-Users-danhurt-Projects-VeM/memory/` and indexed in `MEMORY.md`.
- Log proactively — do not wait to be asked.

## Workflow
- Read files before editing. Check existence before creating.
- Confirm before any destructive action.
- Do not commit code unless explicitly asked.

## Environment
- Solana network — mainnet/devnet TBD
- macOS Apple Silicon
