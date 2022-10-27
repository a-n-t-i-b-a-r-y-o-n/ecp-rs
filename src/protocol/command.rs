use std::collections::HashMap;

#[allow(dead_code)]
pub enum Set {
    AudioOutput {
        audio_output: String, sas_min_version: i32, sas_max_version: i32,
        guid: String, sas_ip_address: String, sas_port: String, app_build: String,
    },
    AudioSetting { id: String, value: String },
    CaptureScreen,
    LaunchApp { channel_id: i32 },
    PressKey { key: String },
    ResetAudioSettings { scope: String },
    ScreenSaver { channel_id: i32 },
    TexteditText {
        textedit_id: String, text: String,
        selection_start: i32, selection_end: i32
    },
}

#[allow(dead_code)]
impl Set {
    /// Get the subject for this message
    pub fn subject(&self) -> &str {
        match self {
            Set::AudioOutput { .. } => "set-audio-output",
            Set::AudioSetting { .. } => "set-audio-setting",
            Set::CaptureScreen => "capture-screen",
            Set::LaunchApp { .. } => "launch",
            Set::PressKey { .. } => "key-press",
            Set::ResetAudioSettings { .. } => "reset-audio-settings",
            Set::ScreenSaver { .. } => "set-screensaver",
            Set::TexteditText { .. } => "set-textedit-text"
        }
    }

    /// Get any params this message might have
    pub fn params(&self) -> Option<HashMap<String, String>> {
        let mut map = HashMap::new();
        match self {
            Set::AudioOutput {
                audio_output, sas_min_version, sas_max_version,
                guid, sas_ip_address, sas_port, app_build
            } => {
                map.insert(String::from("param-audio-output"), String::from(audio_output));
                map.insert(String::from("param-sas-min-version"), format!("{}", sas_min_version));
                map.insert(String::from("param-sas-max-version"), format!("{}", sas_max_version));
                map.insert(String::from("param-guid"), String::from(guid));
                map.insert(String::from("param-sas-ip-address"), String::from(sas_ip_address));
                map.insert(String::from("param-sas-port"), String::from(sas_port));
                map.insert(String::from("param-app-build"), String::from(app_build));
                Some(map)
            }
            Set::AudioSetting { id, value } => {
                map.insert(String::from("param-id"), String::from(id));
                map.insert(String::from("param-value"), String::from(value));
                Some(map)
            }
            Set::LaunchApp { channel_id } => {
                map.insert(String::from("param-channel-id"), format!("{}", channel_id));
                Some(map)
            }
            Set::PressKey { key } => {
                map.insert(String::from("param-key"), String::from(key));
                Some(map)
            }
            Set::ResetAudioSettings { scope } => {
                map.insert(String::from("param-scope"), String::from(scope));
                Some(map)
            }
            Set::ScreenSaver { channel_id } => {
                map.insert(String::from("param-channel-id"), format!("{}", channel_id));
                Some(map)
            }
            Set::TexteditText {
                textedit_id, text, selection_start, selection_end
            } => {
                map.insert(String::from("param-textedit-id"), String::from(textedit_id));
                map.insert(String::from("param-text"), String::from(text));
                map.insert(String::from("param-selection-start"), format!("{}", selection_start));
                map.insert(String::from("param-selection-end"), format!("{}", selection_end));
                Some(map)
            }
            _ => None
        }
    }
}