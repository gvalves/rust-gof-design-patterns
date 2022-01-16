use std::env;

use template_method::*;

pub fn main() {
    let path_buf = env::current_dir().unwrap();
    let curr_dir = path_buf.to_string_lossy();
    let curr_dir = format!("{}/template-method/src", curr_dir);
    let file_path_txt = format!("{}/files/customer.txt", curr_dir);
    let customer_data_parser_txt = CustomerDataParserTxt::new(&file_path_txt);

    let customers_data_txt = customer_data_parser_txt.fix_customer_data();
    println!("{:#?}", customers_data_txt);

    let file_path_json = format!("{}/files/customer.json", curr_dir);
    let customer_data_parser_json = CustomerDataParserJson::new(&file_path_json);

    let customers_data_json = customer_data_parser_json.fix_customer_data();
    println!("{:#?}", customers_data_json);
}
