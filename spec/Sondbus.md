# The Sondbus protocol v1.0.0

This is the specification for the sondbus protocol (from now on sondbus).
It is a simple and minimal protocol for facilitating communication of multiple controllers using a shared medium.

The goal is to create a solid bus that can be used in a cyclic manner for facilitating time-critical and reliable communication.

It works in a single-master principle in that a single device in the network takes on the role of the master that coordinates all communication.
This makes the whole network timing predictable and the slave implementations smaller.

Table of contents

- [Data Link Layer](#data-link-layer)
- [Frame Types](#frame-types)
- [Addressing](#addressing)
- [Unframed Response](#unframed-response)

# Data Link layer

The first layer in the sondbus protocol is the data link layer.
This is comparable to layer 2 of the OSI model and facilitates the transport of single frames over the medium.

This layer is also used for cyclic communication, as it provides a deterministic and minimal way for transmitting data.

In either way, all communication has to be transmitted via this channel.

## Frame layout

The following table explains the frame format:

| Start | Type | Data | CRC |
| :---: | :--: | :--: | :-: |
|   1   |  1   | .... |  1  |

### Start

The start byte is always `0x55` to indicate to the other network participants that a new frame is being transmitted.

### Type

The type of data that is transmitted in this frame, see [frame types](#frame-types).

### Data

The data potion of a frame contains the payload whose length and fields depend on the frame type at hand.

### CRC

All frames are checked for errors using a crc8 checksum.
The checksum to be used is the `AUTOSAR` flavor.

The CRC is calculated over all the leading bytes, including:

- Start
- Type
- Data

## Error handling

The inclusion of a CRC hints at the presence of error detection.
In contrast to OSI layer 2 there is no error correction on the data link layer.

As the communication is time-critical, a mismatched CRC will simply lead to the frame being dropped by all network participants.

# Frame Types

There are various frame types that facilitate the sondbus communication protocol.

- Management frames (`0x0_`)
  - [`0x00` Sync](#0x00-sync)
  - [`0x01` Ping](#0x01-ping)
- Acyclic communication (`0x1_`)
  - [`0x10` SDO Read](#0x10-sdo-read)
  - [`0x11` SDO Response](#0x11-sdo-response)
  - [`0x16` SDO Write](#0x16-sdo-write)
  - [`0x1E` SDO Ack](#0x1e-sdo-ack)
  - [`0x1F` SDO Abort](#0x1f-sdo-abort)
- Cyclic frame types (`0x2_`)
  - [`0x20` Cyclic request](#0x20-cyclic-request)
  - [`0x21` Cyclic object configuration](#0x21-cyclic-object-configuration)
  - [`0x22` Cyclic object configuration confirm](#0x22-cyclic-object-configuration-confirm)
  - [`0x23` Cyclic object configuration reject](#0x23-cyclic-object-configuration-reject)

# Addressing

Sondbus has three layers of addressing:

- Physical address (MAC)
- Universe address
- Logical address

The sondbus master is free to partition the network into universes with logical addresses to bundle devices together in a way that makes sense.

### Physical address

The physical address is globally and universally unique to each device that can communicate using the sondbus protocol.
It is a normal `MAC` address consisting of 6 bytes.

### Universe address

The sondbus protocol supports the concept of universes to expand the address space.
This allows for more possible addresses while not fully committing to 16 bit addresses.
The universe address is a 8-bit number.

### Logical address

The logical address is unique within a universe and identifies a single device or multiple functions of a device.
The sondbus protocol does not care if a single device emulates multiple logical addresses.

# Object dictionary

Each sondbus slave presents an object dictionary that holds all the possible data fields that can be read or written by the master.

An object is identified by its object id which is a `16-bit` unique identifier.
Sondbus does not foresee the use of subindices to make implementations as small and easy as possible.
Each object defines a type and a size with the maximum being `255 bytes`.

## 0x00 Sync

The sync frame type is here to allow slaves to synchronize with the communication.
It provides a known sequence of bytes that should be unique enough for a slave to detect it and thus sync up with the communication that is going on on the bus.

The `data` section of the frame is fixed as the following 15-byte hex string:

```hex
1F 2E 3D 4C 5B 6A 79 88 97 A6 B5 C4 D3 E2 F1
```

This frame is one-way only in that there is no response from any slave to this frame as it is purely for synchronization purposes.

Once a slave starts up or rejoins the communication from a fatal error, it will wait for exactly this sequence before accepting and processing further frames.
This prevents from a slave getting out of sync and trying to handle invalid and unexpected data frames.

## 0x01 Ping

This frame type allows the master to request presence information from a slave.

The `data` portion of the frame is 12 bytes long, containing first the 6-byte MAC address of the device being pinged and then the 6-byte MAC address of the pinging device.

### Response

If a slave's MAC matches up with the pinged MAC, it responds with a ping frame with swapped order, inserting its MAC as the pinging device and the master's MAC as the pinged address.

## 0x10 SDO Read

This frame type allows the master to request object data from a slave.

The `data` field is fixed with the following parameters:

| Universe | Address | Object Index | Start | End |
| :------: | :-----: | :----------: | :---: | :-: |
|    u8    |   u8    |     u16      |   u8  | u8  |

The `Object Index` field dictates the index of the object to be read by the master.
The `Start` field indicates the starting position of the object to be read, while the `End` field indicates the ending position.
This allows a master to read only a slice of the full object contents while giving the slave an indication of the amount of data expected by the calling device.

## 0x11 SDO Response

This frame type originates from the slave and is initiated by a [SDO read](#0x10-sdo-read).
It responds with the requested data to the requesting device.

| Length | Data |
| :----: | :--: |
|   u8   |  ... |

The frame layout is simple in that it only contains the length of the response and the data.

> [!NOTE]
>
> The response length can be lower than the requested length for various reasons.

## 0x16 SDO Write

This frame type allows a master to write an object via the SDO channel.
The layout of the `data` section is as follows:

| Universe | Address | Object Index | Start | End | Data |
| :------: | :-----: | :----------: | :---: | :-: | :--: |
|    u8    |   u8    |     u16      |   u8  | u8  | ...  |

The `Object Index` field dictates the index of the object to be written by the master.
The `Start` field indicates the starting position in bytes within the object to be written, while the `End` field indicates the ending position.
This allows a master to write only a slice of the full object contents while giving the slave an indication of the amount sent by the calling device.

The slave will respond with one of the following frames:

- [SDO Ack](#0x1e-sdo-ack): The write was successful
- [SDO Abort](#0x1f-sdo-abort): The write operation failed

## 0x1E SDO Ack

This frame type acknowledges one of the following SDO operations:

- [SDO Write](#0x16-sdo-write)

The `data` field contains the frame type of the operation to be acknowledged:

| Frame Type |
| :--------: |
|     u8     |

## 0x1F SDO Abort

This frame type originates from the slave and is initiated by any SDO request and indicates failure to fulfil the request by the master.

The `data` field is filled with the following structure:

```rust
struct SDOError {
  operation: u8,    // 0x00 for read, 0x10 for write
  index: u16,       // The index of the object that was accessed
  abort_code: u16   // An abort code indicating the error
}
```

### SDO Abort codes

- `0x10`: Unknown object index: The requested `index` is not in the object dictionary
- `0x20`: Invalid access (unknown): The operation is not allowed due to unknown or other reasons
  - `0x21`: Read from Write-Only object
  - `0x22`: Write to Read-Only object
  - `0x23`: Read start (`Start`) is out of range
  - `0x24`: Read end (`End`) is out of range
- `0x1xxx`: Device-specific errors

## 0x20 Cyclic request

This frame type initiates a new cycle on the network and is sent by the master.
The address field is used as the source address and should always be 0.

The data section of the frame contains the amount of data in bytes each slave is required to send.
The layout of the response is agreed-upon in the setup phase of the communication via [cyclic configuration](#0x1f-cyclic-configuration) frames.

After this, each slave awaits its turn and sends its data plus a CRC if it sends some data.
If the slave is not required to send any data, it does not transmit the CRC, leaving the bus untouched.

Each slave can send a maximum of 255 bytes per cycle.
The following table shows a possible configuration of the data section for this frame type:

| Master (`0`) | Slave 1 (`1`) | Slave 2 (`2`) | ... |
| :----------: | :-----------: | :-----------: | --- |
|      2       |       0       |       4       | ... |

This declares the following:

- The master (addr=0) sends 2 bytes (+1 byte CRC)
- Slave 1 (addr=1) sends nothing (no CRC)
- Slave 2 (addr=2) sends 4 bytes (+1 byte CRC)

This request is then followed by a [unframed response](#unframed-response).

## 0x21 Cyclic object configuration

This frame type changes the configuration for one slave and adjusts a new set of data to be sent by this slave.

> [!NOTE]
>
> The address field is used as the destination of the frame, addressing the slave to be configured.

The data section of the frame contains the object IDs of the objects to be responded to [cyclic request](#0x10-cyclic-request) frames.

Multi-byte object IDs are repeated for the size, while cutting upper bytes.
This is best shown in a example:

An object with ID `0xFD` with 32 bits (4 bytes) will be requested via the following string: `FDFDFDFD` (4 repetitions of the object ID).
If the master is only interested in some subset, the lower bytes will be transmitted.
The example of the master being only interested in 2 bytes will look as follows: `FDFD` and the lower 16 bits of the object would be sent by the slave.

The master will also broadcast a configuration frame for its own configuration.
This informs all the slaves about the objects to expect from the master.

## 0x22 Cyclic object configuration confirm

This frame is a response to a [cyclic object configuration](#0x21-cyclic-object-configuration) frame.

It basically repeats the contents of the frame to be confirmed and applies it for the following [cyclic request](#0x20-cyclic-request)s.
This confirms to the master that the slave has successfully applied the new configuration.

> [!NOTE]
>
> The address field is used as the destination, always addressing the master (0).

## 0x23 Cyclic object configuration reject

This frame is a response to a [cyclic object configuration](#0x21-cyclic-object-configuration) frame.

The slave rejects the cyclic configuration and the new configuration is not applied.
This can be caused by many different things and the data section of the response contains a UTF-8 string explaining the failure.

> [!NOTE]
>
> The address field is used as the destination, always addressing the master (0).

# Unframed response

Some responses do not happen in a framed manner, but rather in an interleaved one.
This saves on data when slaves respond to the master, especially in time-critical communication paths.

In this form of response, the data exchange is not framed up, but rather sent loosely on the bus.
The slave simply awaits its turn and sends its data and a [CRC](#unframed-response-crc), completely bypassing the framed nature of the bus.
Slaves that are asked to send no data (0 bytes) still send the [CRC](#unframed-response-crc) to signal that the chain is still intact.

The following example shows the response to a [cyclic request](#0x20-cyclic-request) requesting the following (`XX` represents the CRC):

- Master (addr=0) - 2 bytes: `ABCD`
- Slave 1 (addr=1) - 0 bytes, but the CRC
- Slave 2 (addr=2) - 4 bytes: `DEADBEEF`

```
.<request frame>.....ABCD..XX.......XX........DEADBEEF.XX...<end>
|     Master    |___| Master |__| Slave 1 |__|  Slave 2  |_______
```

### Unframed response timeout

This approach, however has its drawbacks in that a non-responding slave can completely block the communication.
To prevent this, the master takes a strong hold on the timing of the responses and strictly governs the communication.

Each slave must start its response within `<timeout> x <byte count (without CRC)> x <byte time>`.
If the slave fails to do so, knowingly or not, the master takes over and fills the slot with arbitrary data and an invalid CRC checksum, the other slaves may also detect the timeout, but may **never** respond, that is the job of the communication master.
This temporarily bridges the missing slave's communication while not inserting any wrong data into the network.

> [!NOTE]
>
> Such a timeout will introduce some delay into the network, but depending on the master, the timed out slave may be excluded from the following cycles.

Slaves may also drop from the bus while in the act of transmitting.
In this case, bytes may have at most `<timeout> * <byte time>` of spacing.
In case of this timing being violated, the master takes over the bus and finishes the broken data slot.

> [!NOTE]
>
> Once the master has taken over the communication, the overridden slave may not restart transmission, but rather await the next cycle to regain the ability to communicate.
> The master may **never** be overridden.

### Unframed response CRC

The CRC in each response block is calculated over the following:

- The CRC of the request frame
- All the data received up to the slave's slot
- The data transmitted by this slave

This creates a verification chain where each network participant can determine how far it can trust the received data.

If a slave reaches its turn, but the CRC of the previous response is incorrect, it may not respond to the bus, but rather let the master take over communication as if it were timed out.

This makes sure that once one slave times out, all further communication is invalidated and the slaves run no risk of getting out of sync.

The following example shows the CRC influence for the previous response example:

```
.<request frame>..XX.....ABCD..XX.....DEADBEEF..XX...123456..XX..<end>
_________________|   CRC 1   |________________________________________
_________________|           CRC 2            |_______________________
_________________|                   CRC 3                 |__________
```
