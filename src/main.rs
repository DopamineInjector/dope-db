use dope_db::storage;

fn main() {
    match storage::insert("main".to_owned(), "test".to_owned(), "some value".to_owned()) {
        Err(e) => {
            eprintln!("Something went wrong when inserting stuff, {e}");
        },
        Ok(()) => {
            println!("good insert");
        }
    };
    match storage::get("main".to_owned(), "test") {
        Ok(v) => {
            let val = v.unwrap();
            println!("Got value back: {val}")
        },
        Err(e) => {
            eprintln!("Something went terribly wrong, {e}");
        }
    }
}
