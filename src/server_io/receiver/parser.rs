use lexpr::Value;

pub fn parse_into_struct(parsed_expr: lexpr::Value) {
    match parsed_expr {
        lexpr::Value::Cons(value) => {
            let message_name = value.car().as_symbol().unwrap();
            match message_name {
                "sense_body" => parse_sense_body(),
                "see" => println!("See message!"),
                "hear" => println!("Hear message!"),
                "server_param" => println!("Server Param message!"),
                "player_param" => println!("Player Param message!"),
                "player_type" => println!("Player Type message!"),
                "init" => println!("Init message!"),
                &_ => println!("Unknown message {}!", message_name),
            }
        }
        _ => panic!(),
    }
}

fn parse_sense_body() {

}
