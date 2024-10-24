use dope_db::storage;

fn main() {
    storage::create_namespace("data/stuff/main".to_owned());
    storage::create_namespace("data/stuff/other".to_owned());
    match storage::insert("data/stuff/other".to_owned(), "test".to_owned(), "some value".to_owned()) {
        Err(e) => {
            eprintln!("Something went wrong when inserting stuff, {e}");
        },
        Ok(()) => {
            println!("good insert");
        }
    };
    match storage::get("data/stuff/other".to_owned(), "test") {
        Ok(v) => {
            let val = v.unwrap();
            println!("Got value back: {val}")
        },
        Err(e) => {
            eprintln!("Something went terribly wrong, {e}");
        }
    }
}
