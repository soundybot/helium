# helium api documentation

| Route                | Method   | Request                            | Response                                | Expected Code |
|----------------------|----------|------------------------------------|-----------------------------------------|---------------|
| `/api`              | `GET`    |  No parameters required            | ``` {"api_version": "exampleversion", "version": "exampleversion"} ``` | `200`         |
| `/file`              | `POST`    |  Upload file via multipart upload  | ```{"path": "/examplepath.ext"}```      | `200`         |
| `/file`              | `DELETE` |  ```{ "path": "/filename.ext" }``` | empty respnse                           | `204`         |
