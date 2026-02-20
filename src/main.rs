mod calcMethod;
mod tryList;
mod tryHashMap;

fn main( ) {
    println!("<< try calculations. >>");
    calcMethod::testType();
    println!("<< try Lists. >>");
    tryList::testList();
    println!("<< try HashMap. >>");
    tryHashMap:: testHashmap();
}