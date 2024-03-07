#!/bin/bash

# Attach namespaces
kubectl attach -f ../k8s/namespace.yaml

# Attach secrets
kubectl attach -f ../k8s/db/secret.yaml
kubectl attach -f ../k8s/mailer/secret.yaml

# Attach configmaps

# Attach pv
kubectl attach -f ../k8s/db/pv.yaml

# Attach pvc
kubectl attach -f ../k8s/db/pvc.yaml

# Attach deployments
kubectl attach -f ../k8s/app/deployment.yaml
kubectl attach -f ../k8s/db/deployment.yaml
kubectl attach -f ../k8s/mailer/deployment.yaml
kubectl attach -f ../k8s/redis/deployment.yaml

# Attach services
kubectl attach -f ../k8s/app/service.yaml
kubectl attach -f ../k8s/db/service.yaml
kubectl attach -f ../k8s/mailer/service.yaml
kubectl attach -f ../k8s/redis/service.yaml
