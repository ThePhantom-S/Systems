use std::io::BufRead;

pub fn process_lines(reader: BufRead){
    for line in reader.lines(){
        println!("{}", line);
    }
}