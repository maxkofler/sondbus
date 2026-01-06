= The Transfer Layer

The transfer layer is the lowest layer of the sondbus protocol suite.
It is responsible for giving the master access to the memory regions of all the slaves it has attached.
At its core, this layer is a mechanism for reading from and writing to memory of remote devices in an efficient and reliable manner.

== Topology

The transfer layer has a fixed topology in that there is exactly one master in the system that issues commands to slaves which then react and respond to these commands.
The slaves themselves are never allowed to initiate communication on the bus by themselves without previous master activity.
This fixed hierarchy completely eliminates the problem of collisions and removes the need for error detection and correction mechanisms, as it is easily predictable and manageable when which device can take control of the bus and talk.

== Addressing

The transfer layer supports the following addressing modes:

- #link(<addressing_broadcast>)[Broadcast]
- Physical
- Logical
- Virtual

=== Broadcast <addressing_broadcast>

#pagebreak()

== Transfer modes

=== Immediate Transfers

The immediate operating mode may be chosen on lower layers that have a single connection to the master, be it due to there being only one slave or all slaves sharing a common bus line that they can use to talk to the master.
In this mode, the master and slave take turns on using the communication line and talking on it.
For commands that contain a slave response, the master yields the communication line to the slave that is responding during the slots of the response or the slots that the slave must fill.

This mode can achieve very low latency, as the request and response of master and slave are interleaved and no double transmission is required.

=== Deferred Transfers

The deferred operating mode can be used for lower layers that do not allow single bytes to be sent on a bus, or where the physical medium is not shared between slaves, but rather used in a peer-to-peer manner.
In this mode, the master sends a complete frame or command to the first slave, which then forwards it to its next connected slave and so on.
In this mode, the frame or command travels through the network and gets manipulated by the slaves that it passes through, depending on whether they are addressed by it or not.
Once the roundtrip is completed, the first slave sends the frame back to the master, which can then examine and use the data that is coming back.

The deferred operating mode can be used upon two underlying bus types:

- #link(<deferred_transfer_framed>)[Framed]
- #link(<deferred_transfer_unframed>)[Unframed]

==== Framed <deferred_transfer_framed>

==== Unframed <deferred_transfer_unframed>
