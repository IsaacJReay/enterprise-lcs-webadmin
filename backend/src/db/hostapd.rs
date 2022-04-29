use crate::config;

pub fn read_hostapd() -> (String, bool, u8, String, u8, String, bool, bool) {
    let config = config::read_file("/etc/hostapd/hostapd.conf");
    let mut ssid: String = String::new();
    let mut hide_ssid: bool = false;
    let mut channel: u8 = 0;
    let mut wpa: u8 = 0;
    let mut hw_mode: String = String::new();
    let mut passphrase: String = String::new();
    let mut hw_n_mode: bool = false;
    let mut qos: bool = false;

    config.lines().for_each(|each_line| {
        if each_line.contains("ignore_broadcast_ssid=") {
            hide_ssid = each_line.split("=").last().unwrap().parse::<u8>().unwrap() != 0;
        } else if each_line.contains("ssid=") {
            ssid = each_line.split("=").last().unwrap().to_string();
        } else if each_line.contains("wpa=") {
            wpa = each_line.split("=").last().unwrap().parse::<u8>().unwrap();
        } else if each_line.contains("hw_mode=") {
            hw_mode = each_line.split("=").last().unwrap().to_string();
        } else if each_line.contains("channel=") {
            channel = each_line.split("=").last().unwrap().parse::<u8>().unwrap();
        } else if each_line.contains("wpa_passphrase=") {
            passphrase = each_line.split("=").last().unwrap().to_string();
        } else if each_line.contains("ieee80211n=") {
            hw_n_mode = each_line.split("=").last().unwrap().parse::<u8>().unwrap() != 0;
        } else if each_line.contains("wmm_enabled=") {
            qos = each_line.split("=").last().unwrap().parse::<u8>().unwrap() != 0;
        }
    });

    (
        ssid, hide_ssid, wpa, hw_mode, channel, passphrase, hw_n_mode, qos,
    )
}
