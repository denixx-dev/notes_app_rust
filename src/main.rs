use std::{collections::HashMap, io};
use std::fs::OpenOptions;
use std::io::Write;

mod notes_storage;


fn main() {
    let mut inp_index = String::new();
    let mut inp_string = String::new();
    
    println!("Enter index and string: ");

    let _ = io::stdin().read_line(&mut inp_index);
    let _ = io::stdin().read_line(&mut inp_string);
       
    let mut index: i32 = inp_index.trim().parse().expect("Error in parsing index");
    let mut string: String = inp_string.trim().parse().expect("Error in parsing string");
    
    let mut notes = notes_storage::NotesStorer {map: HashMap::new()};

    let mut do_continue: String = String::new();
    
    notes.add_to_map(index, string);

    println!("Do you want to continue adding values? (y) or (n)");

    let _ = io::stdin().read_line(&mut do_continue);
    do_continue = do_continue.trim().parse().expect("Error in parsing do_continue");

    // let mut do_continue_2: String = String::new();

    while (do_continue == "y") {
        let mut inp_index_2 = String::new();
        let mut inp_string_2 = String::new(); 

        println!("Enter index and string: ");
       
        let _ = io::stdin().read_line(&mut inp_index_2);
        let _ = io::stdin().read_line(&mut inp_string_2);
        
        let mut index_2: i32 = inp_index_2.trim().parse().expect("Error in parsing index");
        let mut string_2: String = inp_string_2.trim().parse().expect("Error in parsing string");

        notes.add_to_map(index_2, string_2);

        println!("Do you want to continue adding values? (y) or (n)");
        do_continue.clear(); // Очищаем переменную перед новым вводом
        io::stdin().read_line(&mut do_continue).expect("Error in parsing do_continue");
        do_continue = do_continue.trim().to_string();
    }

    notes.print_map();

    notes.write_map_to_file();



}


// use std::fs::OpenOptions;
// use std::io::Write;

// fn main() {
//     let mut file = OpenOptions::new()
//         .append(true)
//         .create(true)
//         .open("file.txt")
//         .unwrap();
//     if let Err(e) = writeln!(file, "Additional line1") {
//         eprintln!("Couldn't write to file: {}", e);
//     }
// }