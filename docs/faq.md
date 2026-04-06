# FAQ

## Why do I need `Arbitrum` network type?

It gives you typed Arbitrum transaction and receipt handling. Without it you lose protocol-specific typing.

## Why do some `arb_*` methods fail with `-32601`?

That usually means the method is not enabled in your Nitro node build or config.

## Is sequencer feed supported as a provider?

Not as a normal JSON-RPC provider. The sequencer feed is a separate protocol but feel free to use my other library for reading the feed.
