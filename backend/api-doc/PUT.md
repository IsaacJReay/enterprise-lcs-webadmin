# PUT API Documentations

> ### - /private/api/settings/dns/domain_name/rename/{zone}/{old_domain_name}
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
>>      "newDomainName"
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

> ### - /private/api/settings/dns/edit/{zone}/{domain_name}
>>
>> | Header Parameter | Data Type |
>> | ---------------- | --------- |
>> | Authorization    | `String`  |
>> 
>>
>> | Parameter        | Data Type                 |
>> | ---------------- | ------------------------- |
>> | zone             | `internal` or `external`  |
>> | domain_name      | Example: `koompi.net`     |
>>
>>  - Body
>> ```json
>> {
>>   "old_record": {
>>     "subdomain_name": "ns",
>>     "dns_type": "A",
>>     "address": "10.100.100.1"
>>   },
>>   "new_record": {
>>     "subdomain_name": "ns",
>>     "dns_type": "A",
>>     "address": "10.1.2.2"
>>   },
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

> ### - /private/api/settings/dns/sort/{zone}/{domain_name}
>>
>> | Header Parameter | Data Type |
>> | ---------------- | --------- |
>> | Authorization    | `String`  |
>> 
>>
>> | Parameter        | Data Type                 |
>> | ---------------- | ------------------------- |
>> | zone             | `internal` or `external`  |
>> | domain_name      | Example: `koompi.net`     |
>>
>>  - Body
>> ```json
>> [
>>     {
>>       "subdomain_name": " ",
>>       "dns_type": "A",
>>       "address": "192.168.1.7"
>>     },
>>     {
>>       "subdomain_name": " ",
>>       "dns_type": "NS",
>>       "address": "john"
>>     },
>>     {
>>       "subdomain_name": "@",
>>       "dns_type": "NS",
>>       "address": "koompi.app."
>>     },
>>     {
>>       "subdomain_name": "john",
>>       "dns_type": "A",
>>       "address": "192.168.1.8"
>>     },
>>     {
>>       "subdomain_name": "dev",
>>       "dns_type": "CNAME",
>>       "address": "john"
>>     }
>> ]  
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