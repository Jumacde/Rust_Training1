pub fn testList() {

    let list1 : [i32; 2] = [2, 4];
    let list2 : [&str; 3] = ["hello", "world", "rust"];
    let mut empList= Vec:: new();
    let num1 : i32 = 123;
    empList.push(num1);
    println!("list1 is {:?}", list1);
    println!("the index 0 of List1 is:{} ", list1[0]) ;
    println!("list2 is {:?}", list2);
    println!("the index 1 of List2 is:{} ", list2[1]) ;
    print!("empList empList is added {} and show it : {:?} " ,num1
    , empList);


} 