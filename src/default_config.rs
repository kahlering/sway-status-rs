pub const DEFAULT_CONFIG: &str = r#"[status_bar_config]
refresh_rate_ms= 1000

[modules]

[modules.battery]
name = "battery_module"
instance = "1"

[modules.date_and_time]
name = "date_and_time_module"
format = "%Y/%m/%d %T"

[modules.audio_volume]
name = "audio_volume_module""#;