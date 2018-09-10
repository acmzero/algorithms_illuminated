use std::io;
/*
 * First rust program ever, I'm sure it's ugly as it can be...
 */

//Input: two n-digit positive integers x and y.
//Output the product x.y
//Assumption n is a power of 2
fn main(){
    let a = read_string_line();
    let b = read_string_line();

    println!("{}", rec_int_mult(&a, &b));

}

fn rec_int_mult(x: &String, y: &String) -> String{
    let n = x.len();
    if n == 1 {
        // return simple multiplication
        let a = x.parse::<i32>().unwrap();
        let b = y.parse::<i32>().unwrap();
        return format!("{}", a*b);

    }
    //else
    let half = n/2;

    let a = x[..half].to_string();
    let b = x[half..].to_string();

    let c = y[..half].to_string();
    let d = y[half..].to_string();

    let ac = rec_int_mult(&a, &c);
    let ad = rec_int_mult(&a, &d);
    let bc = rec_int_mult(&b, &c);
    let bd = rec_int_mult(&b, &d);

    let ten_ac = zero_fill(&ac, n);
    let ad_bc = sum(&ad, &bc);
    let ad_bc = zero_fill(&ad_bc, half);
    let ad_bc = sum(&ad_bc, &bd);//sum right part
    let res = sum(&ten_ac, &ad_bc);//then sum left

    res
}

fn sum(a: &String, b: &String) -> String {
    let mut carry = 0;
    let len_a = a.len();
    let len_b = b.len();

    let mut ia: i32 = ((len_a-1) as i32);
    let mut ib: i32 = ((len_b-1) as i32);

    let mut res = String::new();

    let mut x = 0;
    let mut y = 0;
    let mut z = 0;

    while ia>=0 || ib>=0 {

        if ia >=0 {
            x = a.chars().nth(ia as usize).unwrap().to_string().parse::<i32>().unwrap();
            ia-=1;
        }else{
            x = 0;
        }

        if ib >= 0 {
            y = b.chars().nth(ib as usize).unwrap().to_string().parse::<i32>().unwrap();
            ib-=1;
        }else{
            y = 0;
        }

        z = x+y+carry;
        carry = z/10;
        res.push(format!("{}", z%10).chars().nth(0).unwrap());

    }

    if carry > 0 {
        res.push(format!("{}", carry).chars().nth(0).unwrap());
    }

    //println!("Pre Rev sum of  of {}, {} -> {}", a, b, res);
    let res = res.chars().rev().collect::<String>();
    //println!("Sum of  of {}, {} -> {}", a, b, res);
    res
}

fn zero_fill(x: &String, n: usize) -> String {
    let mut res = String::new();
    let mut i = 0;
    //FIXME: do copy instead
    while i < x.len() {
        res.push((*x).chars().nth(i).unwrap());
        i+=1;
    }
    i=0;

     while i<n {
        res.push('0');
        i+=1;
    }

    //println!("Zero fill of {}, {} -> {}", x, n, res);

    res.to_string()
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
