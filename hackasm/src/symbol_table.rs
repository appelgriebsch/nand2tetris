use std::collections::HashMap;

#[derive(Debug)]
pub(crate) struct SymbolTable {
	symbols: HashMap<String, u16>
}

impl SymbolTable {
	pub fn new() -> Self {
		let mut table = SymbolTable{ symbols: HashMap::new() };
		table.init();
		table
	}

	fn init(&mut self) {
		for reg_cnt in 0..16 {
			self.symbols.insert(format!("R{}", reg_cnt), reg_cnt);
		}
		self.symbols.insert("SCREEN".to_owned(), 16384);
		self.symbols.insert("KBD".to_owned(), 24576);
		self.symbols.insert("SP".to_owned(), 0);
		self.symbols.insert("LCL".to_owned(), 1);
		self.symbols.insert("ARG".to_owned(), 2);
		self.symbols.insert("THIS".to_owned(), 3);
		self.symbols.insert("THAT".to_owned(), 4);
	}

	pub fn get_symbol(&self, key: &str) -> Option<&u16> {
		self.symbols.get(key)
	}

	pub fn has_symbol(&self, key: &str) -> bool {
		self.symbols.contains_key(key)
	}

	pub fn register_symbol(&mut self, key: &str, value: u16) {
		self.symbols.insert(key.to_owned(), value);
	}

	pub fn print_symbols(&self) {
		for (s,v) in &self.symbols {
			println!("{:?}: {:?}", s, v);
		}
	}
}