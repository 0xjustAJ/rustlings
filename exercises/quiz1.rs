// quiz1.rs
//
// This is a quiz for the following sections:
// - Variables
// - Functions
// - If
//
// damola is buying apples. The price of an apple is calculated as follows:
// - An apple costs 2 rustbucks.
// - If damola buys more than 40 apples, each apple only costs 1 rustbuck!
// Write a function that calculates the price of an order of apples given the
// quantity bought.
//
// No hints this time ;)

// Put your function here!
 fn calculate_price_of_apples(no_of_apples: i32)-> i32 {
  // let  no_of_apples: Vec<i32> = vec![0..100];
    
    /////// MY CODE /////////////
    
    // if no_of_apples < 40{
    //     no_of_apples * 2
    // } else if no_of_apples == 40{
    //     no_of_apples * 2
    // } else if no_of_apples > 40{
    //     no_of_apples * 1
    // } else {
    //     no_of_apples*0
    // }

    //////////// REFACTOR ////////
    
    /////// ANOTHER /////////////
    
    // if no_of_apples < 40 || no_of_apples == 40 {
    //     no_of_apples * 2
    // }else if no_of_apples > 40{
    //     no_of_apples * 1
    // }else{
    //     no_of_apples*0
    // }
    
    /////// BETTER /////////////
    
    if no_of_apples <= 40 {
        no_of_apples * 2
    }else if no_of_apples > 40{
        no_of_apples * 1
    }else{
        no_of_apples*0
    }

   }
     
 //}

// Don't modify this function!
#[test]
fn verify_test() {
    let price1 = calculate_price_of_apples(35);
    let price2 = calculate_price_of_apples(40);
    let price3 = calculate_price_of_apples(41);
    let price4 = calculate_price_of_apples(65);

    assert_eq!(70, price1);
    assert_eq!(80, price2);
    assert_eq!(41, price3);
    assert_eq!(65, price4);
}