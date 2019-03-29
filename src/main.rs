
use ckb_vm::{run, SparseMemory};
use std::fs::File;
use std::io::Read;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_name = &args[1];

    let pramater = &args[2].as_bytes();
    
    let mut file = File::open(file_name).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    // use x64 executor
    let result = run::<u64, SparseMemory<u64>>(&buffer, &vec![
            b"simple".to_vec(),     // arg 1
            pramater.to_vec(),        // arg 2
        ]);
    println!("Result: {:?}", result.unwrap());
}
