# Rust-API-alerting

RustAPI alerting is tool design to send logs to differents sources.

### How to use it?
**Discord** 
Using a Simple Curl this will triggered the Alert directly into Discord.


```bash
curl -d "data=Alerting from POST cURL" -X POST http://localhost:8000/discord
```

Don't forget to use the Discord token using the ENV variable `export DISCORD_TOKEN=Secret_token`

![image](https://user-images.githubusercontent.com/23560242/198521827-c75e53a7-7be3-4e1b-b1a0-163f768ca923.png)

Then this will create a message to the `channelID` hardcoded.

![image](https://user-images.githubusercontent.com/23560242/198522741-ef1d1962-ca2b-46af-8fd9-517a33beaf2f.png)
