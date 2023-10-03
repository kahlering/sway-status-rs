pub const DEFAULT_CONFIG: &str = r#"{
    "status_bar_config": {
        "refresh_rate_ms": 1000,
        "test": 4
    },
    
    "modules": [
        {
            "name": "battery_module",
            "instance": "1",
            "type": 0
        },
        {
            "name": "date_and_time_module",
            "type": 1,
            "format": "%Y/%m/%d %T"
        }
    ]
}"#;