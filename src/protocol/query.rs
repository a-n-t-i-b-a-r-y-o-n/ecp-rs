use std::collections::HashMap;

#[allow(dead_code)]
pub enum Get {
    ActiveApp,
    ActiveTvChannel,
    ActiveTvInput,
    AudioDevice,
    AudioSetting,
    AudioSettings,
    AvSyncOffset,
    DeviceInfo,
    InstalledApps,
    MediaPlayer,
    QueryAppIcon { channel_id: i32 },
    Screensavers,
    TexteditState,
    Themes,
    TvChannels,
    VoiceServiceInfo,
    WarmStandby,
}

impl Get {
    /// Get the subject for this message
    pub fn subject(&self) -> &str {
        match self {
            Get::ActiveApp => "query-active-app",
            Get::ActiveTvChannel => "query-tv-active-channel",
            Get::ActiveTvInput => "query-tv-active-input",
            Get::AudioDevice => "query-audio-device",
            Get::AudioSetting => "query-audio-setting",
            Get::AudioSettings => "query-audio-settings",
            Get::AvSyncOffset => "query-av-sync-offset",
            Get::DeviceInfo => "query-device-info",
            Get::InstalledApps => "query-apps",
            Get::MediaPlayer => "query-media-player",
            Get::QueryAppIcon { .. } => "query-icon",
            Get::Screensavers => "query-screensavers",
            Get::TexteditState => "query-textedit-state",
            Get::Themes => "query-themes",
            Get::TvChannels => "query-tv-channels-ex",
            Get::VoiceServiceInfo => "query-info-for-voice-service",
            Get::WarmStandby => "query-warm-standby",
        }
    }

    /// Get any params this message might have
    pub fn params(&self) -> Option<HashMap<String, String>> {
        match self {
            Get::QueryAppIcon { channel_id } => {
                let mut map = HashMap::new();
                map.insert(String::from("param-channel-id"), format!("{}", channel_id));
                Some(map)
            }
            _ => None
        }
    }
}