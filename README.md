## protocol

1. Client says ``HELLOSERVER``.

```json
{
    "inf": "HelloServer"
}
```

2. Server answers with ``G, P, and GSP``.

```json
{
    "inf": "NumbersServer",
    "G": 4123,
    "P": 9,
    "GSP": 234
}
```

3. Client responds with his own ``GSP``.

```json
{
    "inf": "NumbersClient",
    "GSP": 2634
}
```

4. Server sends an ``OK``.

```json
{
    "inf": "OkServer"
}
```

*Now both have the private key*

*A "secure" connection is now established.*

7. Client sends the first (and for now the last) message.

### for later implementation between 4 and 7

5. Client responds with the password
6. If server accepts the password it sends true else it sends false and won't accept messages from the client.
