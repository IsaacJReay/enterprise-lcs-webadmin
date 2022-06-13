# Delete API Documentations

> ### - /private/api/settings/dns/delete/{zone}/{domain_name}
>>
>> | Header Parameter | Data Type |
>> | ---------------- | --------- |
>> | Authorization    | `String`  |
>> 
>> | Parameter        | Data Type                 |
>> | ---------------- | ------------------------- |
>> | zone             | `internal` or `external`  |
>> | domain_name      | Example: `koompi.com`     |
>>
>>  - Body
>> ```json
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

> ### - /private/api/settings/dns/delete/{zone}/{domain_name}/subdomain
>>
>> | Header Parameter | Data Type |
>> | ---------------- | --------- |
>> | Authorization    | `String`  |
>> 
>> | Parameter        | Data Type                 |
>> | ---------------- | ------------------------- |
>> | zone             | `internal` or `external`  |
>> | domain_name      | Example: `koompi.com`     |
>>
>>  - Body
>> ```json
>> {
>>     "subdomain_name": "",
>>     "dns_type": "A",
>>     "address": "10.100.100.1"
>> }
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


> ### - /private/api/settings/storage/device/deletion
>>
>> | Header Parameter | Data Type |
>> | ---------------- | --------- |
>> | Authorization     | `String` |
>> 
>>  - Body
>> ```json
>>  {
>>     "drive_partuuid": "kmp",
>>     "selected_filedir": [ "isaac/qwe/sjdf.txt", "john/aaaa", "john/text.txt" ]
>> }
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
