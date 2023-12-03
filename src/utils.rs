
pub mod file {
    use std::fs::File;
    use std::io;
    use std::io::BufRead;
    use std::path::Path;


    pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
        where P: AsRef<Path>, {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }


    pub fn aggregate_lines<P, F, R>(filename: P, line_processor: F, init: R) -> Result<R, &'static str> where P: AsRef<Path>, F: Fn(&R, &str) -> Option<R> {
        let mut agg = init;
        if let Ok(lines) = read_lines(filename) {
            for line in lines {
                match line {
                    Ok(code) => {
                        if let Some(new_result) = line_processor(&agg, &code) {
                            agg = new_result;
                        }
                    }
                    Err(_) => {
                        return Err("Error reading the file, please try again");
                    }
                }
            }
        }
        Ok(agg)
    }
}

pub mod output {
    use std::fmt::Display;

    pub fn print_solution<T:Display>(day: i32, part: i32, potential_result: Result<T, &str>)  {
        match potential_result {
            Ok(result) => {
                let message = format!("Solution to day {} part {} is {}", day, part, result);
                println!("{}", message);

            }
            Err(msg) => {
                println!("{}", msg)
            }
        }
    }
}
