fn main() {
    // Set database
    if let Err(e) = set_database(){
        eprintln!("Error setting database: {}", e);
        std::process::exit(1);
    }

    //Start server
    let listener = TcpListener::bind(format!("0.0.0.0:8080")).unwrap();
    println!("Server started on port 8080");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream);
            }
            Err(e) => {
                eprintln!("Unable to connect: {}", e);
                std::process::exit(1);
            }
        }
    }
}
