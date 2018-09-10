use std::io;
use std::cmp;

/*
 * Struct to hold a big integer number as a vector of i8 numbers
 * The String source should never start with - sign 
 * nor 0* pattern
 */
struct Number {
    arr: Vec<i8>,
    negative: bool,
}

impl Number {
/*
 * returns a Number struct from a string and its sign
 */
    fn new(s: &String, neg: bool) -> Number {
        let l = s.len();
        let chars = s.chars();
        let negative = neg; 
        let mut v: Vec<i8> = Vec::with_capacity(l);
        for c in chars {
            v.push(c.to_digit(10).unwrap() as i8 );
        }

        let n = Number {
            arr: v,
            negative: negative
        };
        return n;
    }

    fn to_string(&self) -> String {
        let mut s = String::new();
        if self.negative {
            s.push('-');
        }
        for i in 0..self.arr.len() {
            s.push(format!("{}", self.arr[i]).chars().nth(0).unwrap());
        }

        return s;
    }
    
    fn first_half(&self) -> Number {
        return self.half(true);
    }

    fn second_half(&self) -> Number {
        return self.half(false);
    }

    fn half(&self, first: bool) -> Number {
        let l = self.arr.len()/2;
        let mut v = Vec::with_capacity(l);

        if first {
            v.extend_from_slice(&self.arr[..l]);
        }else{
            v.extend_from_slice(&self.arr[l..]);
        }

        return Number{
            arr: v,
            negative: self.negative
        };
    }
    /*
     * Compare self with other number,
     * return self<o: -1
     *        self=o: 0
     *        self>o: 1
     */
    fn compare(&self, o: &Number) -> i8 {
        let sl = self.arr.len();
        let ol = o.arr.len();
        if sl>ol {
            return 1;
        } else if ol>sl {
            return -1;
        }
        //if ls==ol compare each member
        let mut i = 0;
        while i < sl {
            if self.arr[i] > o.arr[i] {
                return 1;
            }else if self.arr[i] < o.arr[i] {
                return -1
            }
            i+=1;
        }
        return 0;
    }
}


fn main(){
    let a = read_string_line();
    let b = read_string_line();

    let mut na = Number::new(&a, false);
    let mut nb = Number::new(&b, false);

    println!("{}", karatsuba(&mut na, &mut nb).to_string());
}

fn karatsuba(x: &mut Number, y: &mut Number) -> Number {
    let mut mlen = x.arr.len();
    if y.arr.len() > mlen {
        mlen = y.arr.len();
    }
    fill_zero_pow(x, mlen);
    fill_zero_pow(y, mlen);
    let len = x.arr.len();
    //println!("K({}, {}) started.", x.to_string(), y.to_string());

    if x.arr.len() == 1 {
        let res = x.arr[0] * y.arr[0];
        let s = format!("{}", res.abs());
        let n = Number::new(&s, res<0);
        return n;
    }
    let half = len/2;

    let mut a = x.first_half();
    let mut b = x.second_half();
    let mut c = y.first_half();
    let mut d = y.second_half();
    //println!("A halves {} {}", a.to_string(), b.to_string());
    //println!("B halves {} {}", c.to_string(), d.to_string());

    let mut p = sum(&a, &b);
    let mut q = sum(&c, &d);

    let ac = karatsuba(&mut a, &mut c);
    let bd = karatsuba(&mut b, &mut d);
    let pq = karatsuba(&mut p, &mut q);

    let adbc = sum(&pq, &negate(&ac));
    let adbc = sum(&adbc, &negate(&bd));

    let res = sum(&add_zero(&ac, len), &add_zero(&adbc, half));
    let res = sum(&res, &bd);

    //println!("Karatsuba {} * {} == {}", x.to_string(), y.to_string(), res.to_string());
    return res;
}
fn fill_zero_pow(n: &mut Number, ml: usize) {
    let k = (ml as f64);
    let k = k.log2().ceil() as i32;
    //println!("Pow 2 {} for {}", k, n.to_string());
    let mut left = ((2 as f64).powi(k) - (n.arr.len() as f64)) as i32;
    if left <= 0 {
        return;
    }
    //println!(" Missing {} zeroes to {}", left, n.to_string());
    n.arr.reverse();
    n.arr.reserve(left as usize);
    while left > 0 {
        n.arr.push(0 as i8);
        left -= 1;
    }
    n.arr.reverse();
    //println!("Pow {} {}", k, n.to_string());
}

/*
 * this function will substract as well,
 * it will be valid for any of the following combinations
 * a + b
 * -a + b = b - a
 * - a - b = - (a + b)
 * a - b = - (b - a) if b>a else a - b
 */
fn sum(a: &Number, b: &Number) -> Number {
    let n = sum_validate(a, b, true);
    //println!("Sum {} + {} -> {}", a.to_string(), b.to_string(), n.to_string());
    //assert_eq!(a.to_string().parse::<i64>().unwrap()+b.to_string().parse::<i64>().unwrap(), n.to_string().parse::<i64>().unwrap());
    return n;
}

fn sum_validate(a: &Number, b: &Number, validate: bool) -> Number {
    if validate && a.negative && b.negative {
        return negate(&sum_validate(&negate(a), &negate(b), false));
    }
    if validate && !a.negative && b.negative {
        let cmp = a.compare(b);
        if cmp == 0 {
            let mut v = Vec::new();
            v.push(0);
            return Number {
                arr: v,
                negative: false
            };
        }else if cmp < 0 {
            return negate(&sum_validate(&negate(b), &negate(a), false));
        }
    }
    if validate && a.negative && !b.negative {
        return sum_validate(b, a, true);
    }
    //println!("Trying to sum {}, {}", a.to_string(), b.to_string());
    let mut v: Vec<i8> = Vec::with_capacity(a.arr.len()+2);

    let mut ia: i32 = (a.arr.len()-1) as i32;
    let mut ib: i32 = (b.arr.len()-1) as i32;
    let mut carry = 0;
    let mut x;
    let mut y;
    let mut z;
    let mut negcarry = false;

    while ia>=0 || ib>=0 {
        if ia >= 0 {
            x = a.arr[ia as usize];
            ia -= 1;
        }else{
            x = 0;
        }
            x += carry;
            carry = 0;
        if ib >= 0 {
            y = b.arr[ib as usize];
            ib -= 1;
        }else{
            y = 0;
        }
        if b.negative {
            if y>x {
                x+=10;
                negcarry = true;
            }else{
                negcarry = false;
            }
            z = x - y;
            if negcarry {
                carry=-1;
            }else{
                carry=0;
            }
        }else {
            z = x + y;
            carry = z / 10;
        }
        v.push((z%10 as i8).abs());
    }
    if !b.negative && carry>0 {
        v.push(carry);
    }
    v.reverse();
    let mut sign = false;
    let cmp = a.compare(b);
    if a.negative && b.negative {
        sign = true;
    } else if a.negative || b.negative {
        if a.negative && cmp>1 {
            sign = true;
        }else if b.negative && cmp < 0 {
            sign = true;
        }
    }

    remove_front_zeros(&mut v);


    return Number{
        arr: v,
        negative: sign
    };
}

fn remove_front_zeros(v: &mut Vec<i8>)  {
    let zero: i8 = 0;
    
    while v.len()>1 && *v.first().unwrap() == zero {
        //remove
        // println!("removing");
        v.remove(0);
    }
}

/*
 * returns a new Number with the sign toggled
 */
fn negate(n: &Number) -> Number {
    let v = n.arr.to_vec();
    return Number{
        arr: v,
        negative: !n.negative
    };
}

/*
 * adds 0 until arr is length = arr.len()+z
 */
fn add_zero(n: &Number, z: usize) -> Number {
    let mut v = n.arr.to_vec();
    v.reserve(z);
    for _i in 0..z {
        v.push(0 as i8);
    }
    remove_front_zeros(&mut v);
    let nn =  Number{
        arr: v,
        negative: n.negative
    };
    return nn;
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
