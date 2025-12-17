fn main(){
    let (e, f) = (1, 100);

    let greater = f > e;        // true
    let less = f < e;           // false
    let greater_equal = f >= e; // true
    let less_equal = e <= f;    // true
    let equal_to = e == f;      // false
    let not_equal_to = e != f;  // true

    let and = greater && less; // false
    let or = greater || less; // true
    let not = !greater; // false

    let mut k = 9;
    let mut l = k;

    k -= l;


    println!("{}", k);

}