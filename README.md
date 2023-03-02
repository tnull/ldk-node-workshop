# Workshop: Getting Started with LDK Node

Documentation: https://docs.rs/ldk-node

## Regtest URLs
- Faucet: http://ldk-node.tnull.de/
- Bitcoind: `ldk-node.tnull.de:18444`
- Esplora: `http://ldk-node.tnull.de:3002`
- Lightning: `031ee65dc5aca3f6f4c23408014f3554e52ac5c49080b42c1b1d0535ecb636b308@ldk-node.tnull.de:9736`

## Invoice Payment Challenge
0. Install Rust, clone this repo
1. Look through docs
2. Setup `Config` (configure Esplora URL)
3. Setup `Builder` from config, `build()` node
4. Start LDK Node
5. Get a new funding address
6. Have it funded via the faucet
7. Open channel to our node (see above)
8. Get your invoice (10k sats) from the faucet (clock starts ticking!!)
9. Parse invoice (`Invoice::from_str`)
9. Pay invoice
