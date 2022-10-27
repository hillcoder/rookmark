use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};
use std::path::Path;

pub fn is_dbinited() -> bool {
    return Path::new(&get_rookmark_folder()).exists()
}

pub fn get_rookmark_folder() -> String{
    if let Some(m_path) = home::home_dir() {
        let r_path = m_path.as_path().display().to_string();
        return format!("{}/.rookmark", r_path);
    } else {
        return "".to_string();
    }
}

pub fn get_db_fullname() -> String{
    return format!("{}/rookmark.db", get_rookmark_folder());
}

pub fn init_db(){
    PickleDb::new(
        get_db_fullname(),
        PickleDbDumpPolicy::AutoDump,
        SerializationMethod::Bin,
    );
}

pub fn add_db(key : &String , value : &String) {
    let mut mdb = PickleDb::load(
        get_db_fullname(),
        PickleDbDumpPolicy::AutoDump,
        SerializationMethod::Bin,
    )
    .unwrap();

    mdb.set(key, value).unwrap();
}

pub fn list_db() -> String{
    let mdb = PickleDb::load(
        get_db_fullname(),
        PickleDbDumpPolicy::AutoDump,
        SerializationMethod::Bin,
    )
    .unwrap();

    let mut s_return = String::from("");

    for kv in mdb.iter() {
        s_return = format!("{} {} {:?} \n", &s_return,kv.get_key(),kv.get_value::<String>().unwrap()).to_string();
    }

    return s_return.to_string();
}

pub fn delete_db(key : &String) {
    let mut mdb = PickleDb::load(
        get_db_fullname(),
        PickleDbDumpPolicy::AutoDump,
        SerializationMethod::Bin,
    )
    .unwrap();

    mdb.rem(key).unwrap();
}

pub fn get_db(key : &String) -> String{
    let mdb = PickleDb::load(
        get_db_fullname(),
        PickleDbDumpPolicy::AutoDump,
        SerializationMethod::Bin,
    )
    .unwrap();

    return mdb.get::<String>(key).unwrap()
}