use types::*;
use merkle_data_tree::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct SimpleGossipProtocol {
    pub list_neighbor: Vec<usize>
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







pub fn compute_simple_gossip_protocol(common_init: CommonInit, address: String) -> SimpleGossipProtocol {
    let nb_reg = common_init.registrars.len();
    let mut the_vect = Vec::<usize>::new();
    for i_reg in 0..nb_reg {
        let addr_reg = &common_init.registrars[i_reg];
        if addr_reg.address != address {
            the_vect.push(i_reg);
        }
    }
    SimpleGossipProtocol { list_neighbor: the_vect }
}


fn check_transaction(registrar: SingleRegistrar) -> bool {
    true
}




pub fn propagate_info(common_init: CommonInit, sgp: SimpleGossipProtocol) -> bool {
    let nb_neigh = sgp.list_neighbor.len();
    for i_neigh in 0..nb_neigh {
        let i_reg = sgp.list_neighbor[i_neigh];
        let eval = check_transaction(common_init.registrars[i_reg].clone());
        if eval == false {
            return false;
        }
    }
    return true;
}
