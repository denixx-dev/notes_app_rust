use std::collections::HashMap;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;

// pub fn print_hello(string: &str){
//     println!("Hello, {}", string);
// }
pub struct NotesStorer{
    pub map: HashMap<i32, String>,
}

impl NotesStorer {
    pub fn add_to_map(&mut self, index: i32, string: String){
        self.map.insert(index, string);
    }

    pub fn print_map(&mut self) {
        for (key, value) in &self.map {
            println!("{}: {}", key, value);
        }
    }

    pub fn write_map_to_file(&mut self) -> std::io::Result<()>{
        // let mut create_file = File::create("test.txt")?;
        // create_file.write_all("".as_bytes())?;

        let mut f = OpenOptions::new()
            // .write(true)
            // .create(true)
            .append(true)
            .create(true)
            .open("test.txt")
            .unwrap();


        // match File::open("test.txt") {
        //     Ok(_) => f = File::options().append(true).open("test.txt")?,
        //     Err(error) => println!("Файл не существует"),
        // }

        
        // let mut string_to_write: String = String::new();

        for (key, value) in &self.map {
            // string_to_write = String::from("{key}: {value}");
            // writeln!(f, "{key}: {value}");
            if let Err(e) = writeln!(f, "{key}: {value}") {
                eprintln!("Couldn't write to file: {}", e);
            }
            // match f.write_all(format!("{}: {}\n", key, value).as_bytes()) {
            //     Ok(_) => println!(""),
            //     Err(error) => println!("error writing in a file"),
            // }
            
        }
        Ok(())
    }
}