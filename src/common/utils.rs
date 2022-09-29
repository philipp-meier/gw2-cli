use crate::v2::pvp::PvpStats;
use chrono::prelude::*;
use colored::Colorize;

/// Returns the formatted account age (years, days, hours, minutes, seconds).
pub fn get_account_age(timestamp: DateTime<Utc>) -> String {
    let mut seconds = (Utc::now()-timestamp).num_seconds() as usize;

    let years = seconds / 31_536_000;
    seconds = seconds - (years * 31_536_000);

    let days = seconds / 86_400;
    seconds = seconds - (days * 86_400);

    let hours = seconds / 3_600;
    seconds = seconds - (hours * 3_600);

    let minutes = seconds / 60;
    seconds = seconds - (minutes * 60);

    format!("{years}y {days}d {hours}h {minutes}m {seconds}s")
}

/// Formats the in-game currency (Gold, Silver, Copper).
pub fn format_coins(amount: i32) -> String {
    let gold = amount / 10_000;
    let silver = (amount - gold * 10_000) / 100;
    let copper = amount - gold * 10_000 - silver * 100;

    format!("{} Gold, {} Silver, {} Copper",
            gold.to_string().truecolor(212, 175, 55),
            silver.to_string().truecolor(150, 150, 150),
            copper.to_string().truecolor(97, 56, 23))
}

/// Formats the world-vs-world rank and title.
pub fn format_wvw_stats_row(wvw_rank: i32, title: String) -> String {
    format!("{} - {}", wvw_rank.to_string().blue(), title)
}

/// Formats the player-vs-player rank and title.
pub fn format_pvp_stats_row(stats: PvpStats, rank: String) -> String {
    let mut rollovers = String::from("");
    if stats.pvp_rank_rollovers > 0 {
        rollovers = format!(" ({})", stats.pvp_rank_rollovers);
    }
    format!("{}{} - {} (Wins: {}, Losses: {})", stats.pvp_rank.to_string().blue(), rollovers, rank,
            stats.aggregate.wins.to_string().green(), stats.aggregate.losses.to_string().red())
}

/// Returns the ASCII Guild Wars 2 logo.
pub fn get_ascii_logo_rows() -> Vec<&'static str> {
    vec!["        ~!^~!77!^               ",
         "     :YY5GGBPP#&#GJ^            ",
         "    :5GP#G#&P7~G&&&#J:          ",
         "  ~!P#P5#GBJ5? ~&&#&BY:         ",
         "  ?5?^:?75P    :B&&&G7:         ",
         "     ::  ~^   !PBBP?~:          ",
         "           :!?Y?~^  :     :^    ",
         "           ~^          :::      ",
         "      ^~~:      ^: :~7~         ",
         "   :?YYJ7:~7JYY?77??7^^^:       ",
         "  ^?P5J?7?5GGP5Y557!!~          ",
         " :?55PP55##BPGBBGGGJ!^^::  ^:   ",
         " !!7!!!!!YYJJJJJJ??7!~^:        "]
}