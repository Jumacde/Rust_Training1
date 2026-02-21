pub fn test_list() {

    let list1 : [i32; 2] = [2, 4];
    let list2 : [&str; 3] = ["hello", "world", "rust"];
    
    let mut empList1= Vec:: new();
    let num1 : i32 = 123;
    empList1.push(num1);
    
    let mut empList2= Vec:: new();
    empList2.push("A");
    empList2.extend(["B", "C"]);

    let mut empList3= Vec:: new();
    empList3.push(1);
    empList3.extend([2, 3, 4, 5]);
    //empList3.pop(); // remove only the element from the list by index -1.
    //empList3.remove(0); // // remove only the element from the list by their index.
    //empList3.clear();
    empList3.drain( 0..2).for_each(drop); // remove elements by index and range.
    empList3.retain(|&x| x != 3); // remove elements with value 3

    let mut empList4 = Vec:: new();
    empList4.extend(["hello", "moin", "hi", "hey"]); 
    empList4.push("Hallo");
    let mut empList5 = empList4.clone();
    empList5.retain(|x| !x.contains ("llo"));
    empList4.retain(|&o| !o.starts_with("h")); // remove elements by starting "h"
    empList4.retain(|&x| !x.ends_with("o")); // remove elements by ending "o"
    empList4.retain(|&y| !y.contains("ey"));
    

    println!("list1 is {:?}", list1);
    println!("the index 0 of List1 is:{} ", list1[0]) ;
    println!("list2 is {:?}", list2);
    println!("the index 1 of List2 is:{} ", list2[1]) ;
    println!("empList empList1 is added {} and show it : {:?} " ,num1
    , empList1);
    println!("empList empList2 is created and show it : {:?} "
    , empList2);
    println!("empList empList3 is created and show it : {:?} ", empList3);
    println!("empList empList4 is created and show it : {:?} ", empList4);
    println!("the empList cloened and removed element words with include llo {:?}", empList5);


} 