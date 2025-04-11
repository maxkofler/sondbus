# The sondbus protocol version 1.0.0

This document describes the `sondbus` protocol that aims to create a reliable communication channel for embedded devices.

## Table of contents

- [1 - Physical Layer](#1---physical-layer)
- [2 - Frame Layout](#2---frame-layout)
- [3 - Frame Types](#3---frame-types)
- [4 - Slave Synchronization](#4---slave-synchronization)
- [5 - Addressing](#5---addressing)
- [6 - Object Dictionary](#6---object-dictionary)

## Data Layout

Sondbus works on a byte-basis, meaning that it does not require padding.

Multi-byte values are encoded in `little-endian` style.

## About this document

This document describes the workings of the protocol.

Payload structures are explained using syntax of the `Rust` programming language, as its type hints are pretty clear.
This means that the following:

```rust
struct SomeStructure {
    first: u8,
    second: u8,
}
```

translates to this sequence of bytes:

| first | second |
| :---: | :----: |
|  1    |   1    |

# 1 - Physical Layer

Sondbus is designed to work over anything that can transmit and receive individual bytes.
This makes the barrier for entry quite low, as no facilities for framed packet handling are required from the environment.
Additionally, it lowers the overhead on the bus, making the communication channel wider for user data.

# 2 - Frame Layout

Sondbus 'frames' are not fixed in their layout.
This means that it depends on some bytes that are submitted as part of the frame.

The overlaying structure is made up of the following parts:

| Start | Type | Payload | CRC |
| :---: | :--: | :-----: | :-: |
|   1   |  1   |   ...   |  1  |

- `Start`: The start byte is always `0x55` - it indicates the start of a frame
- `Type`: The type of the frame that is about to be transmitted
- `Payload`: The payload that is shipped in this frame
- `CRC`: The checksum over `Start`, `Type` and `Payload`

This layout is expected and will not change, while the payload is free in its contents.

# 3 - Frame Types

Sondbus defines the following frame types:

- `0x0_`: [Management frames](#30---management-frames)
  - `0x00`: [Sync](#300---sync)
  - `0x01`: [Ping](#301---ping)

# 3.0 - Management frames

These frames facilitate management of the bus and form the mandated core of the bus.

# 3.0.0 - Sync

This frame synchronizes a slave with the bus, setting the internal `in_sync` flag.

The payload is fixed as the following sequence of bytes.

```hex
1F 2E 3D 4C 5B 6A 79 88 97 A6 B5 C4 D3 E2 F1
```

If this frame is received with the correct CRC, the slave enters a synchronized state that allows it to put data on the bus and accept data from the bus.

# 3.0.1 - Ping

The ping frame allows a master to check if a slave is online and in a synchronized state, ready to accept instructions and respond to them.

```rust
struct Ping {
    destination: [u8; 6],
    source: [u8; 6],
}
```

- `destination`: The device to be pinged
- `source`: The pinging device

## Response

If the `destination` matches up with the physical address of a slave, it responds with a [Ping](#301---ping) frame with reversed payload, pinging back the device that sent the ping in the first place.

# 4 - Slave synchronization

Sondbus requires a slave to keep exact track of all submitted bytes on the bus.
This requires some mechanism for a slave to synchronize with the bus and 'clock in' to the conversation.

To facilitate this mechanism, all slaves have an internal flag, here called `in_sync` that determines if a slave is synchronized with the bus.
This flag is required for a slave to process **any** data received on the bus.
If it is not set, the slave will only accept one frame: [Sync](#300---sync).
In this out-of-sync state, the slave ignores all bytes, except they line up as the [Sync](#300---sync) frame.

If a slave is out-of-sync (having the `in_sync` flag not asserted), it **must** return to a safe state that does not require bus communication.
In this state, a slave will **never** transmit on the bus and process incoming data until it is back in sync.

# 5 - Addressing

Sondbus uses 2 layers of addressing:

- [Physical Addressing](#51---physical-addressing)
- [Logical Addressing](#52---logical-addressing)

## 5.1 - Physical Addressing

The physical address of a slave is fixed in that it is bound to the physical interface that is attached to the bus.
It consists of 48 bits (6 octets) that are equivalent to a MAC address.
This address is hardcoded into each slave and should not change, as data could be bound to it.

## 5.2 - Logical Addressing

A logical address is split into 2 parts:

- Universe
- Address

A universe is a collection of 254 devices (+1 broadcast), while an address is unique within its universe.
The address `0xFF` is reserved as a broadcast address, directing a frame with this address at all slaves in the universe.
The universe `0xFF` is reserved as a broadcast in a global sense, directing a frame to all universes and slaves within them whose addresses match up.
A global broadcast can be achieved by sending a frame to universe `0xFF` and address `0xFF`.

This separation allows for partitioning of the network to group up slaves in a way that allows the master to optimize transfers and broadcasts.

# 6 - Object Dictionary

Sondbus uses an object dictionary that is compatible with CANOpen.
Implementing it requires each slave to present a dictionary that is indexed by an `Index` of 16 bytes and a `Subindex` of 8 bytes.
