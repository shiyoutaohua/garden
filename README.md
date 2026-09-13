# garden

The study project.

## Cookbook

### CI/CD

- VM

```sh
podman machine stop
podman machine rm podman-machine-default
podman machine init --memory 4096 --cpus 4 --disk-size 100 podman-machine-default
podman machine start
podman machine list

podman login --username=qingyuehanxi@insta insta-registry.cn-shanghai.cr.aliyuncs.com
podman pull --platform linux/amd64 alpine:3.23
podman tag 0d3e09a185d9 insta-registry.cn-shanghai.cr.aliyuncs.com/public/alpine:3.23
podman push insta-registry.cn-shanghai.cr.aliyuncs.com/public/alpine:3.23
```

- Build

```sh
podman build -t garden-daisy:latest -f .docker/Dockerfile.daisy ./
podman build -t garden-rose:latest -f .docker/Dockerfile.rose ./
podman run -d -p 8090:8080 --name c-garden-daisy garden-daisy
podman exec -it c-garden-daisy sh
podman rm -f c-garden-daisy
podman logs -f {ContainerId}
podman run --rm -it --entrypoint /bin/sh {ImageId}
podman container prune
podman image prune
podman volume prune
podman network prune
podman pod prune
```

- Compose

```sh
podman-compose -f .docker/docker-compose.yml up -d --build
podman-compose -f .docker/docker-compose.yml down -v
podman-compose -f .docker/docker-compose.yml --profile app up -d --build
podman-compose -f .docker/docker-compose.yml --profile app down -v
```

### Cmd

- util

```sh
dd if=/dev/random of=./tmp.bin bs=10M count=1
find . -name '.DS_Store' -type f -delete
find . -name '__MACOSX' -type d -exec rm -rf {} +
```

- 7z archive

```sh
7z a garden.zip garden '-x!garden/target' '-x!garden/tmp' '-x!garden/.git' '-x!garden/.claude' '-x!garden/.codex'
```
