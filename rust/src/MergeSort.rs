use std::io;
/*
 * Input: array A of n distinct integers
 * Output: array with the same integers, sorted asc
 */

fn main() {
    let line = read_string_line();
    let a = line_to_array(&line);
    let sorted = merge_sort(&a);
    println!("{:?}", sorted);
}

fn merge_sort(a: &[i32]) -> Vec<i32> {
    if a.len() == 1 {
        return a.to_vec();
    }
    let half = a.len()/2;
    //println!("{} half of {:?}", half, a);
    let c = merge_sort(&(a[..half]));
    let d = merge_sort(&(a[half..]));

    //println!("c {:?}", c);
    /* Problem 1,1
    if half > 1 {
         println!(" 7th {}", d[1] );
    }
    */
    return merge(&c, &d);
}


fn merge(c: &[i32], d: &[i32]) -> Vec<i32> {
    let mut i=0;
    let mut j=0;
    let n = c.len()+d.len();
    let mut v = Vec::with_capacity(n);
    let mut c_exhausted = false;
    let mut d_exhausted = false;
    for _ in 0..n {
        if c_exhausted {
            v.push(d[j]);
            j+=1;
        }
        if d_exhausted {
            v.push(c[i]);
            i+=1;
        }
        if !c_exhausted && !d_exhausted {
            if c[i] < d[j] {
                v.push(c[i]);
                i+=1;
                if i>= c.len() {
                    c_exhausted = true;
                }
            }else{
                v.push(d[j]);
                j+=1;
                if j>= d.len() {
                    d_exhausted = true;
                }
            }
        }
    }
    return v;
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
