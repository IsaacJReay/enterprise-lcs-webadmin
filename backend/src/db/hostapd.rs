use wildmatch::WildMatch;
use crate::{ 
    db::read_lines, 
};


pub fn read_hostapd() -> (String, bool, u8, String, u8, String, bool, bool){
    let mut ssid: String = String::new();
    let mut hide_ssid: bool = 0 != 0;
    let mut channel: u8 = 0;
    let mut wpa: u8 = 0;
    let mut hw_mode: String = String::new();
    let mut passphrase: String = String::new();
    let mut hw_n_mode: bool = 0 != 0;
    let mut qos: bool = 0 != 0;

    if let Ok(file) = read_lines("/etc/hostapd/hostapd.conf") {
        // Consumes the iterator, returns an (Optional) String
        for lines in file {
            if let Ok(line) = lines {
                if WildMatch::new("ssid=*").matches(&line){
                    let split = &line.split("=");
                    let line_vec = split.clone().collect::<Vec<&str>>();
                    ssid = format!("{}",line_vec[1]);
                }
                else if WildMatch::new("ignore_broadcast_ssid=*").matches(&line) {
                    let split = &line.split("=");
                    let line_vec = split.clone().collect::<Vec<&str>>();
                    hide_ssid = format!("{}",line_vec[1]).to_owned().as_str().parse::<u8>().unwrap() != 0;
                }
                else if WildMatch::new("wpa=*").matches(&line) {
                    let split = &line.split("=");
                    let line_vec = split.clone().collect::<Vec<&str>>();
                    wpa = format!("{}",line_vec[1]).to_owned().as_str().parse::<u8>().unwrap();
                }
                else if WildMatch::new("hw_mode=*").matches(&line){
                    let split = &line.split("=");
                    let line_vec = split.clone().collect::<Vec<&str>>();
                    hw_mode = format!("{}",line_vec[1]);
                }
                else if WildMatch::new("channel=*").matches(&line){
                    let split = &line.split("=");
                    let line_vec = split.clone().collect::<Vec<&str>>();
                    channel = format!("{}",line_vec[1]).to_owned().as_str().parse::<u8>().unwrap();
                }
                else if WildMatch::new("wpa_passphrase=*").matches(&line){
                    let split = &line.split("=");
                    let line_vec = split.clone().collect::<Vec<&str>>();
                    passphrase = format!("{}",line_vec[1]);
                }
                else if WildMatch::new("ieee80211n=*").matches(&line){
                    let split = &line.split("=");
                    let line_vec = split.clone().collect::<Vec<&str>>();
                    hw_n_mode = format!("{}",line_vec[1]).to_owned().as_str().parse::<u8>().unwrap() != 0;
                }
                else if WildMatch::new("wmm_enabled=*").matches(&line){
                    let split = &line.split("=");
                    let line_vec = split.clone().collect::<Vec<&str>>();
                    qos = format!("{}",line_vec[1]).to_owned().as_str().parse::<u8>().unwrap() != 0;
                }
            }
        }
    }
    (ssid, hide_ssid, wpa, hw_mode, channel,passphrase, hw_n_mode, qos)
}