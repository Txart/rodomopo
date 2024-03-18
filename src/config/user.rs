use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct UserConfig {
    // User-defined config in the config.yaml
    pub minimum_work_block_duration_in_minutes: i64,
    pub daily_work_goal_in_minutes: i64,
}

impl UserConfig {
    pub fn new() -> Self {
        let config_filepath = &crate::config::internal::CONFIG.config_filepath;
        let _yaml_user_config: String =
            fs::read_to_string(config_filepath).expect("problem reading file!");
        println!("yaml user config {}", _yaml_user_config);

        let user_config_values: UserConfig = serde_yaml::from_str(&_yaml_user_config)
            .expect("Error deserializing config.yaml file. Check the syntax is correct!");

        Self {
            minimum_work_block_duration_in_minutes: user_config_values
                .minimum_work_block_duration_in_minutes,
            daily_work_goal_in_minutes: user_config_values.daily_work_goal_in_minutes,
        }
    }

    pub fn serialize_default_user_config_contents() -> String {
        let default_config = UserConfig {
            minimum_work_block_duration_in_minutes: 25,
            daily_work_goal_in_minutes: 180,
        };
        serde_yaml::to_string(&default_config).expect("Could not serialize default user config!")
    }
}

#[test]
fn serialize_and_deserialize() {
    let default_config = UserConfig {
        minimum_work_block_duration_in_minutes: 25,
        daily_work_goal_in_minutes: 180,
    };

    let yaml_user_config = UserConfig::serialize_default_user_config_contents();
    let s: String = String::from(
        "minimum_work_block_duration_in_minutes: 25\ndaily_work_goal_in_minutes: 180\n",
    );
    assert_eq!(yaml_user_config, s);

    let deserialized_user_config: UserConfig =
        serde_yaml::from_str(&yaml_user_config).expect("Error deserializing yaml!");
    assert_eq!(deserialized_user_config, default_config);
}
