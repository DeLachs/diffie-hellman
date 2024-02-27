# Diffie Hellman

This is a self written implementation with a somewhat custom protocol. It isn't meant to be used for anything and is not secure in any way.

## The "Protocol"

The protocol uses a 16 bit long unsigned integer that is send before the rest of the "package" to indicate the length of the message. After that comes the real content that is encoded in JSON because it is relative easy to work with. The 16 bit long unsigned integer implements also some sort of message length limit which isn't enforced but stops the client with a error log entry.

1. Client -> Server: ``HELLOSERVER`` to say hello and get ``G, P, and GSP``.

```json
{
    "inf": "HelloServer"
}
```

2. Server -> Client: ``G, P, and GSP``. Numbers needed by the client to generate a key.

```json
{
    "inf": "NumbersServer",
    "G": 4123,
    "P": 9,
    "GSP": 234
}
```

3. Client -> Server: The clients ``GSP`` for use in the key.

```json
{
    "inf": "NumbersClient",
    "GSP": 2634
}
```

4. Server -> Client: ``OK`` to signal that everything worked out.

```json
{
    "inf": "OkServer"
}
```

*Now both server and client have the private key.*

## Things that could be implemented

- A correct way to generate a primitive root.
- AES to send something encrypted (not in a secure way).
- Some way of authentication for clients. Like sending a password after the key exchange.
