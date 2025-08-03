# 100DaysRustDevelopment


kubectl get crd gateways.gateway.networking.k8s.io &> /dev/null || { kubectl apply -f https://github.com/kubernetes-sigs/gateway-api/releases/download/v1.2.1/standard-install.yaml; }