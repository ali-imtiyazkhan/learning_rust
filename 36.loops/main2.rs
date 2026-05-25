fn main(){
    let mut count : u32 = 0u32;

    // infinite loop but with a break condition
    loop {
           count +=1;

           if count == 3 {
            println!("three");

            continue;
           }

         println!(" count is {} ", count);

         if count == 5 {
            println!("ok, that's enough");
            println!("count is {}", count);

            break;
         }  

    }
}