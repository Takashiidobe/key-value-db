# Key-value Database

Key Value Database using `kv` and `axum`.

## Usage

This project supports CRUD operations:

- GET, PUT, DELETE `/:id`
- and POST at `/`.

You can POST a payload at the root:

```sh
curl -X POST -i https://localhost:8367 -H 'Content-Type: application/json' --data '{"value": { "x": [1,2,3] } }'
```

The schema takes a key called `value`, where you can pass any JSON object.

For example, POSTing the above returns:

```json
{ "id": "9a97c3cd-3f2c-43e0-b17a-3a905d134ca1", "value": { "x": [1, 2, 3] } }
```

You can GET an `id` like so: `https://localhost:8367/9a97c3cd-3f2c-43e0-b17a-3a905d134ca1`

Which returns the id and the value:

```json
{ "id": "9a97c3cd-3f2c-43e0-b17a-3a905d134ca1", "value": { "x": [1, 2, 3] } }
```

You can PUT a value:

```sh
curl -X PUT -i http://localhost:8367/9a97c3cd-3f2c-43e0-b17a-3a905d134ca1 -H 'Content-Type: application/json' --data '{ "value": 1 }'
```

Which returns the id and the updated value:

```json
{ "id": "9a97c3cd-3f2c-43e0-b17a-3a905d134ca1", "value": 1 }
```

You can DELETE a value:

```sh
curl -X DELETE -i http://localhost:8367/9a97c3cd-3f2c-43e0-b17a-3a905d134ca1 -H 'Content-Type: application/json'
```

Which returns nothing but a status code on proper delete:
