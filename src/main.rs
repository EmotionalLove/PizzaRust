use std::fmt::format;
use untitled::{build_basic_pizza, build_topping, UserSelectedAmount};

fn main() {
    let pepperoni = build_topping(0.5, UserSelectedAmount::NORMAL(0.0), String::from("Pepperoni"));
    let mozz_cheese = build_topping(0.25, UserSelectedAmount::NORMAL(0.0), String::from("Mozzarella Cheese"));
    let tomato_sauce = build_topping(0.0, UserSelectedAmount::NORMAL(0.0), String::from("Tomato Sauce"));
    let bacon = build_topping(1.0, UserSelectedAmount::NORMAL(0.0), String::from("Bacon"));

    let mut my_pizza = build_basic_pizza(4.00);
    print(my_pizza.gen_str().as_str());
    my_pizza.add_topping(mozz_cheese);
    my_pizza.add_topping(tomato_sauce);
    my_pizza.add_topping(pepperoni);
    print(my_pizza.gen_str().as_str());
    let cost = my_pizza.get_total_cost();
    let it = format!("The pizza costs ${}, a steal of a deal, ain't it?", cost);
    print(it.as_str());
}

fn print(to_print: &str) {
    println!("{}", to_print)
}