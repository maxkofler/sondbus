#set page(
  paper: "a4",
  margin: (top: 3cm, bottom: 2cm, x: 1.5cm),
  header: align(left)[
    The Sondbus Transfer Layer
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

#show link: it => {
  set text(blue)
  if type(it.dest) != str {
    it
  } else {
    underline(it)
  }
}

#show table.cell.where(x: 0): strong
#show table.cell: set align(center + horizon)
#show table.cell.where(x: 0): set align(left + horizon)

#title[The Sondbus#linebreak() Transfer Layer]
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

= Message <message>

The sondbus protocol works with messages that can be sent on their own or packed in a frame in framed mode.
A message always starts with a command that identifies the message and describes the following data, optionally some payload and finally a CRC to detect transmission errors.
Each message has the following basic structure:


#table(
  columns: (6.2em, 1fr, 1fr, 1fr),
  [Position], [0], [1], [2],
  [Size in Bits], [8], [n*8], [8],
  [Description], [#link(<message-command>)[Command]], [#link(<message-payload>)[Payload]], [#link(<message-crc>)[CRC]],
)

== Command <message-command>

The *Command* octet has the following structure:

#table(
  columns: (6.2em, 1.5fr, 1.5fr, 3fr, 1fr, 1fr, 1fr, 1fr, 1fr),
  [Position], [7], [6], [5], [4], [3], [2], [1], [0],
  [Description],
  table.cell(colspan: 2, [#link(<message-command-sequence-counter>)[Sequence Counter]]),
  [#link(<command-set-bit>)[Command Set]],
  table.cell(colspan: 5, [Command Set Specific]),
)

=== Sequence Counter <message-command-sequence-counter>

The sequence counter allows slaves and the master to keep track of messages and detect missed ones. The counter is a simple 2-bit counter that is incremented sequentially by one for each message that is sent by the master. The sequence counter also gets incremented for each message in a frame.

The sequence shall cycle through the following states: `0b00`, `0b01`, `0b10`, `0b11`, reapeating the sequence when it is finished.

=== Command Set Bit <command-set-bit>

The `Command Set Bit` indicates which one of the two available command should be used with this command to infer how to interpret the `Command Set Specific` bits:

- `0` => #link(<command-management-command-set>)[Management Command Set]
- `1` => #link(<command-memory-command-set>)[Memory Command Set]

== Payload <message-payload>

The payload field is the field that contains the data that is conveyed through a message.
This field is optional in that it can consist of 0 octets for commands that do not contain any data to be conveyed or the command itself is enough information in order to perform the action.

== CRC <message-crc>

The CRC field provides an error detection mechanism.
The CRC is calculated over the whole message (#link(<message-command>)[Command] + #link(<message-payload>)[Payload]) to ensure the correct command and payload are received and can be processed as intended.

The CRC is of type `CRC8-Autosar`.

#pagebreak()

== Management Command Set <command-management-command-set>

#table(
  columns: (6.2em, 1.5fr, 1.5fr, 3fr, 1fr, 1fr, 1fr, 1fr, 1fr),
  [Position], [7], [6], [5], [4], [3], [2], [1], [0],
  [Description],
  table.cell(colspan: 2, [#link(<message-command-sequence-counter>)[Sequence Counter]]),
  [#link(<command-set-bit>)[Command Set = *0*]],
  table.cell(colspan: 5, [Management Command ID]),
)

=== 0x00 - Sync <management-command-sync>

The sync command is used to bring the state machine of the transceiver into the *Synchronized* state. This state is required for any other operation to be enabled. The sync command consists of the command octet followed by the following hex sequence:

```hex
1F 2E 3D 4C 5B 6A 79 88 97 A6 B5 C4 D3 E2 F1
```

No slave ever responds to this command, as it is used in pure broadcast fashion to synchronize up all slaves. This command can also be repeated multiple times to ensure out-of-sync slaves re-join the network correctly.

#pagebreak()

== Memory Command Set <command-memory-command-set>

The memory command set facilitates reading from and writing to a slave memory over the fabric. This is the core of this protocol suite in that is builds the foundation that is used to create the core link between the master and its slaves.


#table(
  columns: (6.2em, 1fr, 1fr, 1fr, 1fr, 1fr, 1fr),
  [Octets], [1], [0-6], [1-8], [1-8], [1], [n],
  [Initiator], [M], [M], [M], [M], [M], [M/S],
  [Description],
  [#link(<memory-command-set-command-octet>)[Command#linebreak()Octet]],
  [Slave#linebreak()Address],
  [Offset],
  [Length],
  [Header#linebreak()CRC],
  [Payload],
)

=== Command Octet <memory-command-set-command-octet>

The command octet for commands in the memory command set takes the following structure:

#table(
  columns: (6.2em, 2.35em, 2.35em, 6em, 1.3fr, 1fr, 1fr, 1fr, 1fr),
  [Position], [7], [6], [5], [4], [3], [2], [1], [0],
  [Description],
  table.cell(colspan: 2, [#link(<message-command-sequence-counter>)[Sequence Counter]]),
  [#link(<command-set-bit>)[Command Set = *1*]],
  [#link(<memory-command-set-operation>)[Operation]],
  table.cell(colspan: 2, [#link(
    <memory-command-set-slave-addressing-mode>,
  )[Slave#linebreak()Addressing#linebreak()Mode]]),
  table.cell(colspan: 2, [#link(
    <memory-command-set-memory-addressing-mode>,
  )[Memory#linebreak()Addressing#linebreak()Mode]]),
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

= Transceiver <transceiver>

The transceiver is the component of the stack that is concerned with processing messages and forwarding them to the application.

== State Machine <transceiver-state-machine>

The transceiver state machine has the following two states:

- Out of Sync
- In Sync

= Frames

Frames can bundle multiple commands to increase efficiency on framed protocols. Frames are required when running the bus over Ethernet for example. In this case, the underlying layer does not allow for individual octets to be sent. To increase the efficiency on such lower layers, a frame provides a mechanism for bundling multiple commands to be shipped on one frame.

