# discord_bot_rs

A stupid and *blazingly fast* (not really) Discord bot written in Rust

**Disclaimer:** If you are going to use this bot keep in mind that it currently supports only one server.

## Usage
If you decided to use this stupid here's how you can do it.

First you need to edit the `config.rs` file:

```rust
// Replace these values with your channel ids
pub const GUILD_ID: u64 = 0;
pub const COMPLETE_CHANNEL_ID: u64 = 0;
pub const DALL_E_CHANNEL_ID: u64 = 0;
pub const EDIT_CHANNEL_ID: u64 = 0;
// --snip--
```

Then you can run the bot like this:

```bash
OPENAI_KEY="Your OpenAI API key goes here" DISCORD_TOKEN="Your Discord bot's token goes here" cargo run
```

After this just hope everything goes well and the bot is working.
