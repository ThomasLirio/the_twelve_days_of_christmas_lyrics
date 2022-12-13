

fn main() {
    const V1_BEGIN: &str = "On the ";
    const V1_END: &str = " day of Christmas, my true love sent to me";

    let ordinals = ["first",
                                "second", 
                                "third", 
                                "fourth", 
                                "fifth", 
                                "sixth", 
                                "seventh",
                                "eighth", 
                                "ninth", 
                                "tenth", 
                                "eleventh", 
                                "twelfth"];


    let gifted_verses = ["And a partridge in a pear tree",
                                     "Two turtledoves", 
                                     "Three French hens", 
                                     "Four calling birds", 
                                     "Five gold rings (five golden rings)", 
                                     "Six geese a-laying", 
                                     "Seven swans a-swimming",
                                     "Eight maids a-milking", 
                                     "Nine ladies dancing", 
                                     "Ten lords a-leaping", 
                                     "Eleven pipers piping", 
                                     "Twelve drummers drumming"];

    
    println!("Twelve Days Of Christmas\n\n\n");

    for i in 0..=11 {
    
        let cardinal = ordinals[i];
        println!("{V1_BEGIN}{cardinal}{V1_END}");
        if i == 0 {
            println!("A partridge in a pear tree");
        } else {
            for j in (0..=i).rev() {
                if j==10 && i==10 {
                    println!("I sent {}", gifted_verses[j]);
                } else {
                    println!("{}", gifted_verses[j]);
                }
            }
        }
        println!("\n");
    }
   
    println!("{}", gifted_verses[0]);
}
