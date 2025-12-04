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
    let mut search_dir = String::new();
    let mut extention1 = String::new();
    let program = "ffmpeg";
    let mut extention2 = String::new();



    loop{

        println!("Enter the directory you want to search\n");

        //Read user input
        io::stdin().read_line(&mut search_dir).expect("Failed to read line");

        println!("Enter the file extention to search for\n");

        io::stdin().read_line(&mut extention1).expect("Failed to read line");

        println!("Enter file extention to convert to, make sure not put a period infront of it, you want mp3 not .mp3\n");
        
        io::stdin().read_line(&mut extention2).expect("Failed to read line");

        //Remove newline from the end
        newlineremover(&mut search_dir);
        newlineremover(&mut extention1);
        newlineremover(&mut extention2);
        
        
        for file in WalkDir::new(&search_dir).into_iter().filter_map(|e| e.ok()) {

            //Infile for the FFMPEG command
            let in_path = file.path();

            //Converts in_path to a PathBuf so the extentsion can be changes and stores it in out_path
            let mut out_path = in_path.to_path_buf();

            //changes extension of out_path to the second specified extension
            out_path.set_extension(&extention2);

            //sets up the ffmpeg command to run
            let ffmpeg = Command::new(program).arg("-i").arg(in_path).arg(out_path).output();

            //makes sure what its looking at is a file and turns the filename to a string and checks its extention
            if file.metadata().expect("Can't read file metadata").is_file() && file.file_name().to_string_lossy().ends_with(&extention1){
                //runs the command and prints its output
                println!("status: {:?}", ffmpeg)
            }
        }
    }
}
