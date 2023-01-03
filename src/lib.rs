use std::io::Write;
use std::string::FromUtf8Error;

pub enum UserSelectedAmount {
    LIGHT(f64),
    NORMAL(f64),
    EXTRA(f64)
    // f64 = Add'l cost
}

pub struct Topping {
    amount: UserSelectedAmount,
    base_cost: f64,
    name: String,
}

impl Topping {
    pub fn get_base_cost(&self) -> f64 {
        self.base_cost
    }

    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }

    pub fn get_amount(&self) -> &UserSelectedAmount {
        return &self.amount;
    }
}

pub struct Pizza {
    toppings: Vec<Topping>,
    base_cost: f64
}

impl Pizza {
    pub fn get_total_cost(&self) -> f64 {
        let mut total_cost = self.base_cost;
        for x in &self.toppings {
            total_cost += x.get_base_cost();
        };
        return total_cost;
    }

    pub fn add_topping(&mut self, topping: Topping) {
        self.toppings.push(topping);
    }

    pub fn remove_last_topping(&mut self) -> Option<Topping> {
        let popped = self.toppings.pop();
        self.toppings.shrink_to_fit();
        popped
    }

    pub fn gen_str(&self) -> String {
        let mut buf = Vec::new();
        if self.has_toppings() {
            let s = "Pizza with ";
            write_to_buf(&mut buf, String::from(s));
            for topping in &self.toppings {
                write_to_buf(&mut buf, String::from(topping.get_name()));
                write_to_buf(&mut buf, String::from(", "))
            }
            buf.truncate(buf.len() - 2);
            buf.shrink_to_fit();
            let result = String::from_utf8(buf);
            match result {
                Ok(fmtted) => {
                    fmtted
                }
                Err(_) => {
                    String::from("Well shit, the pizza fell on the floor.")
                }
            }
        } else {
            return String::from("Plain Pizza")
        }
    }

    pub fn has_toppings(&self) -> bool {
        self.toppings.len() > 0
    }
}



pub struct PizzaOrder {
    pizzas: Vec<Pizza>,
    customer_name: String
}

pub fn build_basic_pizza(base_cost: f64) -> Pizza {
    Pizza {
        toppings: vec![],
        base_cost,
    }
}

pub fn build_topping(base_cost: f64, amount: UserSelectedAmount, name: String) -> Topping {
    Topping {
        amount,
        base_cost,
        name,
    }
}

fn write_to_buf(mut buf: &mut Vec<u8>, to_write: String) {
    for byte in to_write.into_bytes() {
        buf.push(byte)
    }
}