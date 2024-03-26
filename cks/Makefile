.POSIX:

.PHONY: default
default: kind

.PHONY: kind
kind:
	kind create cluster --config kind/cluster.yaml

.PHONY: kind/info
kind/info:
	docker exec -it kind-control-plane ls -l /etc/kubernetes/pki/
	@echo "CA"
	docker exec -it kind-control-plane cat /etc/kubernetes/pki/ca.crt
	@echo "Kube API server certificate"
	docker exec -it kind-control-plane cat /etc/kubernetes/pki/apiserver.crt
	@echo "etcd server certificate"
	docker exec -it kind-control-plane cat /etc/kubernetes/pki/etcd/server.crt
	@echo "Kube API etcd client certificate"
	docker exec -it kind-control-plane cat /etc/kubernetes/pki/apiserver-etcd-client.crt
	@echo "Kube API kubelet client certificate"
	docker exec -it kind-control-plane cat /etc/kubernetes/pki/apiserver-kubelet-client.crt
	@echo "Kube scheduler client certificate in the config file"
	docker exec -it kind-control-plane cat /etc/kubernetes/scheduler.conf
	@echo "Kube cloud controller manager client certificate in the config file"
	docker exec -it kind-control-plane cat /etc/kubernetes/controller-manager.conf
	@echo "Kubelet client certificate in the config file"
	docker exec -it kind-control-plane cat /etc/kubernetes/kubelet.conf
	@echo "Kubelet certificates"
	docker exec -it kind-control-plane ls -l cat /var/lib/kubelet/pki/

.PHONY: cluster-setup/network-polices
cluster-setup/network-polices:
	echo "TODO"

.PHONY: cluster-setup/restrict-gui
cluster-setup/restrict-gui:
	kubectl apply --context=kind-kind -f https://raw.githubusercontent.com/kubernetes/dashboard/v2.7.0/aio/deploy/recommended.yaml
