extern crate shelf;

#[test]
fn create_db()
{
	let db = shelf::DB::open(&Path::new("test_db"));
}

#[test]
fn put()
{
	let db = shelf::DB::open(&Path::new("test_db"));
	match db.put("key", "value")
	{
		Err(err) => panic!("{}", err.desc),
		_ => ()
	}
}