# PUT API Documentations

> ### - /private/api/settings/dns/domain_name/rename/{zone}/{old_domain_name}/new_domain_name
>>
>> | Header Parameter | Data Type |
>> | ---------------- | --------- |
>> | Authorization    | `String`  |
>> 
>>
>> | Parameter        | Data Type                 |
>> | ---------------- | ------------------------- |
>> | zone             | `internal` or `external`  |
>> | old_domain_name  | Example: `koompi.com`     |
>> | new_domain_name  | Example: `koompi.net`     |
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