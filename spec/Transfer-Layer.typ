#import "@preview/tasteful-pairings:0.1.0": *
#let pairing = font-pairings.at("friendly-weather")

#set page(
  paper: "a4",
  margin: (top: 3cm, bottom: 2cm, x: 1.5cm),
  header: align(left)[
    The Sondbus Transfer Layer
  ],
)

#show title: set text(size: 50pt)
#show heading: set text(size: 20pt, font: pairing.heading)
#show heading: set text(size: 13pt)
#show heading.where(level: 1): set text(size: 20pt)
#show heading.where(level: 2): set text(size: 15pt)

#set text(
  size: 10pt,
  font: pairing.body,
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

#let optional_feature() = {
  link(<optional-features>)[\[optional\]]
}

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

== Endianness

This protocol sticks to the convention of using big-endian notation for multi-octet values in networking.
This has been chosen due to the following reasons:

- Compatibility with network Endianness
- Legibility in network traces

== Addressing

The transfer layer supports the following addressing modes:

- #link(<addressing-broadcast>)[Broadcast]
- #link(<addressing-physical>)[Physical]
- #link(<addressing-logical>)[Logical]
- #link(<addressing-virtual>)[Virtual] #optional_feature()

=== Broadcast <addressing-broadcast>

Some operations can target all devices in the network.
In this case, the broadcast addressing scheme is convenient. This is especially useful in the initialization phase where it is necessary to bring all slaves to a common and known state.

=== Physical <addressing-physical>

Physical addressing uses each slave's unique MAC address to identify the targeted slave.
The MAC address consists of 6 octets (48 bit) addresses.

=== Logical <addressing-logical>

The logical addressing scheme allows for shorter messages.
Logical addresses get distributed by the master to create a logical address space that is smaller than the physical address space.

The logical address is a 16 bit address.

=== Virtual <addressing-virtual>

This protocol also allows for a virtual memory map to be used.
This greatly improves efficiency, as the master must no longer address each slave individually, but it can rather issue a single command to address multiple slaves.
More information on the virtual memory feature can be found in the #link(<virtual-memory>)[Virtual Memory] section of this document.

This is an #optional_feature() feature.

#pagebreak()

= Transfer modes <transfer-modes>

The Sondbus Transfer Protocol is designed to be able to work for a lot of network topologies, be it bus, star or other networks.
This flexibility makes this transfer layer a useful tool, even if other protocols than the rest of the sondbus suite are run over it.

The transfer mode is determined by the following two factors:

- Framed(*`F`*) or Unframed(*`U`*) Transfers
- Immediate(*`I`*) or Deferred(*`D`*) Transfers

These two factors form 4 possibilities the sondbus protocol can run on:

- *`FI`*: Framed Immediate
- *`FD`*: Framed Deferred
- *`UI`*: Unframed Immediate
- *`UD`*: Unframed Deferred

== Immediate Transfers

The immediate operating mode may be chosen on lower layers that have a single shared connection to the master, be it due to there being only one slave or all slaves sharing a common bus line that they can use to talk to the master.
Normally, such connections require one device to take over the bus in order to communicate on it.
This implies that communication must be coordinated in order to avoid collisions.
In this mode, the master and slave take turns on using the communication line and talking on it.
For commands that contain a slave response, the master yields the communication line to the slave that is responding during the slots of the response or the slots that the slave must fill.

This mode can achieve very low latency, as the requests and responses of master and slaves are interleaved.
Additionally, this mode can yield very precise timing and exceptional jitter performance.

== Deferred Transfers

The deferred operating mode can be used for lower layers that do not allow single bytes to be sent on a bus, or where the physical medium is not shared between slaves, but rather used in a peer-to-peer manner.
In this mode, the master sends a complete frame or command to the first slave, which then forwards it to its next connected slave and so on.
In this mode, the frame or command travels through the network and gets manipulated by the slaves that it passes through, depending on whether they are addressed by it or not.
Once the roundtrip is completed, the first slave sends the frame back to the master, which can then examine and use the data that is coming back.

== Unframed Transfers

Unframed transfers assume raw, octet-level access and control over the medium.
In this mode, the protocol takes full responsibilty of framing, bus management etc.

== Framed Transfers

In the framed transfer mode, the protocol assumes a lower layer that takes responsibility on when a new transmission cycle starts.
In this mode, the protocol does not provide facilities for detecting the start and end of a message, because it assumes that is handled by the lower layer that provides the frame mechanism.

#pagebreak()

= Message <message>

The sondbus protocol works with messages that can be sent on their own or packed in a frame in framed mode.
A message always starts with a command that identifies the message and describes the following data, optionally some payload and finally a CRC to detect transmission errors.
Each message has the following basic structure:


#table(
  columns: (6.3em, 1fr, 1fr, 1fr),
  [Position], [0], [1], [2],
  [Size in Bits], [8], [n*8], [8],
  [Description], [#link(<message-command>)[Command]], [#link(<message-payload>)[Payload]], [#link(<message-crc>)[CRC]],
)

== Command <message-command>

The *Command* octet has the following structure:

#table(
  columns: (6.3em, 1.5fr, 1.5fr, 3fr, 1fr, 1fr, 1fr, 1fr, 1fr),
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
  columns: (6.3em, 1.5fr, 1.5fr, 3fr, 1fr, 1fr, 1fr, 1fr, 1fr),
  [Position], [7], [6], [5], [4], [3], [2], [1], [0],
  [Description],
  table.cell(colspan: 2, [#link(<message-command-sequence-counter>)[Sequence Counter]]),
  [#link(<command-set-bit>)[Command Set = *0*]],
  table.cell(colspan: 5, [Management Command ID]),
)

=== 0x00 - NOP <management-command-nop>

The NOP command is, as the name implies a no operation command that does absolutely nothing.
It can be used by the master to determine no-execution round trip times or similar things.
A requirement of this message type is that no slave shall have any side-effects from receiving this message.

#table(
  columns: (6.3em, 1fr, 1fr),
  [Msg Part], [#link(<message-command>)[Command]], [#link(<message-crc>)[CRC]],
  [Octets], [1], [1],
  [Initiator], [M], [M],
  [Description], [Command], [CRC],
)

=== 0x01 - Sync <management-command-sync>

The sync command is used to bring the state machine of the tap into the *Synchronized* state.
This state is required for any other operation to be enabled.
The sync command consists of the command octet followed by the following hex sequence:

```hex
1F 2E 3D 4C 5B 6A 79 88 97 A6 B5 C4 D3 E2 F1
```

#table(
  columns: (6.3em, 3fr, 1fr, 1fr, 1fr, 1fr, 1fr, 1fr, 1fr, 1fr, 1fr, 1fr, 1fr, 1fr, 1fr, 1fr, 1fr, 1fr),
  [Msg Part],
  [#link(<message-command>)[Command]],
  table.cell(colspan: 15, link(<message-payload>)[Payload (Hexadecimal - 0x)]),
  [#link(<message-crc>)[CRC]],
  [Initiator], [M], [M], [M], [M], [M], [M], [M], [M], [M], [M], [M], [M], [M], [M], [M], [M], [M],
  [Value],
  [`0b??00_0001`],
  [1F],
  [2E],
  [3D],
  [4C],
  [5B],
  [6A],
  [79],
  [88],
  [97],
  [A6],
  [B5],
  [C4],
  [D3],
  [E2],
  [F1],
  [?],
  [Description], [Command], table.cell(colspan: 15, [Sync Sequence]), [CRC],
)

No slave ever responds to this command, as it is used in pure broadcast fashion to synchronize up all slaves. This command can also be repeated multiple times to ensure out-of-sync slaves re-join the network correctly.

```rust
const SYNC_SEQUENCE: [u8; 15] = [0x1F, 0x2E, 0x3D, 0x4C, 0x5B, 0x6A, 0x79, 0x88, 0x97, 0xA6, 0xB5, 0xC4, 0xD3, 0xE2, 0xF1];
```

=== 0x02 - Feature Query

#pagebreak()

== Memory Command Set <command-memory-command-set>

The memory command set facilitates reading from and writing to a slave memory over the fabric. This is the core of this protocol suite in that is builds the foundation that is used to create the core link between the master and its slaves.

#table(
  columns: (6.3em, 1fr, 1fr, 1fr, 1fr, 1fr, 1fr, 1fr),
  [Msg Part],
  [#link(<message-command>)[Command]],
  table.cell(colspan: 5, [#link(<message-payload>)[Payload]]),
  [#link(<message-crc>)[CRC]],
  [Octets], [1], [0-6], [1-8], [1], [1], [n], [1],
  [Initiator], [M], [M], [M], [M], [M], [M/S], [M/S],
  [Description],
  [Command],
  [#link(<memory-command-slave-address>)[Slave#linebreak()Address]],
  [#link(<memory-command-offset>)[Offset]],
  [#link(<memory-command-length>)[Length]],
  [#link(<memory-command-header-crc>)[Header#linebreak()CRC]],
  [#link(<memory-command-payload>)[Payload]],
  [CRC],
)

=== Command <memory-command-set-command>

The command octet for messages in the memory command set takes the following structure:

#table(
  columns: (6.3em, 2.35em, 2.35em, 6em, 1.3fr, 1fr, 1fr, 1fr, 1fr),
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

For more infomation on the slave addressing mode, see the #link(<memory-command-slave-address>)[slave address] section.

- `0b00` => *Broadcast*
- `0b01` => *Virtual* using the MMUs to map physical memory to the fieldbus memory #optional_feature()
- `0b10` => *Physical* (By the MAC address of the slave)
- `0b11` => *Logical* (By the logical address of the slave) #optional_feature()

==== Memory Addressing Mode <memory-command-set-memory-addressing-mode>

- `0b00` => 8 bit offset
- `0b01` => 16 bit offset #optional_feature()
- `0b10` => 32 bit offset #optional_feature()
- `0b11` => 64 bit offset #optional_feature()

=== Slave Address <memory-command-slave-address>

The slave address specifies which slave this message targets.
Depending on the #link(<memory-command-set-slave-addressing-mode>)[slave addressing mode], this field can vary in length.

The field has the following length depending on the mode used:

- Broadcast: 0 Octets
- Physical: 6 Octets
- Logical: 2 Octets
- Virtual: 0 Octets

=== Offset <memory-command-offset>

The *Offset* field encodes the offset in the slave's or virtual address space to read or write at.
The length of this field is dictated by the #link(<memory-command-set-memory-addressing-mode>)[Memory Addressing Mode] selected in the #link(<memory-command-set-command>)[Command].

=== Length <memory-command-length>

This field encodes the amount of data in octets in the slave's or virtual address space to read or write.
This field is fixed at 1 octet. The protocol is designed to allow a maximum of 255 octets to be transferred in one message.
This is to ensure that the CRC can provide adequate protection against errors and to make implementations simple.
Larger transfers can be split into multiple messages.

=== Header CRC <memory-command-header-crc>

The header CRC confirms and error-checks the fields leading up to this octet (Command, Slave Address, Offset, Length) to indicate to the slave that the operation is valid. This is especially important for read operations where the slave writes data to the system after this octet.

=== Payload <memory-command-payload>

The payload field contains the data that is to be written to the memory or gets filled with the data read from the slave's memory.

For a write transaction, this field is filled by the Master, otherwise by the Slave.

= Tap <tap>

The tap is the component of the stack that is concerned with processing messages and forwarding them to the application.
Its name stems from the similarity to old Ethernet (ThickNet) tap that would tap into the line and hook up a device to a network.
This is true for the Transfer Layer, too, as the tap provides the connection to the network and allows the network to access the memory area of the network participant.

== State Machine <tap-state-machine>

As this protocol is designed to run on unframed media, there is the need for synchronization in order to determine whether the currently received octet can be interpreted correctly.
This protocol solves that by using a simple state machine that syncs a tap to the network via a dedicated message.

If the tap encounters a critical error, it will fall out of sync and wait for a new sync to be initiated.
More details on how the tap handles errors is explained in #link(<tap-error-handling>)[error handling].

The tap state machine has the following two states:

- Out of Sync
- In Sync

== Error Handling <tap-error-handling>

= Virtual Memory <virtual-memory>

= Frames

Frames can bundle multiple commands to increase efficiency on framed protocols.
Frames are required when running the bus over Ethernet for example.
In this case, the underlying layer does not allow for individual octets to be sent.
To increase the efficiency on such lower layers, a frame provides a mechanism for bundling multiple commands to be shipped on one frame.

= Optional Features <optional-features>

This protocol foresees the use of optional features.
This mechanism allows this protocol to be used on very constrained system by supporting a minimal set of features.

If a slave does not support a feature, the master must not use it with this slave.
It is, however ok to use the feature on an other slave.
Each slave must "understand" the whole protocol to a point where using a protocol feature does not raise any errors in a device that does not support this feature.

One example of this are the different #link(<memory-command-set-slave-addressing-mode>)[Addressing Modes] in the #link(<memory-command-set-command>)[Memory Command Set].
A slave may not support 32 and 64 bit addressing, but despite that, it must still be able to parse a command that uses these two addressing modes.
This comes from the fact that all slaves on a bus must understand all forms and messages.
