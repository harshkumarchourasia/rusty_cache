
# Toy Redis Database built using Rust

## API Usage

### Set value of a key to a value
- Endpoint: `/v1/set?key=key&value=value` (POST)

### Get value of a key
- Endpoint: `/v1/get/:key` (GET)

### Unset a key
- Endpoint: `/v1/unset/:key` (DELETE)

### Is the key set
- Endpoint: `/v1/isset/:key` (GET)
