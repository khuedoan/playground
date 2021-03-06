# Install Kubernetes applications

## Cert Manager

```sh
helm repo add jetstack https://charts.jetstack.io
helm repo update
helm install \
  cert-manager jetstack/cert-manager \
  --namespace cert-manager \
  --version v1.2.0 \
  --create-namespace \
  --set installCRDs=true
kubectl apply -f test-cert-manager.yaml
```

## Dashboard

```sh
helm repo add kubernetes-dashboard https://kubernetes.github.io/dashboard/
helm install kubernetes-dashboard kubernetes-dashboard/kubernetes-dashboard
```

## Gitea

```sh
helm repo add gitea-charts https://dl.gitea.io/charts/
helm install --create-namespace gitea gitea-charts/gitea --namespace gitea -f gitea-values.yaml
```

## Drone CI

```sh
helm repo add drone https://charts.drone.io
helm repo update
helm install --create-namespace drone drone/drone --namespace drone -f drone-values.yaml
helm install --create-namespace drone-runner-kube drone/drone-runner-kube --namespace drone
```

## ArgoCD

```sh
kubectl create namespace argocd
kubectl apply -n argocd -f https://raw.githubusercontent.com/argoproj/argo-cd/stable/manifests/install.yaml
# Get password
kubectl get pods -n argocd -l app.kubernetes.io/name=argocd-server -o name | cut -d'/' -f 2
kubectl port-forward svc/argocd-server -n argocd 8080:443
```
