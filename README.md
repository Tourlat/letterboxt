# letterbot

![bot log][logo_bot_letterboxd]


A simple discord bot for scraping and extracting datas from [Letterboxd](https://letterboxd.com/) pages.

As of now, the bot can extract the following information from a Letterboxd page:
- release year
- director
- rating
- synopsis
- genres

## Foreword

This bot is a personal project and is not intended for public use. It is a simple bot that I made for my own personal use and to learn how to use the [serenity](https://docs.rs/serenity/latest/serenity/) crate (github [link](https://github.com/serenity-rs/serenity)). The bot is not hosted on any server and is only run locally on my machine.

This is a work in progress and I will add more features / fix issues in the future.

Be careful when using this tools, movie names must be written exactly as they are on Letterboxd, otherwise the application won't be able to extract the information.
Also, the application will stop working if the html structure of the Letterboxd page changes.

## Usage

Howerver, if you want to use this bot, you can follow these steps:


1. Clone the repository
2. Install the required dependencies with `cargo build`
3. Create a `.env` file in the root of the project and add your discord bot token in it:
```
DISCORD_TOKEN=your_discord_bot_token
```
4. Run the bot with `cargo run`

You should create a discord bot and add it to your server. You can follow the instructions [here](https://discord.com/developers/docs/intro).


