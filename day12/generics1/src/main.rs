fn get_max(arr: &[i32]) -> i32 {
    let mut max_ele=&arr[0];
    
    for ele in arr{
        if ele > max_ele {
            max_ele=ele;
        }
    }
    max_ele
}

fn main() {
    let arr=vec![50,2,60,40,5];
    let res=get_max(&arr);
    println!("Maximum element will be: {res}");
}
