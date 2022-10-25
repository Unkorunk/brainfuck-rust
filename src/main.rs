use std::collections::HashMap;
use std::io;

struct Jumps {
    data: HashMap<usize, usize>,
}

impl Jumps {
    fn new(data: HashMap<usize, usize>) -> Jumps {
        Jumps { data }
    }

    fn get_inverse(&self) -> Jumps {
        let mut jumps = HashMap::<usize, usize>::new();
        for (from, to) in &self.data {
            jumps.insert(*to, *from);
        }

        Jumps { data: jumps }
    }

    fn jump(&self, ip: usize) -> usize {
        self.data[&ip]
    }
}

fn calculate_jumps(program: &str) -> Option<Jumps> {
    let mut jumps = HashMap::<usize, usize>::new();
    let mut open_indexes = Vec::<usize>::new();

    for i in 0..program.chars().count() {
        let command = program.chars().nth(i).unwrap();
        if command == '[' {
            open_indexes.push(i);
        } else if command == ']' {
            if open_indexes.is_empty() {
                return None;
            }

            let open_idx = open_indexes.pop().unwrap();
            jumps.insert(open_idx, i);
        }
    }

    if !open_indexes.is_empty() {
        None
    } else {
        Some(Jumps::new(jumps))
    }
}

fn main() {
    let mut data = [0u8; 30000];
    let mut dp = 0usize;

    let program = ".>+.  >>++++++++++++  [ -<<  <[->>+<<]>  [- <+> >+<]>  .  [-<+>]  >  ]";
    let mut ip = 0usize;
    let forward_jumps = calculate_jumps(program).expect("square brackets must be balanced");
    let backward_jumps = forward_jumps.get_inverse();

    loop {
        if let Some(command) = program.chars().nth(ip) {
            match command {
                '>' => dp += 1,
                '<' => dp -= 1,
                '+' => data[dp] += 1,
                '-' => data[dp] -= 1,
                '.' => print!("{} ", data[dp]),
                ',' => {
                    let mut input_str = String::new();
                    io::stdin().read_line(&mut input_str).unwrap();
                    data[dp] = input_str.trim_end().parse().unwrap();
                }
                '[' => if data[dp] == 0 {
                    ip = forward_jumps.jump(ip);
                },
                ']' => if data[dp] != 0 {
                    ip = backward_jumps.jump(ip);
                },
                ' ' => {}
                _ => panic!("invalid type of command: {command}"),
            }
        } else {
            break;
        }

        ip += 1;
    }
}