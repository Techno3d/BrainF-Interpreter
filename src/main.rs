use std::{fs, env, process, io};


//Commands in brainf
#[derive(Debug, PartialEq, Copy, Clone)]
enum Token {
    Add,        // +
    Subtract,   // -
    Right,      // >
    Left,       // <
    Read,       // ,
    Write,      // .
    BeginLoop,  // [
    EndLoop,    // ]
}

fn tokenize(command: &str) -> Vec<Token> {
    let mut tokens = Vec::<Token>::new();

    let chars = command.chars();

    for c in chars.into_iter() {
        match c {
            '+' => tokens.push(Token::Add),
            '-' => tokens.push(Token::Subtract),
            '>' => tokens.push(Token::Right),
            '<' => tokens.push(Token::Left),
            ',' => tokens.push(Token::Read),
            '.' => tokens.push(Token::Write),
            '[' => tokens.push(Token::BeginLoop),
            ']' => tokens.push(Token::EndLoop),
            _ => {}
        }
    }

    tokens
}

fn main() {
    if env::args().len() < 2 {
        println!("Give a brainf file to compile");
        process::exit(1);
    }
    
    let mut _file = env::args().last().expect("No File Given");
    let contents = fs::read_to_string(&_file).expect("Couldn't read it sad");

    let tokens = tokenize(&contents);
    
    let mut brainf_array= [0; 20000];
    let mut ptr: usize = 0;
    let mut stored_ptr = 0usize;
    let mut stored_loc = 0usize;
    let mut i = 0;

    while i < tokens.len() {
        let mut do_loop = false;
        let token = tokens.get(i).unwrap();
        match token {
            Token::Add => {
                brainf_array[ptr] += 1;
            },
            Token::Subtract => {
                if brainf_array[ptr] > 0 {
                    brainf_array[ptr] -= 1;
                }
            },
            Token::Right => {
                ptr += 1;
            },
            Token::Left => {
                if ptr > 0 {
                    ptr -= 1;
                } else {
                    ptr = 0;
                }
            },
            Token::Read => {
                let mut read = String::new();
                io::stdin().read_line(&mut read).expect("Failed To Read");
                
                brainf_array[ptr] = read.chars().last().expect("Oops") as u32;
            },
            Token::Write => {
                print!("{}", char::from_u32(brainf_array[ptr]).unwrap());
            },
            Token::BeginLoop => {
                stored_ptr = ptr;
                stored_loc = i + 1;
            },
            Token::EndLoop => {
                if brainf_array[stored_ptr] == 0 {
                } else {
                    i = stored_loc;
                    do_loop = true;
                }
            },
        }
        if !do_loop {
            i += 1;
        }
    }


}
