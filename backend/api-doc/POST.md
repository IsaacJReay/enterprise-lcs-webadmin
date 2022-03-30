# POST API Documentations

> ### - /private/api/user/login
>>
>> | Header Parameter | Data Type |
>> | ---------------- | --------- |
>> | `NULL`           |  `NULL`   |
>> 
>>  - Body
>> ```json
>>  {
>>     "username": "alarm",
>>     "password": "alarm"
>>  }
>> ```
>>
>> - Response 200 
>> ```
>> token_goes_here
>> ```
>> - Response 401 
>> ```
>> wrong_username_or_password
>> ```

> ### - /private/api/user/password
>>
>> | Header Parameter | Data Type |
>> | ---------------- | --------- |
>> | Authorization    | `String`  |
>> 
>>  - Body
>> ```json
>>  {
>>   "old_password": "alarm",
>>   "new_password": "123"
>>  }
>> ```
>>
>> - Response 200 
>> ```
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
>> **Note:** New Password must not be the same as Old Password

> ### - /private/api/settings/reset
>>
>> | Header Parameter | Data Type |
>> | ---------------- | --------- |
>> | Authorization    | `String` |
>> 
>>  - Body
>> ```json
>> ```
>>
>> - Response 200 
>> ```text
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
>> **Note:** Everything goes to default factory configuration

> ### - /private/api/settings/hostapd
>>
>> | Header Parameter | Data Type |
>> | ---------------- | --------- |
>> | Authorization     | `String` |
>> 
>>  - Body
>> ```json
>>  {
>>   "ssid": "Sala",
>>   "hide_ssid": false,
>>   "hw_mode": "g",
>>   "channel": 11,
>>   "wpa": 2,
>>   "passphrase": "Koompi-Onelab",
>>   "hw_n_mode": true,
>>   "qos": true
>>  }
>> ```
>>
>> - Response 200 
>> ```text
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
>> **Note:** `WPA` can only be 1 or 2; `Channel` can only range from 1 to 14

> ### - /private/api/settings/wirelessnetwork
>>
>> | Header Parameter | Data Type |
>> | ---------------- | --------- |
>> | Authorization     | `String` |
>> 
>>  - Body
>> ```json
>>  {
>>   "router_ip": "10.100.100.1",
>>   "netmask": "255.255.255.0",
>>   "range_start": "10.100.100.1",
>>   "range_end": "10.100.100.254",
>>   "dns": "1.1.1.1 8.8.8.8",
>>   "default_lease": "1800",
>>   "max_lease": "7200",
>>   "timezone": "Asia/Phnom_Penh"
>>  }
>> ```
>>
>> - Response 200 
>> ```text
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

> ### - /private/api/settings/wirednetwork/static
>>
>> | Header Parameter | Data Type |
>> | ---------------- | --------- |
>> | Authorization     | `String` |
>> 
>>  - Body
>> ```json
>>  {
>>   "internet_ip": "192.168.1.10",
>>   "netmask": "255.255.255.0",
>>   "gateway": "192.168.1.1",
>>   "dns": "1.1.1.1 8.8.8.8 1.0.0.1 8.8.4.4"
>>  }
>> ```
>>
>> - Response 200 
>> ```text
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

> ### - /private/api/settings/wirednetwork/dynamic
>>
>> | Header Parameter | Data Type |
>> | ---------------- | --------- |
>> | Authorization     | `String` |
>> 
>>  - Body
>> ```
>> ```
>>
>> - Response 200 
>> ```text
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

> ### - /private/api/settings/dns/new/{zone}
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
>>
>> for change status or add new domain_name
>> ```json
>>   {
>>     "domain_name": "koompi.com",
>>     "status": true,
>>     "zone_record": null
>>   }
>> ```
>>  
>> for add new subdomain_name records
>> 
>> ```json
>>   {
>>     "domain_name": "koompi.com",
>>     "status": true,
>>     "zone_record": [
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
>>   }
>> ```
>>
>> - Response 200 
>> ```text
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
>> **Note:** `subdomain_name` must be a subdomain without its main website name after and is also website form; `dns_type` must only be A, AAAA, CNAME, MX 10, PTR, CAA, SRV, TXT, SOA.

> ### - /private/api/settings/time/timedate
>>
>> | Header Parameter | Data Type |
>> | ---------------- | --------- |
>> | Authorization     | `String` |
>> 
>>  - Body
>> ```json
>>  {
>>      "time": "18:17:16",
>>      "date": "2012-10-30"
>>  }
>> ```
>>
>> - Response 200 
>> ```text
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
>> **Note:** `time` and `date` must follow the format shown in body.

> ### - /private/api/settings/time/timezone
>>
>> | Header Parameter | Data Type |
>> | ---------------- | --------- |
>> | Authorization     | `String` |
>> 
>>  - Body
>> ```json
>>  {
>>    "timezone": "Asia/Phnom_Penh",
>>  }
>> ```
>>
>> - Response 200 
>> ```text
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
>> **Note:** `timezone` must be a valid timezone from website https://en.wikipedia.org/wiki/List_of_tz_database_time_zones;


> ### - /private/api/settings/storage/device/copy_or_move
>>
>> | Header Parameter | Data Type |
>> | ---------------- | --------- |
>> | Authorization     | `String` |
>> 
>>  - Body
>> ```json
>> {
>>     "operation": "move",
>>     "source_uuid": "kmp",
>>     "source_items": ["www/test1", "testt"],
>>     "destination_uuid": "62AA-7652",
>>     "items_destination": ""
>> }
>> ```
>> Or,
>> ```json
>> {
>>     "operation": "copy",
>>     "source_uuid": "kmp",
>>     "source_items": ["www/test1", "testt"],
>>     "destination_uuid": "62AA-7652",
>>     "items_destination": ""
>> }
>> ```
>>
>> - Response 200 
>> ```text
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


> ### - /private/api/settings/storage/device/directory/creation
>>
>> | Header Parameter | Data Type |
>> | ---------------- | --------- |
>> | Authorization     | `String` |
>> 
>>  - Body
>> ```json
>>  {
>>     "directory_name": "newfolder1234",
>>     "parent_directory": "",
>>     "drive_partuuid": "3EB010966E49278D"
>>  }
>> ```
>>
>> - Response 200 
>> ```text
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
> ### - /private/api/settings/storage/device/unmount
>>
>> | Header Parameter | Data Type |
>> | ---------------- | --------- |
>> | Authorization     | `String` |
>> 
>>  - Body
>> ```json
>>  {
>>    "drive_partuuid": "7df645f6-2912-4f6f-bc80-6e823e75e8cb"
>>  }
>> ```
>>
>> - Response 200 
>> ```text
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

> ### - /private/api/settings/update/update
>>
>> | Header Parameter | Data Type |
>> | ---------------- | --------- |
>> | Authorization     | `String` |
>> 
>>  - Body
>> ```json
>>  {
>>    "id": "1",
>>    "sys_update": true
>>  }
>> ```
>>
>> - Response 200 
>> ```text
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