# helium
helium is a small and lightweight s3 storage proxy
## What is helium?
First of all: helium is not an image server. It is only the proxy used by the helium clients (coming soon™).
helium is a proxy for uploading and deleting files on storage servers (for example for use in your personal image server).
Of course you can work on your own implentation of the helium api



## Configuration
---
### General environment variables
| Environment Variable          | Default Value              | Type     | Description                                                                  |
|-------------------------------|----------------------------|----------|------------------------------------------------------------------------------|
| `helium_key`                  | won't start without this   | `string` | This is the key needed to upload and delete files                            |
| `helium_host`                 | `0.0.0.0`                  | `string` | Defines the ip helium will listen to                                         |
| `helium_port`                 | `3000`                       | `int`    | Defines the port helium will bind to                                         |
---
### Storage
As of now, only one storage method is supported: s3 and s3 compatible storage servers (e.g. minio etc)

#### S3
| Environment Variable          | Default Value              | Type     | Description                                                                  |
|-------------------------------|----------------------------|----------|------------------------------------------------------------------------------|
| `helium_s3_host`              | `localhost`                | `string` | Defines the host, the s3 server runs on                                      |
| `helium_s3_acc_key`           |  none                      | `string` | The access key helium will use to connect to the s3 server                   |
| `helium_s3_sec_key`           |  none                      | `string` | The security key, helium will use to connect to the s3 server                |
| `helium_s3_port`              |  `9000`                    | `int`    | The port helium will use to connect to the s3 server                         |
