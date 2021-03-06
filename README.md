# idocker-rs
A work-in-progress CLI utility for interactively dealing with multiple containers. 

```
idocker container rm --force
```
```
  [x] /container_sockets_client_1 , image python , status Exited (0) 4 days ago
  [x] /container_sockets_server_1 , image python , status Exited (137) 4 days ago
  [ ] /mystifying_sanderson , image ubuntu , status Exited (0) 3 weeks ago
  [x] /keen_neumann , image sha256:a86ee5538082a457ab2dbbb17a61d357e3d1f16992b12e41f5d6728b3edb3de8 , status Exited (1) 2 months ago
  [ ] /elastic_maxwell , image sha256:5ef74449dfeac9d81122ff82f5a7aa248f2cd5664a7aafba85f4ee27ff3c7d43 , status Exited (1) 2 months ago
> [x] /tender_hoover , image sha256:e47c5f22533cfd2fd078472aebf54ecc77faae7036efbb3410a5e2133e642977 , status Exited (1) 2 months ago
```
