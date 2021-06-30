# Enterprise-LocalContentServer-WebAdmin

## Backend
Backend is build with Rust, Bash, and Sqlite


There are 2 GET api
- get Status Page     -> */private/api/status*
- get Current User    -> *private/api/user/query*
---
There are 11 POST api
> ### -  /private/api/user/login
```
  - Use for Login
  - Must be done first before everything else
```
Example: 
```json
{
  "username": "alarm",
  "password": "alarm"
}
```

> ### -  /private/api/user/password
```
  - Use for Reset Login Password
  - Required Successful log in first
  - Old password cannot be the same as new password
```
Example: 
```json
{
  "old_password": "alarm",
  "new_password": "123"
}
```

> ### -  /private/api/settings/import
```
  - Use for upload back config after Backup
  - Required Successful log in first
  - Password Cannot exceed 16 characters
```
Example:
```json
{
  "password": "ABCabc123"
}
```
**Note:** Need upload File in form of Multiload

> ### -  /private/api/settings/export
```
  - Use for download backup config
  - Required Successful log in first
  - Filename cannot have space and cannot have any sign
  - Password Cannot exceed 16 characters
```
Example: 
```json
{
  "filename": "lcs_backup-2020";
  "password": "ABCabc123";
}
```

> ### -  /private/api/settings/reset
```
  - Use for reset all configuration
  - Requred Successful log in first
  - Doesn't have a body
```

> ### - /private/api/settings/hostapd
```
  - Use for setup wifi hotspot related setting
  - Required Successful log in first
  - WPA can only have 1 or 2
  - Channel can only range from 1 to 14
```
Example:
```json
{
  "ssid": "Sala",
  "hide_ssid": false,
  "hw_mode": "g",
  "channel": 11,
  "wpa": 2,
  "passphrase": "Koompi-Onelab",
  "hw_n_mode": true,
  "qos": true
}
```

> ### -  /private/api/settings/wirelessnetwork
```
  - Use for setup wifi network address related settings
  - Required Successful log in first
```
Example:
```json
{
  "router_ip": "10.100.100.1",
  "netmask": "255.255.255.0",
  "range_start": "10.100.100.1",
  "range_end": "10.100.100.254",
  "dns": "1.1.1.1 8.8.8.8",
  "default_lease": "1800",
  "max_lease": "7200",
  "timezone": "Asia/Phnom_Penh"
}
```

> ### -  /private/api/settings/wirednetwork/static
```
  - Use for setup WAN network with static address
  - Required Successful log in first
```
Example:
```json
{
  "internet_ip": "192.168.1.10",
  "netmask": "255.255.255.0",
  "gateway": "192.168.1.1",
  "dns": "1.1.1.1 8.8.8.8 1.0.0.1 8.8.4.4"
}
```
> ### -  /private/api/settings/wirednetwork/dynamic
```
  - Use for setup WAN network with dynamic address
  - Required Successful log in first
  - Doesn't have a body
```
