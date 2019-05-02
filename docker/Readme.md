
## Table of Contents
* [About](#about)
* [Quick start](#quick-start)
* [Directory structure](#directory-structure)
* [Alacris containers](#list-of-containers)
   * [alacris_mkb_build_prerequisites](#alacris_mkb_build_prerequisites)
   * [alacris_mkb_run_prerequisites](#alacris_mkb_run_prerequisites)
   * [alacris_mkb](#alacris_mkb)
* [Logs](#logs)

## About
This directory contains Docker configuration used for building images and
running containers/services needed for the `legicash-mkb` system to work in
local Docker environments. All containers in this project run in the same Docker network as `legicash-facts` so they can communicate between each other. 

[Docker](https://docs.docker.com/) images are created with `Dockerfile`s.

[docker-compose](https://docs.docker.com/compose/overview/) is used to describe
the desired state of Legicash services, thus implementing an Infrastructure as
Code (IaC) approach.

## Quick start
**Prerequisites:**
(If you already have `legicash-facts` Docker setup you can skip this part)
  - Install and configure the most recent version of the
    [Cloud SDK](https://cloud.google.com/sdk/docs/),
    which includes the gcloud command-line tool (use `us-central-1a` region and
    `legicash-demo-1950` project)
  - Install [Docker](https://docs.docker.com/install/) minimum required version
    `17.12.0-ce`
  - Install [docker-compose](https://docs.docker.com/compose/install/) minimum
    required version `1.16.1`
  - Get [access to the registries which](https://cloud.google.com/container-registry/docs/access-control)
    you will be pushing to and pulling from
  - Configure Docker to use `gcloud` as a credential helper, or are use another
    [authentication method](https://cloud.google.com/container-registry/docs/advanced-authentication).
    To use `gcloud` as the crediential helper, run the command:
    ```bash
    gcloud auth configure-docker
    ```

#### Pull build and runtime prerequisites images:
```bash
$ make docker-pull
```
Check **docker-compose.yml** file for detailed service configuration.

#### Build all images
NOTE: This takes a lot of time to build everything from scratch. Please use
`docker-pull` and `docker-build` targets instead.
```bash
$ make docker-build-all
```

#### Build app images
```bash
$ make docker-build
```

#### List available containers
To list all containers run:
```bash
$ make docker-list
```
```
alacris_registrar_alice
alacris_registrar_bob

```

#### Run all containers
To run all containers at once run the command:
```bash
$ make docker-up
```

To run all containers at once in detached mode run the command:
```bash
$ make docker-start
```

## Directory structure
```
docker
├── config
│   ├── common_init_file.json
│   ├── init_Alice.json
│   └── init_Bob.json
├── containers
│   ├── alacris_mkb
│   │   ├── Dockerfile
│   │   ├── dummy_src
│   │   │   └── src
│   │   │       ├── main_client.rs
│   │   │       ├── main_keygen_secp256k1.rs
│   │   │       └── main_mkb_registrar.rs
│   │   └── files
│   │       ├── conf
│   │       │   ├── supervisord_alice.conf
│   │       │   └── supervisord_bob.conf
│   │       └── scripts
│   │           ├── keygen.sh
│   │           ├── run_registrar_alice.sh
│   │           └── run_registrar_bob.sh
│   ├── alacris_mkb_build_prerequisites
│   │   └── Dockerfile
│   └── alacris_mkb_run_prerequisites
│       └── Dockerfile
├── docker-compose.yml
├── Readme.md
└── scripts
    ├── build_all_images.sh
    └── pull_images.sh

```

## Containers
#### alacris_mkb_build_prerequisites
This image is used as a build image for alacris-mkb. 

#### alacris_mkb_run_prerequisites
Container used for running the registrars. Based on oficial Rust image.

#### alacris_mkb
Final app image.


#### alacris_registrar_alice
To build alacris_registrar_alice run the command:
```bash
$ make docker-build c=alacris_registrar_alice
```
To start alacris_registrar_alice run the command:
```bash
$ make docker-start c=alacris_registrar_alice
```

#### alacris_registrar_bob
To build alacris_registrar_bob run the command:
```bash
$ make docker-build c=alacris_registrar_bob
```
To start alacris_registrar_bob run the command:
```bash
$ make docker-start c=alacris_registrar_bob
```

## Connect into containers
As application user
```bash
$ docker exec -ti containername bash
```
As root user
```bash
$ docker exec -ti -u 0 containername bash
```

## Logs
Application logs that are in `stdout` 
