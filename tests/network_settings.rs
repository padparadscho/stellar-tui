//! Ensures active network selection updates correctly and ignores out-of-bounds indices.

use stellar_tui::settings::{Network, Settings};

#[test]
fn set_active_network_is_bounds_checked() {
    let mut settings = Settings {
        networks: vec![
            Network {
                name: "A".to_string(),
                endpoint: "https://a".to_string(),
            },
            Network {
                name: "B".to_string(),
                endpoint: "https://b".to_string(),
            },
        ],
        active_network: 0,
    };

    settings.set_active_network(1);
    assert_eq!(settings.active_network, 1);

    settings.set_active_network(99);
    assert_eq!(settings.active_network, 1);
}

#[test]
fn settings_deserializes_legacy_and_current_field_aliases() {
    let legacy = r#"{
        "networks": [{"name": "Legacy", "endpoint": "https://legacy"}],
        "active_network": 0
    }"#;
    let current = r#"{
        "profiles": [{"name": "Current", "endpoint": "https://current"}],
        "active_profile": 0
    }"#;

    let legacy_settings: Settings =
        serde_json::from_str(legacy).expect("legacy schema should deserialize");
    let current_settings: Settings =
        serde_json::from_str(current).expect("current schema should deserialize");

    assert_eq!(legacy_settings.networks[0].name, "Legacy");
    assert_eq!(legacy_settings.active_network, 0);
    assert_eq!(current_settings.networks[0].name, "Current");
    assert_eq!(current_settings.active_network, 0);
}
