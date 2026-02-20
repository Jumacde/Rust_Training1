use std::collections::HashMap;

pub fn testHashmap() {
 let mut hashmap1 = HashMap::new();
 hashmap1.insert("Alice", 1001);
 hashmap1.insert("Bob", 1129);
 hashmap1.insert("Charlie", 102390);

 let mut hashmap2 = HashMap:: new();
 let mut list1 = Vec:: new();
 let mut list2 = Vec:: new();
 list1.extend(["David", "Erick"]);
 list2.extend([4512, 312479]);
 hashmap2 = list1.into_iter().zip(list2).collect();
 println!("hashmap1: {:?}", hashmap1);
 println!("hashList2(created by 2 lists: {:?}", hashmap2);
 hashmap1.extend(hashmap2);
 println!("hasmap2 is inserted in to hashmap1: {:?}", hashmap1);


}