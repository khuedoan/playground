Requires Nix and Docker.

```sh
❯ nix --version
nix (Nix) 2.18.1

# If you're on macOS
❯ colima --version
colima version 0.6.8
❯ colima start --cpu 2 --memory 8
```

```sh
nix develop
```

Create a cluster:

```sh
make kind
kubectl get nodes
make kind/info
```

Curriculum:

```sh
# 10% - Cluster Setup
make cluster-setup/network-polices
make cluster-setup/cis-benchmark
make cluster-setup/ingress
make cluster-setup/protect-node
make cluster-setup/block-gui
make cluster-setup/verify-binaries
```

References:

- https://github.com/cncf/curriculum/blob/master/CKS_Curriculum_%20v1.29.pdf
- https://www.youtube.com/watch?v=d9xfB5qaOfg
