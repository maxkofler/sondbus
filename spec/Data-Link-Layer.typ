= The Data Link Layer <data_link_layer>

This section of the document describes the data link layer of the sondbus protocol suite.
It is in essence a protocol that allows a master instance to read and write to memory of its slaves.
Each read or write transaction is contained in a separate command that is sent by the master and then processed by all slaves.
There are various addressing modes to keep the protocol minimal in the data that is sent over the wire, but yet powerful and flexible to allow for different forms of communication.

== Processing and commanding

Sondbus allows for 2 main ways of transporting commands across the network to support all kinds of network architectures and types.

The lowest requirement for the sondbus data link layer to work is that a device is capable to send at least individual octets.
This should make sondbus fairly portable and minimal on requirements to the physical medium.

=== Shared Medium

Sondbus can run on a shared medium where commands are received by one or more slaves at a time and the communication lines are shared for all devices.
This means that all devices need to take turns on communicating on the line and precise coordination is required to avoid collisions.

=== Peer To Peer

Sondbus can also work in a peer to peer manner in which it uses on-the-fly processing of frames or commands.
In this mode, the master has a direct connection only to the first slave in the network and the following slaves are connected through each other.

== Command Structure


