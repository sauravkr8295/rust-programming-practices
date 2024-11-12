use std::collections::HashMap;

fn find_median_mode(v: &mut Vec<i32>) -> (f64, i32){
    v.sort(); //[1,2,3,3,4,5,6,9,9]
    let length=v.len();
    let median:f64= if length%2!=0{
        v[length/2] as f64
    }
    else{
        (v[length/2-1] as f64+v[length/2] as f64)/2.0
    };

    let mut map=HashMap::new();
    let mut counti=0;
    for i in v{
        let count=map.entry(i).or_insert(0);
        *count +=1;
        if counti<*count{
            counti=*count;
        }
    }
    (median,counti)

}



fn main() {
    let mut v=vec![3,2,1,6,5,3,9,9,3,3];
    let (median,mode) =find_median_mode(&mut v);
    println!("Median: {median}");
    println!("Mode: {mode}");
}
