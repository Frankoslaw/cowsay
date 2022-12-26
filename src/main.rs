use std::io;

fn main() {
    let mut input_buffer: Vec<String> = vec![];

    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read from pipe");
        if input == "" {
            break;
        }

        input_buffer.push(input.trim().to_string());
    }

    let cow = r"        \  ^__^
         \ (oo)\_______
           (__)\       )\/\
               ||----w |
               ||     ||";

    let longest_line_size = input_buffer
        .iter()
        .map(|c| c.len())
        .fold(std::usize::MIN, |a,b| a.max(b));

    println!(" {}", "_".repeat(longest_line_size+2));

    if input_buffer.len() < 2{
        println!("< {} >", input_buffer[0]);
    }else{
        println!("/ {:<longest_line_size$} \\", input_buffer[0]);

        for n in 1..input_buffer.len() - 1{
            println!("| {:<longest_line_size$} |", input_buffer[n]);
        }

        println!("\\ {:<longest_line_size$} /", input_buffer[input_buffer.len()-1]);
    }

    println!(" {}", "-".repeat(longest_line_size+2));

    println!("{}", cow);
}
