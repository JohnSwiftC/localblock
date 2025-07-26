an experiment

# Protocol

Localblock is split between a client and server application that can run on several different machines.

Authority servers for a specific currency / network must all be completely aware of eachother. Each node communicates with every other node in the authority network.

> I am aware that this raises obvious scaling issues, but this is very managable for lower node counts.

Transactions are verified by a sender's signature of a transaction to another's public key. This transaction, made from the client, can be sent to either one or many of the authority servers.

> Here is a good stop to point out that this is nothing like BitCoin, many networks with the same protocol can be set up and controlled by an organization, it is not peer-to-peer, and it requires a certain amount of trust in the people behind the network. However, it does not require near the same amount of compute that BitCoin and similar peer-to-peer cryptocurrencies do. Given that you have a group of friends you can trust, this could be a fun toy to mess around with.