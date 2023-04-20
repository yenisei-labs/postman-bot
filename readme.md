# Postman-bot

A simple bot for getting a telegram chat ID.

## Configuration

In order to configure the bot, you can use environment variables.

- `TELOXIDE_TOKEN` - Access token for the bot, required.
- `WEBHOOK_ADDR` - Address for the webhook that will be sent to the telegram. If the variable is not specified, long pooling will be used.
- `WEBHOOK_PORT` - The port on which the server will be available if the webhook address is specified. 8080 by default.
