pub fn test_list() {

    let list1 : [i32; 2] = [2, 4];
    let list2 : [&str; 3] = ["hello", "world", "rust"];
    
    let mut emp_list1= Vec:: new();
    let num1 : i32 = 123;
    emp_list1.push(num1);
    
    let mut emp_list2= Vec:: new();
    emp_list2.push("A");
    emp_list2.extend(["B", "C"]);

    let mut emp_list3= Vec:: new();
    emp_list3.push(1);
    emp_list3.extend([2, 3, 4, 5]);
    //emp_list3.pop(); // remove only the element from the list by index -1.
    //emp_list3.remove(0); // // remove only the element from the list by their index.
    //emp_list3.clear();
    emp_list3.drain( 0..2).for_each(drop); // remove elements by index and range.
    emp_list3.retain(|&x| x != 3); // remove elements with value 3

    let mut emp_list4 = Vec:: new();
    emp_list4.extend(["hello", "moin", "hi", "hey"]); 
    emp_list4.push("Hallo");
    let mut emp_list5 = emp_list4.clone();
    emp_list5.retain(|x| !x.contains ("llo"));
    emp_list4.retain(|&o| !o.starts_with("h")); // remove elements by starting "h"
    emp_list4.retain(|&x| !x.ends_with("o")); // remove elements by ending "o"
    emp_list4.retain(|&y| !y.contains("ey"));
    

    println!("list1 is {:?}", list1);
    println!("the index 0 of List1 is:{} ", list1[0]) ;
    println!("list2 is {:?}", list2);
    println!("the index 1 of List2 is:{} ", list2[1]) ;
    println!("emp_list1 is added {} and show it : {:?} " ,num1
    , emp_list1);
    println!("emp_list2 is created and show it : {:?} "
    , emp_list2);
    println!("emp_list3 is created and show it : {:?} ", emp_list3);
    println!("emp_list4 is created and show it : {:?} ", emp_list4);
    println!("the emp_list cloened and removed element words with include llo {:?}", emp_list5);


} 