use std::fs::File;
use std::io::ErrorKind;

fn ex01_panicked() {
    panic!("crash and burn")
}

fn ex02_result() {
    let f = match File::open(("hell0.txt")) {
        Ok(file) => file,
        Err(err) => {
            panic!("There was a problem opening the file: {:?}", err);
        }
    };
}

fn ex03_result_err() {
    let filename = "hel.txt";
    let f = match File::open(filename) {
        Ok(file) => file,
        Err(ref err) if err.kind() == ErrorKind::NotFound => {
            match File::create(filename) {
                Ok(new_file) => new_file,
                Err(err) => {
                    panic!("Tried to create file but there was a problem: {:?}", err);
                }
            }
        },
        Err(err) => {
            panic!("There was a problem opening the file: {:?}", err);
        }
    };

}

fn main() {
    ex03_result_err();
}
