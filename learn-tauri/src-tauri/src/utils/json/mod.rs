pub mod json_tool {
    use regex::Regex;

    #[derive(serde::Serialize)]
    pub enum SerializationResult {
        Str(String),
        Bool(bool),
    }

    #[tauri::command]
    pub fn serialization(json: &str) -> SerializationResult {
        // 检查json参数中是否包含{或者[或者}或者]，如果不包含，直接返回false
        let re = Regex::new(r"[{}\[\]]").unwrap();
        if !re.is_match(json) {
            return SerializationResult::Bool(false);
        }
        // 检查是否是合法的json，是的话返回json，否则返回false
        let result = serde_json::from_str::<serde_json::Value>(json);
        match result {
            Ok(res) => {
                let pretty_json = serde_json::to_string_pretty(&res).unwrap_or_default();
                SerializationResult::Str(pretty_json)
            }
            Err(_) => SerializationResult::Bool(false),
        }
    }
}
