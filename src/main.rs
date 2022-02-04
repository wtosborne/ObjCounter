use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::process;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut vertices = 0;
    let mut faces = 0;
    let mut texCoords = 0;
    let mut normals = 0;
    let mut pvertices = 0;
    let mut mlines = 0;
    if (args.len() < 2) {
        println!("Please supply a file");
        process::exit(1);
    }
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines(&args[1]) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                let mut split = ip.split(" ");
                for s in split {
                    match s {
                        "v"=>vertices+=1,
                        "vt"=>texCoords+=1,
                        "vn"=>normals+=1,
                        "vp"=>pvertices+=1,
                        "f"=>faces+=1,
                        "l"=>mlines+=1,
                        _ => break,
                    }
                }
            }
        }
    }
    println!("{}:", args[1]);
    println!("Vertices: {}", vertices);
    println!("Texture Coordinates: {}", texCoords);
    println!("Vertex Normals: {}", normals);
    println!("Parameter Space Vertices: {}", pvertices);
    println!("Polygonal Faces: {}", faces);
    println!("Lines: {}", mlines);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}