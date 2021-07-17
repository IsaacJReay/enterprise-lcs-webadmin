# PUT API Documentations

> ### - /private/api/settings/dns/zone_record/deletion
>>
>> | Header Parameter | Data Type |
>> | ---------------- | --------- |
>> | Authorization    | `String`  |
>> 
>>  - Body
>> ```json
>>  {
>>   "id": {
>>     "id": "1"
>>   },
>>   "foreign_key": {
>>     "foreign_key": "1",
>>   }
>>  }
>> ```
>>
>> - Response 200 
>> ```json
>>  {
>>    "operation": "Success",
>>    "reason": ""
>>  }
>> ``` 
>> - Response 500 
>> ```json
>>  {
>>    "operation": "Failed",
>>    "reason": "actual_reason_goes_here"
>>  }
>> ``` 
>> - Response 410 
>> ```json
>>  {
>>    "operation": "Failed",
>>    "reason": "token-timeout"
>>  }
>> ```
>> - Response 401 
>> ```json
>>  {
>>    "operation": "Failed",
>>    "reason": "incorrect-token"
>>  }
>> ```
>> ```json
>>  {
>>    "operation": "Failed",
>>    "reason": "missing-token"
>>  }
>> ```

> ### - /private/api/settings/dns/domain_name/deletion
>>
>> | Header Parameter | Data Type |
>> | ---------------- | --------- |
>> | Authorization    | `String`  |
>> 
>>  - Body
>> ```json
>>  {
>>   "id": {
>>     "id": "1"
>>   }
>>  }
>> ```
>>
>> - Response 200 
>> ```json
>>  {
>>    "operation": "Success",
>>    "reason": ""
>>  }
>> ``` 
>> - Response 500 
>> ```json
>>  {
>>    "operation": "Failed",
>>    "reason": "actual_reason_goes_here"
>>  }
>> ``` 
>> - Response 410 
>> ```json
>>  {
>>    "operation": "Failed",
>>    "reason": "token-timeout"
>>  }
>> ```
>> - Response 401 
>> ```json
>>  {
>>    "operation": "Failed",
>>    "reason": "incorrect-token"
>>  }
>> ```
>> ```json
>>  {
>>    "operation": "Failed",
>>    "reason": "missing-token"
>>  }
>> ```