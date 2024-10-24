# Dope DB
A database used as an underlying KV store for dopechain

## How it works
TBH, I don't know. The answer is barely. We mainly operate on the concept of so called "namespaces". Each namespace can have some values and other namespaces associated with it - think a directory with some files and subdirectories. The database provides a guarantee that namespaces are completely separated, even from their child-namespaces. We also keep a checksum calculated based on the state of the entire DB - updated kind of like with Merkle Trees, but in a more simple and probably less optimal manner - still, recalculation of the entire hash requires an update only in the namespaces representing a path from the root to the namespace storing the value.

## Professional api which uses http - i love performant solutions
### GET /api/checksum
Returns a checksum of the DB state.
```ts
class ChecksumResponse {
    checksum: string
}
```

### GET /api/get
Fetches the value present under a key in a namespace
Request body:
```ts
class GetRequest {
    key: string; // Key that the value is stored under
    namespace: string; // Path to the namespace
}
```
Response:
```ts
class GetResponse {
    value: string;
    checksum: string; // Checksum of the entire db state
}
```

### POST /api/insert
Inserts or updates a KV pair into a namespace   
Request:
```ts
class InsertRequest{
    namespace: string;
    key: string;
    value: string;
}
```
Response:
```ts
class ChecksumResponse {
    checksum: string
}
```

### DELETE /api/delete
Deletes a KV pair in a namespace (does not delete the namespace itself, even if empty)
Request:
```ts
class DeleteRequest {
    key: string; // Key that the value is stored under
    namespace: string; // Path to the namespace
}
```
Response: 
```ts
class ChecksumResponse {
    checksum: string
}
```
