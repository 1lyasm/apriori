use std::io::BufRead;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_file_name = "input.txt".to_string();
    let input_file = std::fs::File::open(input_file_name)?;
    let mut lines = std::io::BufReader::new(input_file).lines();

    let first_line = lines.next().unwrap()?;
    let first_line_words: Vec<_> = first_line.split(" ").collect();

    let support = first_line_words.get(0).unwrap().parse::<i64>()?;
    println!("main: support: {}", support);

    let confidence = first_line_words.get(1).unwrap().parse::<f64>()?;
    println!("main: confidence: {}", confidence);

    Ok(())
}

