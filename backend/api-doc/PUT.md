# PUT API Documentations

> ### - /private/api/settings/dns/status/update
>>
>> | Header Parameter | Data Type |
>> | ---------------- | --------- |
>> | Authorization    | `String`  |
>> 
>>  - Body
>> ```json
>>  {
>>     "id": "1",
>>     "status": true,
>>  }
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

> ### - /private/api/settings/dns/domain_name/update
>>
>> | Header Parameter | Data Type |
>> | ---------------- | --------- |
>> | Authorization    | `String`  |
>> 
>>  - Body
>> ```json
>>  {
>>   "foreign_key": {
>>     "foreign_key": "1"
>>   },
>>   "new_domain_name": "salala",
>>  }
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