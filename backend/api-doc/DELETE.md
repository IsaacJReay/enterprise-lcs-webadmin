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
<<<<<<< HEAD
>>  {
>>     "id": "1",
>>     "foreign_key": "1"
>>  }
=======
>>>>>>> ponereay
>> ```
>>
>> - Response 200 
>> ```json
>>  {
>>    "operation_status": "Success",
>>    "reason": ""
>>  }
>> ``` 
>> - Response 200 
>> ```json
>>  {
>>    "operation_status": "Failed",
>>    "reason": "actual_error_goes_here"
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

> ### - /private/api/settings/dns/delete/{zone}/{domain_name}/{subdomain_name}
>>
>> | Header Parameter | Data Type |
>> | ---------------- | --------- |
>> | Authorization    | `String`  |
>> 
>> | Parameter        | Data Type                 |
>> | ---------------- | ------------------------- |
>> | zone             | `internal` or `external`  |
>> | domain_name      | Example: `koompi.com`     |
>> | subdomain_name   | Example: `sala` or `wiki` |
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
>> - Response 200 
>> ```json
>>  {
>>    "operation_status": "Failed",
>>    "reason": "actual_error_goes_here"
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
