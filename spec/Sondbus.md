# The Sondbus protocol v1.0.0

This is the specification for the sondbus protocol (from now on sondbus).
It is a simple and minimal protocol for facilitating communication of multiple controllers using a shared medium.

The goal is to create a solid bus that can be used in a cyclic manner for facilitating time-critical and reliable communication.

It works in a single-master principle in that a single device in the network takes on the role of the master that coordinates all communication.
This makes the whole network timing predictable and the slave implementations smaller.

Table of contents

- [Data Link Layer](#data-link-layer)
- [Frame Types](#frame-types)
- [Cyclic communication](#cyclic-communication)

# Data Link layer

The first layer in the sondbus protocol is the data link layer.
This is comparable to layer 2 of the OSI model and facilitates the transport of single frames over the medium.

This layer is also used for cyclic communication, as it provides a deterministic and minimal way for transmitting data.

In either way, all communication has to be transmitted via this channel.

## Frame layout

The main difference of sondbus is, that all communication is performed in a broadcasting manner.
This manifests itself in the frame layout in that there is only a source, but no destination address:

| Start | Type | Src | Length | Data  | CRC |
| :---: | :--: | :-: | :----: | :---: | :-: |
|   1   |  1   |  1  |   1    | 0-255 |  1  |

### Start

The start byte is always `0x55` to indicate to the other network participants that a new frame is being transmitted.

### Type

The type of data that is transmitted in this frame, see [frame types](#frame-types).

### Src

The source address where the frame is sent from.

Address `0` is reserved for the master.
All the other addresses are free to be assigned arbitrary values.

### Length

Indicates the length of the following data block.

### Data

The data to be transmitted with this message.
This can be arbitrary data or even nothing, depending on the frame type.

### CRC

All frames are checked for errors using a crc8 checksum.
The checksum to be used is the `AUTOSAR` flavor.

## Error handling

The inclusion of a CRC hints at the presence of error detection.
In contrast to OSI layer 2 there is no error correction on the data link layer.

As the communication is time-critical, a mismatched CRC will simply lead to the frame being dropped by all network participants.

# Frame types

- [`0x1.` Cyclic frames](#0x1_-cyclic-frames)

## 0x1\_ Cyclic frames

This is probably the most common frame class.
Frames in this class describe frames that are used in the cyclic communication part of sondbus.

### 0x10 Cyclic request

This frame is sent by the master and initiates a new cycle.

The payload is the sequence the master asks the slaves to respond,
where the first byte is the sequence number of the cycle.

Each slave stores its position in this queue and responds once it is its turn.

### 0x11 Cyclic response

A possible response to the [cyclic request](#0x10-cyclic-request).

The slave inserts the agreed-upon data into the data area of the frame and sends it, reserving the first byte in the response for the sequence number of the cycle that it responds to.

### 0x12 Cyclic skip

A possible response to the [cyclic request](#0x10-cyclic-request).

The slave skips this cycle due to internal reasons, inserting the sequence number of the cycle as the first and only byte into the data section.

This response contributes 1 to the slave's fail counter, counting as if it timed out.

### 0x13 Cyclic abort

A possible response to the [cyclic request](#0x10-cyclic-request).

A slave actively refuses to respond to the cyclic request and aborts the cycle.
All further communication will not happen and the cycle will not be completed, skipping all following slaves.

The first and only byte in the data section is the sequence number of the cycle to be aborted.

This response contributes 1 to all the slave's fail counter, counting as if they timed out.

> [!CAUTION]
>
> This response is actively hurting bus performance and should only be used when absolutely necessary.
> Too many aborts can completely fail the cyclic communication and lead the master to abort all communication.

### 0x14 Cyclic exit

A possible response to the [cyclic request](#0x10-cyclic-request).

The slave asks to be excluded from all further cyclic communication.
This is a clean way for a slave to exit the cyclic communication and tell the master to not include it in further cycles.

The first and only byte in the data section of the frame is the sequence number of the responded-to cyclic request.

> [!NOTE]
>
> It is up to the master when to exclude the slave from the cyclic request frames.
> After the first exit request, the slave must continue communication with exit responses until it is excluded from the cyclic request frames, allowing it to safely leave the bus.

### 0x1D Cyclic configuration acknowledge

Acknowledges the configuration received via a [cyclic configuration](#0x1e-cyclic-configuration) frame.

The data section is mirrored from the cyclic configuration frame to inform the master about the committed layout.

### 0x1E Cyclic configuration

This frame configures a slave's response to the [cyclic request](#0x10-cyclic-request).

The first byte of the data section contains the address of the slave to be configured.

The rest of data section of the frame contains the object IDs to be placed into the frame.
For multi-byte objects, there are multiple entries, selecting the lower parts of the object.

> [!NOTE]
>
> If the master requests only 2 of 4 available bytes for an object, the lower 2 bytes may be sent

# Cyclic communication
