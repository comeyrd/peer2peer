Client has to initiate a connection to an unrestricted server, which notes endpoint and session information.

Client receives their public address from the server they can initiate connection with a peer.

Then matchmaking happens and the client receives endpoint and session information from the other client.

Each client tries to connect to its peer through the specified IP address and port that the peer's firewall has opened for the server.

==> Hole can be punched by sending a small UDP packet directly. For TCP it's harder. with ICMP it's simpler.

Successful exchange of an *authentication nonce* between both clients indicates the completion of a hole punching procedure.