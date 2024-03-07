#!/bin/bash

# apply namespaces
kubectl apply -f $(pwd)/k8s/namespace.yaml

# apply secrets
# kubectl apply -f $(pwd)/k8s/app/pullSecret.yaml
kubectl apply -f $(pwd)/k8s/db/secret.yaml
kubectl apply -f $(pwd)/k8s/mailer/secret.yaml

# apply configmaps

# apply pv
kubectl apply -f $(pwd)/k8s/db/pv.yaml

# apply pvc
kubectl apply -f $(pwd)/k8s/db/pvc.yaml

# apply deployments
kubectl apply -f $(pwd)/k8s/app/deployment.yaml
kubectl apply -f $(pwd)/k8s/db/deployment.yaml
kubectl apply -f $(pwd)/k8s/mailer/deployment.yaml
kubectl apply -f $(pwd)/k8s/redis/deployment.yaml

# apply services
kubectl apply -f $(pwd)/k8s/app/service.yaml
kubectl apply -f $(pwd)/k8s/db/service.yaml
kubectl apply -f $(pwd)/k8s/mailer/service.yaml
kubectl apply -f $(pwd)/k8s/redis/service.yaml
