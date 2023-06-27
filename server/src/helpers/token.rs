use rand::Rng;
use uuid::Uuid;

pub fn generate_troken() -> String {
    let pizza_ingriedients = vec!["cheese", "tomato", "pepperoni", "mushrooms", "olives", "onions", "pineapple", "ham", "bacon", "chicken", "beef", "sausage", "peppers", "spinach", "anchovies", "jalapenos", "garlic", "artichokes", "broccoli", "basil", "feta", "goat-cheese", "ricotta", "provolone", "parmesan", "asiago", "cheddar", "mozzarella", "romano", "blue-cheese", "gorgonzola", "swiss-cheese", "cheddar-cheese"];
    
    let mut pizza = String::new();
    let mut rng = rand::thread_rng();

    for i in 0..4 {
        let random_number = rng.gen_range(0..pizza_ingriedients.len());
        pizza.push_str(pizza_ingriedients[random_number]);
        if i != 3 {
            pizza.push_str("-");
        }
    }

    return pizza;
}

pub fn generate_uuid() -> String {
    let id = Uuid::new_v4();
    return id.to_string();
}