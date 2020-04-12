use clap::{App, load_yaml};

fn main() {
    // The YAML file is found relative to the current file, similar to how modules are found
    let yaml = load_yaml!("cli.yml");
    let matches = App::from(yaml).get_matches();

    // matches.subcommand(&cmp);
    let sub_cmd_name = matches.subcommand_name().unwrap();

    if !sub_cmd_name.trim().is_empty() {
        let args_match = matches.subcommand_matches(sub_cmd_name).unwrap();

        // Process create command
        if sub_cmd_name == "create" {
            let app_name = args_match.value_of_os("INPUT").unwrap();
            let is_srv = args_match.is_present("srv");
            let is_api = args_match.is_present("api");
            let is_func = args_match.is_present("func");
            let is_graph = args_match.is_present("graph");

            println!("Binary name: {:?}", app_name);
            println!("Binary name: {:?}", is_srv);
            println!("Binary name: {:?}", is_api);
            println!("Binary name: {:?}", is_graph);
            println!("Binary name: {:?}", is_func);

            if is_srv {

            } else if is_api {

            }

        }
    }
}
