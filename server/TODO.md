The server has to track for each client :
- public IP and port : public address
- private IP and port : private address

When a client initiates connection, send back their public IP to them.

Then matchmaking based on ...

When client A wants to connect to client B and vice versa:
- forward client B public address to client A
- forward client A public address to client B

Once hole is succesfully punched : drop the info ?

