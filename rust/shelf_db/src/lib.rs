extern crate rustc;

use std::io::{File, FilePermission, fs};
use std::result::Result;
use rustc::util::sha2::{Sha256, Digest};

pub struct Error
{
	pub desc: &'static str,
}

pub struct DB
{
	path: Path,
}

impl DB
{
	pub fn open(path: &Path) -> DB
	{
		DB{path: path.clone()}
	}
	pub fn put(&self, key: &str, value: &str) -> Result<(), Error>
	{
		let mut digest = Sha256::new();
		digest.input_str(key);
		let dir = self.create_path_(digest.result_str());
		match File::create(&dir.join("key")).write(key.as_bytes())
		{
			Err(err) => return Err(Error{desc: err.desc}),
			_ => (),
		};
		match File::create(&dir.join("value")).write(value.as_bytes())
		{
			Err(err) => return Err(Error{desc: err.desc}),
			_ => (),
		};
		Ok(())
	}

	fn create_path_(&self, digest: String) -> Path
	{
		let dir = self.path.join(digest);
		fs::mkdir_recursive(&dir, FilePermission::all());
		dir
	}
}