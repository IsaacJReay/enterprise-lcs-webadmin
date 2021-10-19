# GET API Documentations

> ### - /private/api/token/validation
>>
>> | Header Parameter | Data Type |
>> | --------- | --------- |
>> | Authorization | `String` |
>> 
>>  - Body
>> ```
>> ```
>>
>> - Response 200 
>> ```json
>>  {
>>    "operation_status": "Success",
>>    "reason": "token-valid"
>>  }
>> ``` 
>> - Response 410 
>> ```json
>>  {
>>    "operation_status": "Failed",
>>    "reason": "token-timeout"
>>  }
>> ```
>> - Response 401 
>> ```json
>>  {
>>    "operation_status": "Failed",
>>    "reason": "incorrect-token"
>>  }
>> ```
>> ```json
>>  {
>>    "operation_status": "Failed",
>>    "reason": "missing-token"
>>  }
>> ```


> ### - /private/api/user/query
>>
>> | Header Parameter | Data Type |
>> | --------- | --------- |
>> | Authorization | `String` |
>> 
>>  - Body
>> ```
>> ```
>>
>> - Response 200 
>> ```json
>>  {
>>    "Current User: isaac"
>>  }
>> ``` 
>> - Response 410 
>> ```json
>>  {
>>    "operation_status": "Failed",
>>    "reason": "token-timeout"
>>  }
>> ```
>> - Response 401 
>> ```json
>>  {
>>    "operation_status": "Failed",
>>    "reason": "incorrect-token"
>>  }
>> ```
>> ```json
>>  {
>>    "operation_status": "Failed",
>>    "reason": "missing-token"
>>  }
>> ```

> ### - /private/api/settings/status
>>
>> | Header Parameter | Data Type |
>> | --------- | --------- |
>> | Authorization | `String` |
>> 
>> - Body
>> ```
>> ```
>> - Response 200 
>> ```json
>> {
>>   "wan_macaddress": "dc:a6:32:bc:e0:c7",
>>   "wan_ip": "192.168.1.2",
>>   "wan_netmask": "255.255.255.0",
>>   "wan_gateway": "192.168.1.1",
>>   "wlan_macaddress": "dc:a6:32:bc:e0:c8",
>>   "wlan_ip": "10.100.100.1",
>>   "wlan_netmask": "255.255.255.0",
>>   "wlan_dns": "8.8.8.8 1.1.1.1 ",
>>   "wlan_ssid": "Sala",
>>   "wlan_hw_mode": "g",
>>   "wlan_channel": 6,
>>   "wlan_hw_n_mode": true,
>>   "wlan_qos": true
>> }
>> ```
>>
>> - Response 410 
>> ```json
>>  {
>>    "operation_status": "Failed",
>>    "reason": "token-timeout"
>>  }
>> ```
>> - Response 401 
>> ```json
>>  {
>>    "operation_status": "Failed",
>>    "reason": "incorrect-token"
>>  }
>> ```
>> ```json
>>  {
>>    "operation_status": "Failed",
>>    "reason": "missing-token"
>>  }
>> ```

> ### - /private/api/settings/wirelessnetwork/status
>>
>> | Header Parameter | Data Type |
>> | --------- | --------- |
>> | Authorization | `String` |
>> 
>> - Body
>> ```
>> ```
>> - Response 200 
>> ```json
>> {
>>   "router_ip": "10.100.100.1",
>>   "netmask": "255.255.255.0",
>>   "range_start": "10.100.100.1",
>>   "range_end": "10.100.100.255",
>>   "dns": "10.100.100.1 1.1.1.1",
>>   "default_lease": "1800",
>>   "max_lease": "7200",
>>   "timezone": "Asia/Phnom_Penh"
>> }
>> ```
>>
>> - Response 410 
>> ```json
>>  {
>>    "operation_status": "Failed",
>>    "reason": "token-timeout"
>>  }
>> ```
>> - Response 401 
>> ```json
>>  {
>>    "operation_status": "Failed",
>>    "reason": "incorrect-token"
>>  }
>> ```
>> ```json
>>  {
>>    "operation_status": "Failed",
>>    "reason": "missing-token"
>>  }
>> ```

> ### - /private/api/settings/wirednetwork/status
>>
>> | Header Parameter | Data Type |
>> | --------- | --------- |
>> | Authorization | `String` |
>> 
>> - Body
>> ```
>> ```
>> - Response 200 
>> ```json
>> {
>>  "dhcp": true,
>>  "wired_network_param": {
>>     "internet_ip": "192.168.1.2",
>>     "netmask": "255.255.255.0",
>>     "gateway": "192.168.1.1",
>>     "dns": " 1.1.1.1 8.8.8.8"
>>   }
>> }
>> ```
>>
>> - Response 410 
>> ```json
>>  {
>>    "operation_status": "Failed",
>>    "reason": "token-timeout"
>>  }
>> ```
>> - Response 401 
>> ```json
>>  {
>>    "operation_status": "Failed",
>>    "reason": "incorrect-token"
>>  }
>> ```
>> ```json
>>  {
>>    "operation_status": "Failed",
>>    "reason": "missing-token"
>>  }
>> ```

> ### - /private/api/settings/hostapd/status
>>
>> | Header Parameter | Data Type |
>> | --------- | --------- |
>> | Authorization | `String` |
>> 
>> - Body
>> ```
>> ```
>> - Response 200 
>> ```json
>> {
>>   "ssid": "Sala",
>>   "hide_ssid": false,
>>   "hw_mode": "g",
>>   "channel": 6,
>>   "wpa": 2,
>>   "passphrase": "Koompi-Onelab",
>>   "hw_n_mode": true,
>>   "qos": true
>> }
>> ```
>>
>> - Response 410 
>> ```json
>>  {
>>    "operation_status": "Failed",
>>    "reason": "token-timeout"
>>  }
>> ```
>> - Response 401 
>> ```json
>>  {
>>    "operation_status": "Failed",
>>    "reason": "incorrect-token"
>>  }
>> ```
>> ```json
>>  {
>>    "operation_status": "Failed",
>>    "reason": "missing-token"
>>  }
>> ```

> ### - /private/api/settings/dns/domain_name/status
>>
>> | Header Parameter | Data Type |
>> | --------- | --------- |
>> | Authorization | `String` |
>> 
>> - Body
>> ```
>> ```
>> - Response 200 
>> ```json
>> [
>>  {
>>    "id": "1",
>>    "domain_name": "website1.local",
>>    "status": true
>>  },
>>  {
>>    "id": "2",
>>    "domain_name": "website2.local",
>>    "status": false
>>  }
>> ]
>> ```
>>
>> - Response 410 
>> ```json
>>  {
>>    "operation_status": "Failed",
>>    "reason": "token-timeout"
>>  }
>> ```
>> - Response 401 
>> ```json
>>  {
>>    "operation_status": "Failed",
>>    "reason": "incorrect-token"
>>  }
>> ```
>> ```json
>>  {
>>    "operation_status": "Failed",
>>    "reason": "missing-token"
>>  }
>> ```

> ### - /private/api/settings/dns/zone_records/status
>>
>> | Header Parameter | Data Type |
>> | --------- | --------- |
>> | Authorization | `String` |
>> 
>> - Body
>> ```
>> {
>>   "foreign_key": "1"
>> }
>> ```
>> - Response 200 
>> ```json
>> {
>>   "domain_name": "website1.local",
>>   "status": false,
>>   "record_table": [
>>     {
>>       "id": "1",
>>       "subdomain_name": "ns01",
>>       "dns_type": "A",
>>       "address": "10.100.100.1",
>>       "foreign_key": "1"
>>     },
>>     {
>>       "id": "2",
>>       "subdomain_name": "ns01",
>>       "dns_type": "A",
>>       "address": "10.100.100.1",
>>       "foreign_key": "1"
>>     },
>>     {
>>       "id": "3",
>>       "subdomain_name": "ns01",
>>       "dns_type": "A",
>>       "address": "10.100.100.1",
>>       "foreign_key": "1"
>>     }
>>   ]
>> }
>> ```
>>
>> - Response 410 
>> ```json
>>  {
>>    "operation_status": "Failed",
>>    "reason": "token-timeout"
>>  }
>> ```
>> - Response 401 
>> ```json
>>  {
>>    "operation_status": "Failed",
>>    "reason": "incorrect-token"
>>  }
>> ```
>> ```json
>>  {
>>    "operation_status": "Failed",
>>    "reason": "missing-token"
>>  }
>> ```

> ### - /private/api/settings/time/status
>>
>> | Header Parameter | Data Type |
>> | --------- | --------- |
>> | Authorization | `String` |
>> 
>> - Body
>> ```
>> ```
>> - Response 200 
>> ```json
>> {
>>    "ntp_status": true,
>>    "timezone": "Asia/Phnom_Penh",
>>    "time": "18:09:09",
>>    "date": "2021-46-13"
>> }
>> ```
>>
>> - Response 410 
>> ```json
>>  {
>>    "operation_status": "Failed",
>>    "reason": "token-timeout"
>>  }
>> ```
>> - Response 401 
>> ```json
>>  {
>>    "operation_status": "Failed",
>>    "reason": "incorrect-token"
>>  }
>> ```
>> ```json
>>  {
>>    "operation_status": "Failed",
>>    "reason": "missing-token"
>>  }
>> ```

> ### - /private/api/settings/storage/status
>>
>> | Header Parameter | Data Type |
>> | --------- | --------- |
>> | Authorization | `String` |
>> 
>> - Body
>> ```
>> ```
>> - Response 200 
>> ```json
>> [
>>   {
>>     "drive_label": "Local Content Storage",
>>     "drive_partuuid": {
>>       "drive_partuuid": "kmp"
>>     },
>>     "free_space": "3.4T",
>>     "total_space": "3.6T"
>>   },
>>   {
>>     "drive_label": "Removeable Device",
>>     "drive_partuuid": {
>>       "drive_partuuid": "2021-10-06-15-49-55-00"
>>     },
>>     "free_space": "0",
>>     "total_space": "2.3G"
>>   },
>>   {
>>     "drive_label": "Removeable Device",
>>     "drive_partuuid": {
>>       "drive_partuuid": "5B0C-9920"
>>     },
>>     "free_space": "1.6M",
>>     "total_space": "50M"
>>   }
>> ]
>> ```
>>
>> - Response 410 
>> ```json
>>  {
>>    "operation_status": "Failed",
>>    "reason": "token-timeout"
>>  }
>> ```
>> - Response 401 
>> ```json
>>  {
>>    "operation_status": "Failed",
>>    "reason": "incorrect-token"
>>  }
>> ```
>> ```json
>>  {
>>    "operation_status": "Failed",
>>    "reason": "missing-token"
>>  }
>> ```

