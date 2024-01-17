use std::io::BufRead;

fn print_hashmap<K: std::fmt::Debug, V: std::fmt::Debug>(
    hash_map: &std::collections::HashMap<K, V>,
) {
    print!("print_hashmap: [ ");
    for (key, value) in hash_map {
        print!("{{{:?}: {:?}}} ", key, value);
    }
    print!("\n");
}

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

    let mut item_counts = std::collections::HashMap::<String, i64>::new();

    for line in lines {
        let words: Vec<_> = line.as_ref().unwrap().split(" ").collect();
        for word in words {
            item_counts
                .entry(word.to_owned())
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        }
    }

    print_hashmap(&item_counts);

    item_counts.retain(|_, value| *value >= support);

    print_hashmap(&item_counts);

    Ok(())
}

