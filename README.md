The Mutual Knowledge base

The goal of this repository is to implement the MKB in Rust.
This follows the MKB document.


Design of the system:
* Data is signed via Ed25519
* Data is spread via the gossip protocol.
* The data is received 



Gossip protocol works as follows:
* Data is emitted from one source and send to others.
* Neighbors are determined by their address using the classical DHT procedure.
* This can be made random via the VRF technology.
* Single packet are sent which contains a single request.
* They must also contains the signature of the transactions.
* We send single packet. They are sent to all the neighbors.







Main author right now is Mathieu Dutour Sikiric <mathieu.dutour@gmail.com>
