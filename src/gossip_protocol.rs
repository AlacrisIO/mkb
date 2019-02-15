use types;

#[derive(Debug, Serialize, Deserialize)]
pub struct SimpleExportScheme {
    pub list_neighbor: Vec<String>
}


#[derive(Debug, Serialize, Deserialize)]
pub struct RoutingLine {
    pub list_direct_neighbor: Vec<String>
}


pub struct GossipProtocol {
    pub list_routing_line: Vec<RoutingLine>,
    pub initial_address: String
}

pub struct Message {
}



pub fn compute_gossip_protocol(common_init: types::CommonInit, address: String) -> GossipProtocol {
    let len = address.len();
    let mut the_vect = Vec::new();
    for i in 0..len {
        let esub_str = &address[0..i];
        let get_routing_line = || {
            for eval in common_init.clone().registrars {
                if address != eval.address {
                    let eval_str = &eval.address[0..i];
                    if eval_str == esub_str {
                        return vec![eval.clone().address];
                    }
                }
            }
            vec![]
        };
        let e_line: RoutingLine = RoutingLine { list_direct_neighbor: get_routing_line() };
        the_vect.push(e_line);
        
    }
    GossipProtocol { list_routing_line: the_vect, initial_address: address }
}


pub fn propagate_info(common_init: types::CommonInit, gp: GossipProtocol) {
    
    
}
