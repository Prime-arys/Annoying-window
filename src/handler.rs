
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

    thread::sleep(time::Duration::from_secs(3));

    //prend un texte en argument
    let args: Vec<String> = env::args().collect();
    let text = &args[1];

    
    //hide console window
    //let _ = unsafe { winapi::um::wincon::FreeConsole() };

    //verifie si le fichier avec le meme nom existe deja dans le dossier courant
    let path = Path::new(text);
    let existe = path.exists();
    if existe == true {
        //lit le fichier et le stock dans une variable
        let mut file = File::open(text).expect("file not found");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("something went wrong reading the file");
        //affiche le contenu du fichier
        println!("With text:\n{}", contents);

        //le contenu du fichier est un PID, on le converti en entier
        let pid = contents.parse::<i32>().unwrap();

        

        //dans une boucle, on verifie toutes les 30 secondes si le processus existe toujours et si le fichier existe toujours
        loop {
            let process = process_exists(pid);
            let path = Path::new(text);
            let existe = path.exists();
            if process == true && existe == true {
                println!("process {} is running", pid);
                thread::sleep(time::Duration::from_secs(5));
            } else {
                println!("process {} is not running", pid);
                //on supprime le fichier
                fs::remove_file(text).expect("file not found");
                //on redemarre le programme
                std::process::Command::new("cmd.exe").arg("/C").arg("start").arg("tool.exe").spawn().expect("failed to execute process");
                process::exit(0);
            }
        }



        
    } else {
        //si le fichier n'existe pas, ferme le programme
        println!("file not found");
        process::exit(1);

        
    }
}
