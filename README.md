## protocol

1. Client says ``HELLOSERVER``.

```json
{
    "inf": "HELLOSERVER"
}
```

2. Server answers with ``G, P, and GSP``.

```json
{
    "inf": "NUMBERS",
    "G": 4123,
    "P": 9,
    "GSP": 234
}
```

3. Client responds with his own ``GSP``.

*Now both have the private key*

4. Server sends an encrypted ``OK``.

*A "secure" connection is now established.*

7. Client sends the first (and for now the last) message.

### for later implementation between 4 and 7

5. Client responds with the password
6. If server accepts the password it sends true else it sends false and won't accept messages from the client.
