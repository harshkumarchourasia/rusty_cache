
# Toy Redis Database built using Rust (WIP)

## API Usage

### Set value of a key to a value

- **Endpoint**: `/v1/set`
- **Method**: `POST`
- **Headers**:
    - `Content-Type: application/json`
- **Request Body**:
  ```json
  {
    "key": "key",
    "value": "value"
  }
  ```
- **Description**: Sets the value of the specified key in the database.
- **Example**:
  ```bash
  curl -X POST http://localhost:3000/v1/set \
  --header 'Content-Type: application/json' \
  --data '{
    "key": "1",
    "value": "1"
  }'
  ```

### Get value of a key

- **Endpoint**: `/v1/get/:key`
- **Method**: `GET`
- **Description**: Retrieves the value associated with the specified key from the database.
- **Example**:
  ```bash
  curl -X GET http://localhost:3000/v1/get/1
  ```

### Unset a key

- **Endpoint**: `/v1/unset/:key`
- **Method**: `DELETE`
- **Description**: Removes the specified key and its associated value from the database.
- **Example**:
  ```bash
  curl -X DELETE http://localhost:3000/v1/unset/1
  ```

