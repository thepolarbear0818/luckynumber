use std::io;

fn check_lucky(input_number: String) {
    let v = input_number.trim().to_string();
    let v: Vec<&str> = v.split(" ").collect();
    //println!("{:?}",v);

    if v.len() != 6 {
        panic!("Your number must be 6 digits long!");
    }

    let x1 = v[0];
    let x1: i32 = x1.trim().parse().expect("Input not an integer");
    let x2 = v[1];
    let x2: i32 = x2.trim().parse().expect("Input not an integer");
    let x3 = v[2];
    let x3: i32 = x3.trim().parse().expect("Input not an integer");
    let y1 = v[3];
    let y1: i32 = y1.trim().parse().expect("Input not an integer");
    let y2 = v[4];
    let y2: i32 = y2.trim().parse().expect("Input not an integer");
    let y3 = v[5];
    let y3: i32 = y3.trim().parse().expect("Input not an integer");
    let x = x1 + x2 + x3;
    let y = y1 + y2 + y3;

  

    if y == x {
        println!("{} is a lucky number",input_number);
    }
    else{
        println!("This is not a lucky number")
    } 

}

fn main() {
/*     println!("Input");
    let mut input = String::with_capacity(6);
    io::stdin().read_line(&mut input).unwrap();
    let input: Vec<i32> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();
    println!("{:?}",input);
   */
    let mut total_number = String::new();
    println!("Total Number of Input Statements");
    io::stdin()
    .read_line(&mut total_number)
    .expect("Failed to read line");
    let total_number: i32 = total_number.trim().parse().expect("Input not an integer");

    for i in 0..total_number{
        let mut input_number = String::with_capacity(6);
        println!("Input your {} value",i);
        io::stdin()
        .read_line(&mut input_number)
        .expect("Failed to read line");
        input_number.insert(1,' ');
        input_number.insert(3,' ');
        input_number.insert(5,' ');
        input_number.insert(7,' ');
        input_number.insert(9,' ');

        //println!("{}",input_number);

        if input_number.len() != 12 {
            panic!("Your number must be 6 digits long!, Your number is {} digits long",input_number.len());
        }


        check_lucky(input_number);
    }


}


// seperate the input into six values 
// catch exeption if the input if over 6 
//add sum of first 3
// then sum of second 3
//rememeber to use .trim() when comparing

//get the first 3 values of the map, add them together
//same with last 3
