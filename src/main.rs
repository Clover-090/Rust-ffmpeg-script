use walkdir::WalkDir;
use std::io;
use std::process::Command;


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
    let program = "ffmpeg";
    let mut extention2 = String::new();



    loop{

        println!("Enter the directory you want to search\n");

        //Read user input
        io::stdin().read_line(&mut searchdir).expect("Failed to read line");

        println!("Enter the file extention to search for\n");

        io::stdin().read_line(&mut extention1).expect("Failed to read line");

        println!("Enter file extention to convert to, make sure not put a period infront of it, you want mp3 not .mp3\n");
        
        io::stdin().read_line(&mut extention2).expect("Failed to read line");

        //Remove newline from the end
        newlineremover(&mut searchdir);
        newlineremover(&mut extention1);
        newlineremover(&mut extention2);
        
        //Currently prints the path of all the files that are under the given directory and all its sub directories, skips files that it doesn not have perms for
        for file in WalkDir::new(&searchdir).into_iter().filter_map(|e| e.ok()) {
            //Infile for the FFMPEG command
            let inpath = file.path();

            let mut out_path = inpath.to_path_buf();
            
            out_path.set_extension(&extention2);


            let ffmpeg = Command::new(program).arg("-i").arg(inpath).arg(out_path).output();

            //makes sure what its looking at is a file and turns the filename to a string and checks its extention
            if file.metadata().expect("Can't read file metadata").is_file() && file.file_name().to_string_lossy().ends_with(&extention1){
                println!("status: {:?}", ffmpeg)
            }
        }
    }
}
