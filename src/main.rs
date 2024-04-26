use std::env;
use std::process::exit;

fn main() {
    let mut index: usize;
    let mut total_len: usize;
    let mut pw_string: Vec<char>;

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} <password>", args[0]);
        println!("Password: MCA Generated Password to parse for ease of reading");
        exit(1);
    }

    for arg_num in 1..args.len() {
        
        pw_string = args[arg_num].chars().collect();

        total_len = pw_string.len();
        index = 0;

        while index < (total_len - 1) {
            if ((pw_string[index]).to_uppercase().next().unwrap() != 'F') && (pw_string[index] != '-') {
                pw_string.insert(index + 1, '-');
                index += 2;
                total_len = pw_string.len();
            }
            else {
                index += 1;
            }
        }

        println!("Password {} with hyphens: {}", arg_num, pw_string.iter().collect::<String>());
    }
}
