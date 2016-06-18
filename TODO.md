Output values as a summary.

option to post results to an endpoint as JSON (e.g. keen.io)

Daemonize server?
have client perform at regular intervals
Daemonize client

## Code
enum for commands
flexible size of buffer size, encode it as an int in the send so receiver can detect it

## Data structure sent
- success / fail
- ping latency
- Upload speed
- Download speed
- Name of SSID done on
- BSSID number
- date & time measurement performed (UTC)
- client IP

Every minute do a check of status and ping a server, record latency (current speed Tx/Rx) if available?
- record the connection mechanism (ethernet, wifi, etc), your IP, SSID used 

Maybe not test upload/download speed every minute to reduce workload.
Obviously only test those if ping works

Every hour, send the data for the last hour (60 data points) to the central server
- it can find your location from the IP you send from and report you online and where


## Device Idea
As part of a simple Pi device with buttons
- connect to net using WPS
- start/stop monitoring
and leds:
- on network
- flash when monitoring

## Stats gathering
Be able to then graph response time, speed and using lost packets, calculate downtime etc.

By organising multiple people’s records by SSID, you can build up an image of quality of an SSID across multiple people.

Geolookup of where the measurement was done either by:
- geo coordinates in the measurement sent
- lookup of the SSID in some service
- IP lookup of the IP it came from

—> maybe some overall global quality measure
