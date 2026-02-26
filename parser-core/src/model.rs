#[repr(C, packed)]
pub struct EvtcHeader {
	pub magic: [u8;4],
	pub version: [u8; 8],
	pub revision: u8,
	pub boss_id: u16,
	pub unused: u8,
}

#[repr(C, packed)]
pub struct RawAgent {
	pub addr: u64,
	pub prof: u32,
	pub is_elite: u32,
	pub toughness: u16,
	pub concentration: u16,
	pub healing: u16,
	pub hitbox_width: u16,
	pub condition: u16,
	pub hitbox_height: u16,
	pub name: [u8; 64],
	pub padding: u32,
}