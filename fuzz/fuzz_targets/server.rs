#![no_main]
use libfuzzer_sys::fuzz_target;

use std::io::{Read, Write};
use websocket::server::upgrade::sync::IntoWs;

fuzz_target!(|data: &[u8]| {
	let _ = fuzz(data);
});

fn fuzz(data: &[u8]) -> Option<()> {
	let request = FuzzInput(data).into_ws().ok()?;

	let mut client = request.use_protocol("rust-websocket").accept().ok()?;

	let _ = client.recv_message();

	None
}

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
