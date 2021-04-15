# Endpoint Detection and Response (EDR) Event Simulator

Endpoint Detection and Response (EDR) agents are responsible to capturing and reporting various telemetry events including:
 * Process creation
 * File creation| modification| and deletion
 * Network Activity

## Quick Start

`cargo run examples/windows_input.csv`

## Full Setup

```csv
cargo build
cargo test
carge run examples/windows_input.csv`
```


## Usage

The application requires a csv-style input file to be passed as the primary argument.

Optional flags allows for specifying a deliminator and a specific output file.

* `-d`| `--deliminator <character>` specify the single character to use as a deliminator for the input file. Default is `,`
* `-o`| `--outfile <file_path>` specify where a log of activities should be written to. Default is `log.csv`

#### Example
`edr_generator.exe input.csv -d ; --outfile output.csv`

***
### Input File Format
The input file is a csv-style file that provides instructions on how the application should process commands. The following commands are supported:
 * `process` start a new child process. (All processed are garbage collected at end of run)
 * `new_file` creates a new file (Only if the file does not already exist)
 * `mod_file` modifies a file by appending a `\0` null byte to the end of the file
 * `delete_file` deletes a file
 * `connect` establishes a TCP/IP connection to a specified host
 * `connect_self` establishes a loopback connection to 
 * `pause` pauses for a specified number of milliseconds

#### Below are the expected commands and their required arguments

| Command     | Arg1 | Arg2 | Arg 3|
| ----------- | ----------- | ----------- | ----------- |
| process | path to process  | optional arguments...
| new_file   | path to file  |
| mod_file   | path to file  |
| delete_file   | path to file  |
| connect | destination IP address | destination port | message |
| connection_self | message
| pause | time (in milliseconds)

#### Example format (for more| see the example folder)
```csv
process,C:\Users\x24\Downloads\nmap-7.91-win32.zip\nmap-7.91\nmap.exe,-lvp 220
new_file,test.txt
mod_file,test3.txt
pause,2000
connect_self,hello world
```
**Note: The CSV file should not have headers.**
***


### Output File Format
The output file is also a csv-style output that captures information what events happened along with useful information for traceability with the EDR tools.

|TYPE|timestamp|username|process name|process command|PID|activity|file_path|source_addr|source_port|dest_addr|dest_port|bytes_sent|protocol|
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | 
| new_process | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |  |  |  |  |  |  |  | 
| new_file | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |  |  |  |  |  |  | 
| mod_file | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |  |  |  |  |  |  | 
| delete_file | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |  |  |  |  |  |  | 
| connect | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |  | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | 
| connection_self | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |  | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | 

Errors are also logged to the output file and only record the timestamp of the error and the error message.

#### Example output file format
```csv
t,timestamp,username,proc_name,proc_cmd,proc_id,activity,file_path,source_addr,source_port,dest_addr,dest_port,bytes_sent,protocol
Information,1618465748,user1,msedge.exe,C:\Program Files (x86)\Microsoft\Edge\Application\msedge.exe msn.com,34808,New Process,,,,,,,
Information,2021-04-15T05:49:10.108096+00:00,user1,EDR-Generator.exe,target\debug\EDR-Generator.exe help,35440,New File,\\?\C:\Users\x24\CLionProjects\EDR\EDR-Generator\test.txt,,,,,,
Information,2021-04-15T05:49:10.108293800+00:00,user1,EDR-Generator.exe,target\debug\EDR-Generator.exe help,35440,Modify File,\\?\C:\Users\x24\CLionProjects\EDR\EDR-Generator\test.txt,,,,,,
Information,2021-04-15T05:49:10.108511800+00:00,user1,EDR-Generator.exe,target\debug\EDR-Generator.exe help,35440,Delete File,\\?\C:\Users\x24\CLionProjects\EDR\EDR-Generator\test.txt,,,,,,
Information,2021-04-15T05:49:10.109806100+00:00,user1,EDR-Generator.exe,target\debug\EDR-Generator.exe help,35440,Network Connection,,127.0.0.1,12712,127.0.0.1,12711,11,TCP/IP
Error,2021-04-15T05:49:10.110008700+00:00,Test Error: This is a sample error
```

