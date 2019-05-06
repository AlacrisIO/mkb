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

MINIMAL 
---Check data that the data has the correct hash.
---At a given address only one block.

How much does the MKB checks?

OPEN STATE CHANNEL between us and the MKB.
ERC20 token for Ethereum for our MKB.
Our own currency on the MKB.

"Discussion of MKB validation support as DSL output"
All sidechain will be different.
DSL that describe the structure.

Assign people whom you trust in the validation.
Language for validation (Can be trivial).

S-expressions for storing the data.


Blind version of the MKB
 * MKB has topics. Topics specify maximum number of item per second.
 * Within topic a notion of content address.
 * Something similar to Bitcoin address.
 * Cannot publish anything at a given address.
 * Fight double spending: we make a new operation at an existing address.
   Only one new operation at the next block.
 * jsonrpc: topic, address, content itself.
   Just like bitcoin script: secret key for the address.
   Salt the secret key.

Data type with leaves (by Alex):
 * List of S-expression.
 *






Main author right now is Mathieu Dutour Sikiric <mathieu.dutour@gmail.com>
