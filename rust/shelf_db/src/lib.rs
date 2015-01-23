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
		self.create_root_path_();
		let mut digest = Sha256::new();
		digest.input_str(key);
		println!("{:?}", digest.result_str());
		let filename = self.path.join(key);
		match File::create(&filename).write(value.as_bytes())
		{
			Err(err) => Err(Error{desc: err.desc}),
			_ => Ok(()),
		}
	}

	fn create_root_path_(&self)
	{
		fs::mkdir_recursive(&self.path, FilePermission::all());
	}
}