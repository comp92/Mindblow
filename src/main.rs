use std::fs::File;
use std::io::Read;
use std::env;

/**
 * mindblow: A brainfuck interpreter written in Rust.
 * @author: Daniel Shaw (https://github.com/comp92)
 *
 * Desc:
 *  My first project using the Rust language.
 *  Brainfuck is a simple enough interpreted
 *  langugae, so why not start learning Rust
 *  with it :)
 *
 * License: MIT (see LICENSE.MD)
 */
fn main() {
    const MAX_SIZE:usize = 30000;
    let mut mem: [u8; MAX_SIZE] = [0; MAX_SIZE];
    let mut pointer: usize = 0;
    let mut buffer = String::new();
    //TODO: Handle no args given
    let mut script = File::open(env::args().nth(1).unwrap()).unwrap();
    script.read_to_string(&mut buffer).unwrap();
    for token in buffer.chars() {
        match token {
            '+' => {
                mem[pointer]+=1;
            },
            '-' => {
                mem[pointer]-=1;
            },
            '>' => {
                if pointer < MAX_SIZE {
                    pointer+=1;
                } else {
                    pointer=0;
                }
            },
            '<' => {
                if pointer > 0 {
                    pointer-=1;
                } else {
                    pointer=MAX_SIZE;
                }
            },
            '[' => {
                //TODO: Loops
            },
            ']' => {
                //TODO: Loops
            },
            '.' => {
                print!("{}",&(mem[pointer] as char));
            },
            ',' => {
                //http://stackoverflow.com/a/30679861 I am a noob
                mem[pointer] = std::io::stdin()
                    .bytes().next().and_then(|result| result.ok()).map(|byte| byte as u8).unwrap();
            },
            _ => (),
        }
    }
}
