#![no_main]
use libfuzzer_sys::fuzz_target;

use std::io::{Read, Write};
use websocket::ClientBuilder;

fuzz_target!(|data: &[u8]| {
	let client = ClientBuilder::new("ws://127.0.0.1:2998")
		.unwrap()
		.add_protocol("rust-websocket")
		.key([0; 16])
		.connect_on(FuzzInput(data));

	if let Ok(mut client) = client {
		let _ = client.recv_message();
	}
});

struct FuzzInput<'a>(&'a [u8]);

impl Read for FuzzInput<'_> {
	fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
		self.0.read(buf)
	}
}

impl Write for FuzzInput<'_> {
	fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
		Ok(buf.len())
	}

	fn flush(&mut self) -> std::io::Result<()> {
		Ok(())
	}
}
