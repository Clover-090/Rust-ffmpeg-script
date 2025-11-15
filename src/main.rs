use walkdir::WalkDir;
use std::io;

//what it says on the tin, removes newlines from variables that store user inputs
fn newlineremover(x: &mut String) {
    if x.ends_with('\n') || x.ends_with('\r'){
        x.pop();
    }
}


fn main() {
    //Declairing variables for future use
    let mut searchdir = String::new();
    let mut extention1 = String::new();
    //let mut extention2 = String::new();

    loop{

        println!("Enter the directory you want to search\n");

        //Read user input
        io::stdin().read_line(&mut searchdir).expect("Failed to read line");

        println!("Enter the file extention to search for\n");

        io::stdin().read_line(&mut extention1).expect("Failed to read line");
        
        //Remove newline from the end
        newlineremover(&mut searchdir);
        newlineremover(&mut extention1);

        //Currently prints the path of all the files that are under the given directory and all its sub directories, skips files that it doesn not have perms for
        for file in WalkDir::new(&searchdir).into_iter().filter_map(|e| e.ok()) {
            //makes sure what its looking at is a file
            if file.metadata().unwrap().is_file(){
                //turns the filename to a string and checks its extention
                if file.file_name().to_string_lossy().ends_with(&extention1) {
                    println!("{}", file.path().display());
                }
            }
        }
    }
}
