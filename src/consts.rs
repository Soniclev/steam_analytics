pub const PERCENTILES: [(u8, f64); 5] =
    [(60, 0.60), (65, 0.65), (70, 0.70), (75, 0.75), (80, 0.80)];
pub const DESIRED_PERCENTILE: u8 = 60;

pub const VALVE_GAME_IDS: [u64; 7] = [
    440, // Team Fortress 2
    570, // Dota 2
    730, // Counter-Strike 2
    753, // Steam
    250820, // SteamVR
    583950, // Artifact
    1422450, // Deadlock
];


// define macro to generate events

macro_rules! define_event {
    ($date:expr, $name:expr) => {
        (
            concat!($date, " 00:00"),
            concat!($date, " 23:59"),
            $name,
        )
    };
}

macro_rules! define_event_duration {
    ($date1:expr, $date2:expr, $name:expr) => {
        (
            concat!($date1, " 00:00"),
            concat!($date2, " 23:59"),
            $name,
        )
    };
}



pub const EVENTS: [(&str, &str, &str); 16] =
        [
            define_event!("2024-10-03", "The Armory and full 7-day tradeban for market/trading"),
            define_event!("2023-09-27", "CS 2 Release"),
            define_event!("2023-03-22", "CS 2 Anouncement"),
            define_event!("2021-06-03", "CS:GO prime became only buyable"),
            define_event!("2018-12-06", "Free to Play"),
            define_event!("2018-03-30", "CS:GO 7-days items ban introduced"),

            // https://counterstrike.fandom.com/wiki/Operations
            define_event_duration!("2021-09-21", "2022-02-21", "Operation Riptide"),
            define_event_duration!("2020-12-03", "2021-05-03", "Operation Broken Fang"),
            define_event_duration!("2019-11-18", "2020-03-31", "Operation Shattered Web"),
            define_event_duration!("2017-05-23", "2017-11-13", "Operation Hydra"),
            define_event_duration!("2016-02-17", "2016-07-15", "Operation Wildfire"),
            define_event_duration!("2015-05-26", "2015-10-01", "Operation Bloodhound"),
            define_event_duration!("2014-11-11", "2015-03-31", "Operation Vanguard"),
            define_event_duration!("2014-07-01", "2014-10-02", "Operation Breakout"),
            define_event_duration!("2014-02-20", "2014-06-11", "Operation Phoenix"),
            define_event_duration!("2013-04-25", "2013-08-31", "Operation Payback"),
        ];
