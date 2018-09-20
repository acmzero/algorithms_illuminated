use std::io;

fn main(){
    let arr = read_string_line();
    let arr = line_to_array(&arr);
    let n = arr.len();
    let mut num_inv = 0;
    for i in 0..n {
        for j in i+1..n {
            if arr[i]>arr[j] {
                num_inv += 1;
            }
        }
    }
    println!("{}", num_inv);
}

fn line_to_array(s: &String) -> Vec<i32> {
    let split = s.split(" ");
    let mut arr = Vec::new();
    for a in split {
        arr.push(a.parse::<i32>().unwrap());
    }
    return arr;
}

fn read_string_line() -> String {
    let mut x = String::new();
    match io::stdin().read_line(&mut x) {
        Ok(_) => {
        }
        Err(_) => {
        }
    };

    x.trim().to_string()
}
