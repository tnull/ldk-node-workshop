use ldk_node::bitcoin::secp256k1::PublicKey;
use ldk_node::bitcoin::Network;
use ldk_node::lightning_invoice::Invoice;
use ldk_node::{Builder, Config, NetAddress};
use std::str::FromStr;

const ESPLORA_SERVER_URL: &str = "http://ldk-node.tnull.de:3002";
const FAUCET_NODE_ID: &str = "031ee65dc5aca3f6f4c23408014f3554e52ac5c49080b42c1b1d0535ecb636b308";
const FAUCET_ADDR: &str = "ldk-node.tnull.de:9736";

fn main() {
	// Welcome! Please run through the the following steps.
	// "..." marks where you'll need to add code.

	// Setup Config
	let mut config = Config::default();
	config.network = Network::Regtest;

	// Configure Esplora URL)
	//
	// ...

	// Setup Builder from config and build() node
	//
	// ...

	// Start LDK Node
	//
	// ...

	// Get a new funding address and have it funded via the faucet
	//
	// ...
	//

	// Open channel to our node (see details above)
	//
	// ...

	//==============================================
	// We're now waiting for the channel to be confirmed:
	match node.wait_next_event() {
		ldk_node::Event::ChannelPending { channel_id, counterparty_node_id, .. } => println!(
			"New channel with {} pending confirmation: {:?}",
			counterparty_node_id, channel_id
		),
		e => println!("Unexpected event: {:?}", e),
	}
	node.event_handled();

	// Wait for 6 blocks (a 15 secs)
	std::thread::sleep(std::time::Duration::from_secs(90));
	node.sync_wallets().unwrap();

	match node.wait_next_event() {
		ldk_node::Event::ChannelReady { channel_id, .. } => {
			println!("Channel {:?} is ready to be used!", channel_id)
		}
		e => println!("Unexpected event: {:?}", e),
	}
	node.event_handled();
	//==============================================

	// Parse invoice (Invoice::from_str)
	//
	// ...
	//

	// Pay invoice
	//
	// ...

	//==============================================
	// Wait for the payment to be successful.
	match node.wait_next_event() {
		ldk_node::Event::PaymentSuccessful { payment_hash } => {
			println!("Payment with hash {:?} successful!", payment_hash)
		}
		e => println!("Unexpected event: {:?}", e),
	}
	node.event_handled();
	node.stop().unwrap();
}

fn pause() {
	use std::io;
	use std::io::prelude::*;

	let mut stdin = io::stdin();
	let mut stdout = io::stdout();

	// We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
	write!(stdout, "Press any key to continue...").unwrap();
	stdout.flush().unwrap();

	// Read a single byte and discard
	let _ = stdin.read(&mut [0u8]).unwrap();
}
