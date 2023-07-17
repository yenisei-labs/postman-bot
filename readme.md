# Postman-bot

A simple bot for getting a telegram chat ID.

- [Demo](https://t.me/yenisei_postman_bot)

## Configuration

In order to configure the bot, you can use environment variables.

- `TELOXIDE_TOKEN` - Access token for the bot, required.
- `WEBHOOK_ADDR` - Address for the webhook that will be sent to the telegram. If the variable is not specified, long pooling will be used.
- `WEBHOOK_PORT` - The port on which the server will be available if the webhook address is specified. 8080 by default.

## Docker-compose

Minimum configuration:
```yml
version: '3.8'

services:
  postman:
    image: ghcr.io/yenisei-labs/postman-bot
    environment:
      TELOXIDE_TOKEN: <your_secret_token>
    restart: unless-stopped
```

Configuration with webhook and traefik:
```yml
version: '3.8'

services:
  postman:
    image: ghcr.io/yenisei-labs/postman-bot
    environment:
      WEBHOOK_ADDR: https://domain.com/webhook
      TELOXIDE_TOKEN: <your_secret_token>
    labels:
      - "traefik.enable=true"
      - "traefik.http.routers.postman-bot.rule=host(`domain.com`) && PathPrefix(`/webhook`)"
      - "traefik.http.routers.postman-bot.tls=true"
      - "traefik.http.routers.postman-bot.tls.certresolver=letsencryptresolver"
    restart: unless-stopped
```
