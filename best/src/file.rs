// Saving data to files
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

pub fn save_columns_to_file(vecs: &Vec<Vec<f64>>, dir_name: &str, file_name: &str) {
    let try_create_dir = std::fs::create_dir(dir_name);
    match try_create_dir {
        Ok(create_dir) => {create_dir},
        Err(_) => {},
    };
    let file_path: PathBuf = [dir_name, file_name].iter().collect();
    let mut my_file = File::create(file_path).expect("Error creating file");
    let vec_num = vecs.len();
    let mut el_nums = Vec::new();
    for i in 0..vec_num {
        el_nums.push(vecs[i].len())
    }
    let num = el_nums.iter().max().unwrap().to_owned();
    for j in 0..num {
        for i in 0..vec_num {
            if el_nums[i] > j {
                write!(my_file, "{:.6} ", vecs[i][j]).expect(&format!("Can't write line {} to file", j));
            } else {
                write!(my_file, "{} ", "NAN").expect(&format!("Can't write line {} to file", j));
            }
        }
        write!(my_file, "{}", "\n").expect(&format!("Can't write line {} to file", j));
    }
}
