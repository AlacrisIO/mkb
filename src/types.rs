use std::vec;

#[derive(Debug)]
struct SingleRegistrar {
    name: String,
    address: String,
    public_key: String,
    port: i32
}


struct CommonInit {
    registrars: Vec<SingleRegistrar>
}


struct SingleEnt {
    name: String,
    address: String,
    public_key: String,
    private_key: String,
    password: String
}


    
