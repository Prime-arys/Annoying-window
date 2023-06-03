
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process;
use std::env;
use std::thread;
use std::time;
use std::process::Command;
use winapi;
use winapi::um::wincon::GetConsoleWindow;



fn process_exists(pid: i32) -> bool {
    //verifie si le pid existe
    let output = Command::new("tasklist")
        .arg("/SVC")
        .arg("/FO")
        .arg("LIST")

        .output()
        .expect("failed to execute process")
        .stdout;

    //recherche le pid dans la liste des processus
    let output = String::from_utf8_lossy(&output);
    let output = output.to_string();
    let output = output.split("\r\n");
    let mut found = false;
    for line in output {
        if line.contains(&pid.to_string()) {
            found = true;
        }
    }
    println!("found: {}", found);
    found

}

fn main() {

    
    
    //write log file
    let mut file_lg = File::create("handler.log").expect("Unable to create file");

    //hide console window into background process
    //let _ = unsafe { winapi::um::wincon::FreeConsole() };
    let _ = unsafe { winapi::um::winuser::ShowWindow(GetConsoleWindow(), 0) };
    

    println!("Handler started\n");
    file_lg.write_all(b"Handler started\n").expect("Unable to write data");
    file_lg.write_all(b"console window hidden\n").expect("Unable to write data");

    //blank all file
    file_lg.set_len(0).expect("Unable to set file length");

    file_lg.write_all(b"Handler started\n").expect("Unable to write data");

    print!("waiting 3 seconds before starting...\n");
    file_lg.write_all(b"waiting 3 seconds before starting...\n").expect("Unable to write data");
    thread::sleep(time::Duration::from_secs(3));

    //prend un texte en argument
    let args: Vec<String> = env::args().collect();
    print!("args: {:?}\n", args);
    //let mut text = &args[1];

    let mut text = "tool.lock";

    

    
    

    //verifie si le fichier avec le meme nom existe deja dans le dossier courant
    let path = Path::new(text);
    let existe = path.exists();
    if existe == true {
        print!("does file exist?\n");
        file_lg.write_all(b"does file existe?").expect("Unable to write data");
        //lit le fichier et le stock dans une variable
        let mut file = File::open(text).expect("file not found");
        file_lg.write_all(b"file found\n").expect("Unable to write data");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("something went wrong reading the file");
        file_lg.write_all(b"file read\n").expect("Unable to write data");

        //affiche le contenu du fichier
        println!("With text:\n{}", contents);
        file_lg.write_all(b"file content : ").expect("Unable to write data");
        file_lg.write_all(contents.as_bytes()).expect("Unable to write data");

        //le contenu du fichier est un PID, on le converti en entier
        let pid = contents.parse::<i32>().unwrap();
        file_lg.write_all(b"PID : ").expect("Unable to write data");
        file_lg.write_all(pid.to_string().as_bytes()).expect("Unable to write data");

        let mut i = 0;      

        //dans une boucle, on verifie toutes les 30 secondes si le processus existe toujours et si le fichier existe toujours
        loop {
            print!("checking if process {} is running... ", pid);
            file_lg.write_all(b"checking if process ").expect("Unable to write data");
            file_lg.write_all(pid.to_string().as_bytes()).expect("Unable to write data");
            file_lg.write_all(b" is running... ").expect("Unable to write data");

            let process = process_exists(pid);
            let path = Path::new(text);
            let existe = path.exists();
            if (process == true && existe == true) || i ==0 {
                i += 1;
                println!("process {} is running", pid);
                file_lg.write_all(b"process ").expect("Unable to write data");
                file_lg.write_all(pid.to_string().as_bytes()).expect("Unable to write data");
                file_lg.write_all(b" is running\n").expect("Unable to write data");

                thread::sleep(time::Duration::from_secs(30));
            } else if existe == true && process == false {
                println!("process {} is not running", pid);
                file_lg.write_all(b"process ").expect("Unable to write data");
                file_lg.write_all(pid.to_string().as_bytes()).expect("Unable to write data");
                file_lg.write_all(b" is not running\n").expect("Unable to write data");

                //on supprime le fichier
                fs::remove_file(text).expect("file not found");
                file_lg.write_all(b"file deleted\n").expect("Unable to write data");
                //on redemarre le programme
                println!("restarting program");
                file_lg.write_all(b"restarting program\n").expect("Unable to write data");
                file_lg.write_all(b"program exited\n").expect("Unable to write data");

                //actual folder
                let path_tool = env::current_dir().unwrap();
                let path_tool = path_tool.to_str().unwrap();
                let path_tool = format!("{}\\tool.exe", path_tool);
                println!("path: {}", path_tool);
                
                std::process::Command::new("cmd.exe").arg("/C").arg("start").arg(path_tool).spawn().expect("failed to execute process");
                process::exit(0);
            }
            else {
                println!("file not found");
                file_lg.write_all(b"proc file not found").expect("Unable to write data");
                //close file
                file_lg.flush().expect("Unable to flush file");
                file_lg.sync_all().expect("Unable to sync file");
                file_lg.write_all(b"file closed\n").expect("Unable to write data");
                drop(file_lg);
                process::exit(1);
            }
        }



        
    } else {
        //si le fichier n'existe pas, ferme le programme
        println!("file not found");
        process::exit(1);

        
    }
}
