use rustop::opts;
use std::env;
use std::path::Path;
use std::fs;

mod db;

fn init_rookmark() -> bool{

    let rookmark_home = db::get_rookmark_folder();
    
    if Path::new(&rookmark_home).is_dir() {
        println!("rookmark_home exists !");
    } else {        
        match fs::create_dir(Path::new(&rookmark_home)) {
            Ok(()) => println!("Folder Created"),
            Err(ex) => {
                println!("{}", ex);
                return false;
            },
        }
    }

    db::init_db();
    return true;
}


fn main(){
    let (args, _rest) = opts! {
        synopsis "a Simple filesystem Bookmark ";
        opt add:Option<String>, desc:"Add a Bookmark on current path position. Es. rookmark -a <name>";
        opt delete:Option<String>, desc:"Delete a Bookmark. Es. rookmark -d <name>";
        opt list:bool, desc:"List Bookmarks";
        opt init:bool, desc:"Init Bookmarks DB";
        param file:Option<String>, desc:"Return the path roorkmarked by name.\n es. \n cd $(rookmark <name>)";
    }.parse_or_exit();

    let path = env::current_dir().unwrap();

    if args.init {
        if db::is_dbinited() {
            println!("DB already present.\nPlease remove it manually:\n"); 
            println!("rm -rf {}\n", &db::get_db_fullname()); 
            std::process::exit(0);
        } else{
            init_rookmark();
            std::process::exit(0);        
        }
    }

    if !db::is_dbinited() {
        println!("DB not inited.\n Please launch with -i options"); 
        std::process::exit(0);
    }

    if let Some(add) = args.add 
    { 
        db::add_db(&add,&path.to_str().unwrap().to_string());
        println!("add bookmark: {} on {}", add,path.display()); 
        std::process::exit(0);
    }

    if let Some(delete) = args.delete 
    { 
        println!("delete bookmark: {}", delete);
        db::delete_db(&delete);
        std::process::exit(0);
    }

    if args.list 
    { 
        println!("list bookmarks"); 
        println!("{}" , db::list_db());
        std::process::exit(0);
    }


    if let Some(file) = args.file 
    { 
        println!("{}", db::get_db(&file)); 
        std::process::exit(0);        
    }

    println!("Missing arguments: rookmark -h for option available"); 
    std::process::exit(0);
}
