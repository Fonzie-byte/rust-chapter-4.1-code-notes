fn main() {
    let a = Box::new([0; 10]); // a on the Stack, is just a pointer to the values on the Heap
    println!("{:?}", a); // A-Ok!

    {
        let b = a; // a gave up ownership to b
        //println!("{:?}", a);  // a "is moved", can no longer be used (empty husk on the Stack)
        println!("{:?}", b); // A-Ok!
    } // b is popped off the Stack; values on the Heap are deallocated, too!

    //println!("{:?}", a);   // a is still "moved", still cannot be used (still empty husk on the Stack)
    //println!("{:?}", b);   // b is already gone by this point
} // a is popped off the Stack; no values are cleared since it didn't own any (anymore)!
