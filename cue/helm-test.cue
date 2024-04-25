bundle: {
    apiVersion: "v1alpha1"
    name:       "cluster-addons"
    instances: {
        "cert-manager": {
            module: url: "oci://ghcr.io/stefanprodan/modules/flux-helm-release"
            namespace: "cert-manager"
            values: {
                repository: url: "https://charts.jetstack.io"
                chart: {
                    name:    "cert-manager"
                    version: "1.x"
                }
                helmValues: {
                    installCRDs: true
                }
            }
        }
        "ingress-nginx": {
            module: url: "oci://ghcr.io/stefanprodan/modules/flux-helm-release"
            namespace: "ingress-nginx"
            values: {
                repository: url: "https://kubernetes.github.io/ingress-nginx"
                chart: {
                    name:    "ingress-nginx"
                    version: "4.x"
                }
                helmValues: {
                    controller: service: type: "NodePort"
                }
            }
        }
        "ingress-nginx-public": {
            module: url: "oci://ghcr.io/stefanprodan/modules/flux-helm-release"
            namespace: "ingress-nginx"
            values: {
                repository: url: "https://kubernetes.github.io/ingress-nginx"
                chart: {
                    name:    "ingress-nginx"
                    version: "4.x"
                }
                helmValues: {
                    controller: service: type: "NodePort"
                }
            }
        }
    }
}
