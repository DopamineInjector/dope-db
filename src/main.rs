use dope_db::storage;

fn main() {
    storage::create_namespace("data/stuff/main".to_owned());
    storage::create_namespace("data/stuff/other".to_owned());
    let initial_sum = storage::get_checksum();
    println!("Initial checksum: ");
    println!("{:?}", initial_sum);
    match storage::insert("data/stuff/other".to_owned(), "test".to_owned(), "some value".to_owned()) {
        Err(e) => {
            eprintln!("Something went wrong when inserting stuff, {e}");
        },
        Ok(checksum) => {
            println!("good insert");
            println!("{:?}", checksum);
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
    match storage::insert("data/stuff/other".to_owned(), "test_other".to_owned(), "some value".to_owned()) {
        Err(e) => {
            eprintln!("Something went wrong when inserting stuff, {e}");
        },
        Ok(checksum) => {
            println!("good insert");
            println!("{:?}", checksum);
        }
    };
    match storage::insert("data/stuff/other".to_owned(), "test_other".to_owned(), "some value".to_owned()) {
        Err(e) => {
            eprintln!("Something went wrong when inserting stuff, {e}");
        },
        Ok(checksum) => {
            println!("good same insert");
            println!("{:?}", checksum);
        }
    };
    match storage::insert("data/stuff/other".to_owned(), "test_other".to_owned(), "some other value".to_owned()) {
        Err(e) => {
            eprintln!("Something went wrong when inserting stuff, {e}");
        },
        Ok(checksum) => {
            println!("good different insert");
            println!("{:?}", checksum);
        }
    };
    match storage::insert("data/stuff/main".to_owned(), "test_other".to_owned(), "some value".to_owned()) {
        Err(e) => {
            eprintln!("Something went wrong when inserting stuff, {e}");
        },
        Ok(checksum) => {
            println!("good insert");
            println!("{:?}", checksum);
        }
    };
}
