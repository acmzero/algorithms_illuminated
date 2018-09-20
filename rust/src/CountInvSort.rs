use std::io;

fn main(){
    let arr = read_string_line();
    let arr = line_to_array(&arr);
    let (_sorted_arr, inv) = sort_and_countinv(&arr);
    println!("{}", inv);
}

fn sort_and_countinv(a: &[i32]) -> (Vec<i32>, i32){
    let n = a.len();
    if n==0 || n==1 {
        return (a.to_vec(),0);
    }
    let half = n/2;
    let (c, left_inv) = sort_and_countinv(&a[..half]);
    let (d, right_inv) = sort_and_countinv(&a[half..]);
    let (b, split_inv) = merge_and_countsplitinv(&c, &d);
    return (b, left_inv+right_inv+split_inv);
}

fn merge_and_countsplitinv(c: &[i32], d: &[i32]) -> (Vec<i32>, i32) {
    let n = c.len() + d.len();
    let mut i = 0;
    let mut j = 0;
    let mut split_inv: i32 = 0;
    let mut b = Vec::new();
    for _ in 0..n {
        if i >= c.len() || j >= d.len() {
            break;
        }
        if c[i] < d[j] {
            b.push(c[i]);
            i += 1;
        }else{
            b.push(d[j]);
            j += 1;
            split_inv += ((n/2) -i ) as i32; // +2??
        }
    }
    if i >= c.len() {
        for k in j..d.len() {
            b.push(d[k]);
        }
    }
    if j >= d.len() {
        for k in i..c.len() {
            b.push(c[k]);
        }
    }

    return (b, split_inv);

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
