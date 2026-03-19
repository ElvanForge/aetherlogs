#[derive(Debug)]

pub struct EvtcHeader {
	pub magic: String,
	pub version: String,
	pub boss_id: u16,
}