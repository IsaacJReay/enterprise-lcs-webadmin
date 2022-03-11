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
>> ```json
>>  {
>>    "operation_status": "Success",
>>    "reason": ""
>>  }
>> ``` 
>> - Response 500 
>> ```json
>>  {
>>    "operation_status": "Failed",
>>    "reason": "actual_reason_goes_here"
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