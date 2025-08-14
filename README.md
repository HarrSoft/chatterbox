# chatterbox
A simple microservice for delivering asynchronous messages to web clients.

# External Dependencies

This package depends on OpenSSL on platforms that are not windows and mac os.

# APIs

## `POST /message`

Headers:
```
Authorization: Basic xxxxxx
Content-Type: application/json
```

Basic auth is a shared secret between the trusted api and the chatterbox.
It is a base64-encoded string of the form `username:password`.

Body:
```typescript
interface Message<T> {
  to: CUID2;          // id of the user to send message to
  kind: const string; // discriminator for body shape
  body: T;
}
```

## `GET /subscribe?cursor=xxx` (SSE)

Above, `xxx` is the last seen message id, if applicable.

Headers:
```
Content-Type: application/json
```

Pass along session cookie as well, from app.

Event Shape:
```typescript
interface Message<T> {
  to: CUID2;
  kind: const string;
  at: Date; // timestamp of message
  body: T;
}
```
