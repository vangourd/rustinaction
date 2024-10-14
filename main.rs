#[derive(Debug,PartialEq)]
pub enum FileState {        // 1
    Open,
    Closed,
}

#[derive(Debug)]
pub struct File {
    pub name: String,
    data: Vec<u8>,      // 2
    pub state: FileState,
}

impl File {
    pub fn new(name: &str) -> File {    // 3
        File {
            name: String::from(name),
            data: Vec::new(),
            state: FileState::Closed
        }
    }
}

fn main() {
    let f7 = File::new("f7.txt");
    println!("{:?}", f7);
}