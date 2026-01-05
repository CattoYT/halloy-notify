# discord_webhook

Settings regarding Discord Webhook integration

- [discord_webhook](#discord_webhook)
  - [Configuration](#configuration)
    - [webhook](#webhook)
    - [user_id](#user_id)
    - [urgency_level](#urgency_level)
    - [spam_for_interview](#spam_for_interview)    

## Configuration

### webhook

The URL of the webhook

```toml
# Type: string
# Values: any string
# Default: not set
#
# Note: To get a webhook, open the channel settings -> Integrations -> New webhook -> Copy url

[discord_webhook]
webhook = "https://discord.com/api/webhooks/..."
```

### user_id

The User ID of the Discord user that you want to ping on updates.

```toml
# Type: string
# Values: any valid Discord UID as a string
# Default: not set

[discord_webhook]
user_id = "696969696969696969" # laugh please
```        

### urgency_level

The minimum 'Urgency' for a Discord ping to be sent out. 

| Urgency | Allowed events |
| ------- | -------------- |
| 0       | Nothing        |
| 1       | All events     |
| 2       | Highlights, Netsplits, Your interview |
| 3       | Netsplits, Your interview |


```toml
# Type: i8
# Values: valid integer between 1-3 inclusive
# Default: 1

[discord_webhook]
urgency_level = 1
```

### spam_for_interview

Whether to spam ping in case of your interview. This will spam 5x times.

```toml
# Type: bool
# Values: true, false
# Default: true

[discord_webhook]
spam_for_interview = true
```
