#set page(
  paper: "a4",
  header: align(left)[
    The Sondbus Protocol Suite
  ],
)

#show title: set text(size: 50pt)
#show heading: set text(size: 20pt)
#show heading.where(level: 1): set text(size: 35pt)
#show heading.where(level: 2): set text(size: 25pt)

#set text(
  size: 12pt,
)

#set heading(
  numbering: "1.1.1.",
  outlined: true,
)

#set par(
  justify: true,
)

#show table.cell.where(y: 0): strong

#title[The Sondbus Protocol Suite]
#pagebreak()
#outline()
#pagebreak()

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

The Sondbus Transfer Protocol is designed to be able to work for a lot of network topologies, be it bus, star or other networks.
This flexibility makes this transfer layer a useful tool, even if other protocols than the rest of the sondbus suite are run over it.

=== Immediate Transfers

The immediate operating mode may be chosen on lower layers that have a single shared connection to the master, be it due to there being only one slave or all slaves sharing a common bus line that they can use to talk to the master.
Normally, such connections require one device to take over the bus in order to communicate on it.
This implies that communication must be coordinated in order to avoid collisions.
In this mode, the master and slave take turns on using the communication line and talking on it.
For commands that contain a slave response, the master yields the communication line to the slave that is responding during the slots of the response or the slots that the slave must fill.

This mode can achieve very low latency, as the requests and responses of master and slaves are interleaved.
Additionally, this mode can yield very precise timing and exceptional jitter performance.

=== Deferred Transfers

The deferred operating mode can be used for lower layers that do not allow single bytes to be sent on a bus, or where the physical medium is not shared between slaves, but rather used in a peer-to-peer manner.
In this mode, the master sends a complete frame or command to the first slave, which then forwards it to its next connected slave and so on.
In this mode, the frame or command travels through the network and gets manipulated by the slaves that it passes through, depending on whether they are addressed by it or not.
Once the roundtrip is completed, the first slave sends the frame back to the master, which can then examine and use the data that is coming back.

== Sondbus over Ethernet

Sondbus can operate on top of Ethernet in the SoE (Sondbus over Ethernet) mode.
This mode runs Sondbus in the `deferred` and `framed` mode, as Ethernet does not allow for single-byte access to the bus. Additionally frames can only be sent as a whole, making the protocol more efficient when multiple commands are bundled.

#pagebreak()

= Commands

The sondbus protocol works with commands that can be sent on their own or packed in a frame in framed mode.
A command always starts with a command byte that identifies the command and describes the following data.



#table(
  columns: 2,
  table.header([Bit Position], [Description]),
  [0], table.cell(rowspan: 6, [Command Set Specific]),
  [1], [2], [3], [4], [5], [6],
  [Command Set
    #linebreak()
    0 => #link(<command-management-command-set>)[Management Command Set]
    #linebreak()
    1 => #link(<command-memory-command-set>)[Memory Command Set]
  ],
  [7], [Toggle Bit],
)

== Management Command Set <command-management-command-set>


#table(
  columns: 2,
  table.header([Bit Position], [Description]),
  [0], table.cell(rowspan: 6, [Management Command ID]),
  [1], [2], [3], [4], [5], [6],
  [Command Set
    #linebreak()
    0 => #link(<command-management-command-set>)[Management Command Set]
    #linebreak()
    1 => #link(<command-memory-command-set>)[Memory Command Set]
  ],
  [7], [Toggle Bit],
)

#pagebreak()

== Memory Command Set <command-memory-command-set>

The memory command set facilitates reading from and writing to a slave memory over the fabric. This is the core of this protocol suite in that is builds the foundation that is used to create the core link between the master and its slaves.


#table(
  columns: 4,
  table.header([Octet Position], [Size], [Initiator], [Description]),
  [0], [1], [Master], [#link(<memory-command-set-command-octet>)[Command Octet]],
  [1], [0-6], [Master], [Slave Address],
  [?], [1-8], [Master], [Offset],
  [?], [1-8], [Master], [Length],
  [?], [1], [Master], [Header CRC],
  [?], [?], [Master / Slave], [Payload],
  [?], [1], [CRC],
)


=== Command Octet <memory-command-set-command-octet>

The command octet for commands in the memory command set takes the following structure:

#table(
  columns: 2,
  table.header([Bit Position], [Description]),
  [0],
  table.cell(rowspan: 2, [#link(<memory-command-set-memory-addressing-mode>)[Memory Addressing Mode]]),
  [1],

  [2],
  table.cell(rowspan: 2, [#link(<memory-command-set-slave-addressing-mode>)[Memory Addressing Mode]]),
  [3],
  [4],
  [#link(<memory-command-set-operation>)[Operation]],
  [5], [Command Set = *1*],
  [6], table.cell(rowspan: 2, [Sequence Counter]), [7],
)

==== Operation <memory-command-set-operation>

The operation bit indicates to the slave whether the operation is to read from or to write to memory.

- `0` => Read from slave memory to the command
- `1` => Write from the command to the slave memory

==== Slave Addressing Mode <memory-command-set-slave-addressing-mode>

- `0b00` => *Broadcast*
- `0b01` => *Physical* (By the MAC address of the slave)
- `0b10` => *Logical* (By the logical address of the slave)
- `0b11` => *Virtual* using the MMUs to map physical memory to the fieldbus memory

==== Memory Addressing Mode <memory-command-set-memory-addressing-mode>

- `0b00` => 8 bit offset and length
- `0b01` => 16 bit offset and length
- `0b10` => 32 bit offset and length
- `0b11` => 64 bit offset and length

=== Slave Address <memory-command-slave-address>

=== Offset <memory-command-offset>

=== Length <memory-command-length>

=== Header CRC <memory-command-header-crc>

=== Payload <memory-command-payload>
