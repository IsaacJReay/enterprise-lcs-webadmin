# GET API Documentations

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
>> ```text
>> isaac
>> ```
>> - Response 410 
>> ```text
>> Token expired or incorrect
>> ```
>> - Response 401 
>> ```text
>> Token invalid
>> ```
>>

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
>> - Response 410 
>> ```text
>> Token expired or incorrect
>> ```
>> - Response 401 
>> ```text
>> Token invalid
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
>> ```text
>> Token expired or incorrect
>> ```
>> - Response 401 
>> ```text
>> Token invalid
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
>> ```text
>> Token expired or incorrect
>> ```
>> - Response 401 
>> ```text
>> Token invalid
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
>> ```text
>> Token expired or incorrect
>> ```
>> - Response 401 
>> ```text
>> Token invalid
>> ```

> ### - /private/api/settings/dns/status/zone
>>
>> | Header Parameter | Data Type |
>> | --------- | --------- |
>> | Authorization | `String` |
>> 
>> | Parameter        | Data Type                 |
>> | ---------------- | ------------------------- |
>> | zone             | `internal` or `external`  |
>>
>> - Body
>> ```
>> ```
>> - Response 200 
>> ```json
>> [
>>   {
>>     "domain_name": "koompi.com",
>>     "status": true,
>>     "zone_record": [
>>       {
>>         "subdomain_name": "ns",
>>         "dns_type": "A",
>>         "address": "10.100.100.1"
>>       },
>>       {
>>         "subdomain_name": "sala",
>>         "dns_type": "CNAME",
>>         "address": "ns"
>>       },
>>       {
>>         "subdomain_name": "salabackend",
>>         "dns_type": "CNAME",
>>         "address": "ns"
>>       },
>>       {
>>         "subdomain_name": "rachel",
>>         "dns_type": "CNAME",
>>         "address": "ns"
>>       },
>>       {
>>         "subdomain_name": "admin",
>>         "dns_type": "CNAME",
>>         "address": "ns"
>>       },
>>       {
>>         "subdomain_name": "phet",
>>         "dns_type": "CNAME",
>>         "address": "ns"
>>       },
>>       {
>>         "subdomain_name": "w3",
>>         "dns_type": "CNAME",
>>         "address": "ns"
>>       },
>>       {
>>         "subdomain_name": "wikipedia",
>>         "dns_type": "CNAME",
>>         "address": "ns"
>>       },
>>       {
>>         "subdomain_name": "wiktionary",
>>         "dns_type": "CNAME",
>>         "address": "ns"
>>       },
>>       {
>>         "subdomain_name": "wikibook",
>>         "dns_type": "CNAME",
>>         "address": "ns"
>>       },
>>       {
>>         "subdomain_name": "test",
>>         "dns_type": "CAA",
>>         "address": "0 issue \"test.net\""
>>       },
>>       {
>>         "subdomain_name": "aaaa",
>>         "dns_type": "MX 10",
>>        "address": "mail.example.com."
>>       }
>>     ]
>>   },
>>   {
>>     "domain_name": "website1.local",
>>     "status": true,
>>     "zone_record": [
>>       {
>>         "subdomain_name": "ns01",
>>         "dns_type": "A",
>>         "address": "10.100.100.1"
>>       },
>>       {
>>         "subdomain_name": "ns01",
>>         "dns_type": "A",
>>         "address": "10.100.100.1"
>>       },
>>       {
>>         "subdomain_name": "ns01",
>>         "dns_type": "A",
>>         "address": "10.100.100.1"
>>       }
>>     ]
>>   }
>> ]
>> ```
>> - Response 500
>> ```text
>> actual_error_goes_here
>> ```
>> - Response 410 
>> ```text
>> Token expired or incorrect
>> ```
>> - Response 401 
>> ```text
>> Token invalid
>> ```
>>

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
>> - Response 500
>> ```text
>> actual_error_goes_here
>> ```
>> - Response 410 
>> ```text
>> Token expired or incorrect
>> ```
>> - Response 401 
>> ```text
>> Token invalid
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
>>  {
>>     "drive_label": "Local Content Storage",
>>     "drive_partuuid": {
>>       "drive_partuuid": "kmp"
>>     },
>>     "free_space": "3.3T",
>>     "total_space": "3.6T",
>>     "percentage": 4
>>   },
>>   {
>>     "drive_label": "Removeable Device",
>>     "drive_partuuid": {
>>       "drive_partuuid": "7df645f6-2912-4f6f-bc80-6e823e75e8cb"
>>     },
>>     "free_space": "3.7G",
>>     "total_space": "3.9G",
>>     "percentage": 1
>>   },
>>   {
>>     "drive_label": "Removeable Device",
>>     "drive_partuuid": {
>>       "drive_partuuid": "3EB7-DF9A"
>>     },
>>     "free_space": "4.0G",
>>     "total_space": "4.0G",
>>     "percentage": 1
>>   },
>>   {
>>     "drive_label": "Removeable Device",
>>     "drive_partuuid": {
>>        "drive_partuuid": "3FA1-D350"
>>      },
>>     "free_space": "4.0G",
>>     "total_space": "4.0G",
>>     "percentage": 1
>>   },
>>   {
>>     "drive_label": "Removeable Device",
>>     "drive_partuuid": {
>>       "drive_partuuid": "3EB010966E49278D"
>>     },
>>     "free_space": "2.8G",
>>     "total_space": "2.8G",
>>     "percentage": 1
>>   }
>> ]
>> ```
>> - Response 500
>> ```text
>> actual_error_goes_here
>> ```
>> - Response 410 
>> ```text
>> Token expired or incorrect
>> ```
>> - Response 401 
>> ```text
>> Token invalid
>> ```

> ### - /private/api/settings/storage/device/status/{drive_partuuid}
>>
>> | Header Parameter | Data Type |
>> | --------- | --------- |
>> | Authorization | `String` |
>> 
>> - Body
>> ```json
>> ```
>> - Response 200 
>> ```json
>> {
>>   "name": "Removeable Device",
>>   "meta": {
>>     "item_last_modify_date": "2021-11-25 06:00:43",
>>     "item_is_dir": true,
>>     "item_size": 32768
>>   },
>>   "children": [
>>     {
>>       "name": "231_1- Keynote Proficient Student's Book_2016 -192p_backup.pdf",
>>       "meta": {
>>         "item_last_modify_date": "2021-11-24 12:18:53",
>>         "item_is_dir": false,
>>         "item_size": 30881022
>>       },
>>       "children": []
>>     },
>>     {
>>       "name": "231_1- Keynote Proficient Student's Book_2016 -192p.pdf",
>>       "meta": {
>>         "item_last_modify_date": "2021-11-24 13:16:09",
>>         "item_is_dir": false,
>>         "item_size": 30921636
>>       },
>>       "children": []
>>     },
>>     {
>>       "name": "Pichponereay NGOR_E4.8_Reflection Paper_Do School Kills Creativity.docx",
>>       "meta": {
>>         "item_last_modify_date": "2021-11-20 16:54:18",
>>         "item_is_dir": false,
>>         "item_size": 2955518
>>       },
>>       "children": []
>>     },
>>     {
>>       "name": "PichponereayNGOR_E4.8_Quiz01.docx",
>>       "meta": {
>>         "item_last_modify_date": "2021-11-20 16:49:37",
>>         "item_is_dir": false,
>>         "item_size": 5870
>>       },
>>       "children": []
>>     },
>>     {
>>       "name": "PichponereayNGOR_E4.8_Quiz02.docx",
>>       "meta": {
>>         "item_last_modify_date": "2021-11-24 13:13:02",
>>         "item_is_dir": false,
>>         "item_size": 6149
>>       },
>>       "children": []
>>     },
>>     {
>>       "name": "PichponereayNGOR_E4.8_Quiz02.pdf",
>>       "meta": {
>>         "item_last_modify_date": "2021-11-24 13:13:08",
>>         "item_is_dir": false,
>>         "item_size": 29102
>>       },
>>       "children": []
>>     },
>>     {
>>       "name": "Unit-2-grammar-answer.pdf",
>>       "meta": {
>>         "item_last_modify_date": "2021-11-20 16:46:05",
>>         "item_is_dir": false,
>>         "item_size": 4346535
>>       },
>>       "children": []
>>     },
>>     {
>>       "name": "Word-Formation_backup.pdf",
>>       "meta": {
>>         "item_last_modify_date": "2021-11-20 16:46:44",
>>         "item_is_dir": false,
>>         "item_size": 553243
>>       },
>>       "children": []
>>     },
>>     {
>>       "name": "Word-Formation.pdf",
>>       "meta": {
>>         "item_last_modify_date": "2021-11-20 16:46:43",
>>         "item_is_dir": false,
>>         "item_size": 553239
>>       },
>>       "children": []
>>     }
>>   ]
>> }
>> ```
>> - Response 500
>> ```text
>> actual_error_goes_here
>> ```
>> - Response 410 
>> ```text
>> Token expired or incorrect
>> ```
>> - Response 401 
>> ```text
>> Token invalid
>> ```

> ### - /private/api/settings/update/status
>>
>> | Header Parameter | Data Type |
>> | ---------------- | --------- |
>> | Authorization    | `String`  |
>> 
>> - Body
>> ```json
>> ```
>> - Response 200 
>> ```json
>> [
>>   {
>>       "id": "1",
>>       "display_name": "First Patch Update",
>>       "update_size": 307,
>>       "sys_update": false,
>>       "status": "Installing",
>>   },
>>   {
>>       "id": "2",
>>       "display_name": "Second Patch Update",
>>       "update_size": 603467495,
>>       "sys_update": false,
>>       "status": "Downloading",
>>   },
>>   {
>>       "id": "3",
>>       "display_name": "Third Patch Update",
>>       "update_size": 157981621,
>>       "sys_update": false,
>>       "status": "New",
>>   }
>> ]
>> ```
>> - Response 500
>> ```text
>> actual_error_goes_here
>> ```
>> - Response 410 
>> ```text
>> Token expired or incorrect
>> ```
>> - Response 401 
>> ```text
>> Token invalid
>> ```
