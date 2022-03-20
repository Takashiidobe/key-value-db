# Key-value Database

Key Value Database using `kv` and `axum`.

## Usage

This project supports two requests, GET `/:id` and POST `/`.

You can GET an `id` like so: `https://kv.takashiidobe.com/1d1a4354-19d9-4c8b-b38d-5ca0994c114f`

Which returns the id and the value:

```json
{ "id": "1d1a4354-19d9-4c8b-b38d-5ca0994c114f", "value": { "x": [1, 2, 3] } }
```

You can POST as well:

```sh
curl -X POST -i https://kv.takashiidobe.com -H 'Content-Type: application/json' --data '{"value": { "x": [1,2,3] } }'
```

The schema takes a key called `value`, where you can pass any JSON object.

For example, POSTing the above returns:

```json
{ "id": "9a97c3cd-3f2c-43e0-b17a-3a905d134ca1", "value": { "x": [1, 2, 3] } }
```
