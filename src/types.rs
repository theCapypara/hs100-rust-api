#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SystemGetSysinfo {
    pub err_code: i64,
    pub sw_ver: String,
    pub hw_ver: String,
    #[serde(rename = "type")]
    pub hw_type: String,
    pub model: String,
    pub mac: String,
    #[serde(rename = "deviceId")]
    pub device_id: String,
    #[serde(rename = "hwId")]
    pub hw_id: String,
    #[serde(rename = "fwId")]
    pub fw_id: String,
    #[serde(rename = "oemId")]
    pub oem_id: String,
    pub alias: String,
    pub dev_name: String,
    pub icon_hash: String,
    pub relay_state: i64,
    pub on_time: i64,
    pub active_mode: String,
    pub feature: String,
    pub updating: i64,
    pub rssi: i64,
    pub led_off: i64,
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct System {
    pub get_sysinfo: SystemGetSysinfo,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmeterGetRealtime {
    pub current: f64,
    pub voltage: f64,
    pub power: f64,
    pub total: f64,
    pub err_code: i64,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmeterGetVgainIgain {
    pub vgain: i64,
    pub igain: i64,
    pub err_code: i64,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Emeter {
    pub get_realtime: Option<EmeterGetRealtime>,
    pub get_daystat: Option<EmeterGetDaystat>,
    pub get_vgain_igain: Option<EmeterGetVgainIgain>,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmeterGetDaystatDayList {
    pub year: i64,
    pub month: i64,
    pub day: i64,
    pub energy: f64,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmeterGetDaystat {
    pub day_list: Vec<EmeterGetDaystatDayList>,
    pub err_code: i64,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlugInfo {
    pub system: Option<System>,
    pub emeter: Option<Emeter>,
}