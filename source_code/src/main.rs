mod main_code;
fn main() {
    use crate::main_code::utilities::general;
    let input_file = general::NumLines::new("example.txt","");
    general::remove_empty_lines("example.txt");
    input_file.numerate_lines();
   // input_file.remove_num_lines();
    let n = input_file.skip_num_line("1 This is a test line.");
    let s = input_file.get_num_line("1 This is another test line.");
    println!("Processing completed for file: {}, {}", n, s);
}
