# Stupid Telegram bot for going

It helps us to be more precise when discussing an opportunity to go by keeping the discussion in agenda.
With it we won't forgot to mention important details anymore.

## Running

Use proxychains4 in some countries:

``` bash
TELEGRAM_BOT_TOKEN=your_bot_token proxychains4 cargo run
```

For production build use `./build.sh` then take the binary in `./target/release/stupid_bot`.
