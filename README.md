# Jellyfin Responder
This is a very simple rust program to listen for jellyfin discovery broadcasts and reply with a configured server address. Intended to point clients that use this variant of server discovery to a configured server address, that may be a jellyfin server in another broadcast domain, or may be any address on the internet. 

## Usage
Populate the environment as shown in `.env.sample` or pass the same values on the command line. or use on the command line as such: `jellyfin-responder <SERVER_ADDRESS> <SERVER_ID> <SERVER_NAME> [ENDPOINT_ADDRESS]`, these are also the environment variable names to set. 

Endpoint address is optional, all arguments are formatted as strings and just filled into JSON in the reply. At this time it is recommended to sniff the reply from your server and fill the values in from there.