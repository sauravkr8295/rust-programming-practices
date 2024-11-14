
fn get_largest(list : &[i32]) -> &i32{
    let mut max_ele=&list[0];

    for ele in list{
        if ele > max_ele {
            max_ele=ele;
        }
    }

    max_ele
}

fn main() {
    let list=vec![100,20,30,10,104,41,40];
    let res=get_largest(&list);
    println!("The maximum element will be {res}");

    let list=vec![123,20,30,112,104,41,40];
    let res=get_largest(&list);
    println!("The maximum element will be {res}");
}
