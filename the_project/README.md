# The Project

This is a project to learn kubernetes from the MOOC.fi course "DevOps with Kubernetes".

## Getting Started

1. Make a k3d local registry for image storage
```bash
k3d registry create local-registry --port 5000
```

2. Make sure your local k3d cluster is running
```bash
k3d cluster create --port 8082:30080@agent:0 -p 8081:80@loadbalancer --agents 2 --registry-use k3d-local-registry:5000
```

3. Create the project namespace

```bash
kubectl apply -f ../namespaces/project.yaml
kubens project
```

4. Create the required persistent volume directory in k3d node

```bash
docker exec k3d-k3s-default-agent-0 mkdir -p /tmp/kube
```

5. Build and push the images

```bash
docker build -f Dockerfile.app -t localhost:5000/todo-app:latest .
docker build -f Dockerfile.backend -t localhost:5000/todo-backend:latest .
docker build -f Dockerfile.cronjob -t localhost:5000/wiki-todo-generator:latest .
docker push localhost:5000/todo-app:latest
docker push localhost:5000/todo-backend:latest
docker push localhost:5000/wiki-todo-generator:latest
```

6. Deploy the infrastructure and application

```bash
# Apply persistent volumes
kubectl apply -f ../volumes/theprojectpv.yaml
kubectl apply -f ../volumes/theprojectpvc.yaml

# Apply everything (excluding CronJob)
kubectl apply -f manifests/configmap.yaml
kubectl apply -f manifests/postgres-statefulset.yaml
kubectl apply -f manifests/backend-service.yaml
kubectl apply -f manifests/service.yaml
kubectl apply -f manifests/backend-deployment.yaml
kubectl apply -f manifests/deployment.yaml
kubectl apply -f manifests/ingress.yaml

# Wait for all deployments to be ready
# Now deploy the cronjob, kinda hacky but the exercise didn't specify any better way from what I remember
kubectl apply -f manifests/cronjob.yaml

# Test the CronJob immediately (optional, or wait 1 hour for it to run automatically)
kubectl create job --from=cronjob/wiki-todo-generator wiki-test-manual -n project
kubectl logs -l job-name=wiki-test-manual -n project
kubectl delete job wiki-test-manual -n project
```

> [!NOTE]
> Note that the images are tagged as `localhost:5000` when pushing from the host machine, but the Kubernetes manifests reference them as `k3d-local-registry:5000` since that's the internal cluster DNS name.

7. Access the application via Ingress

The application is accessed through the ingress controller. Visit:

```
http://localhost:8081
```

## Features

- The backend uses a Postgres StatefulSet to persist todos. The database is deployed first and should reach Ready state before the backend tries to connect.

- A CronJob is deployed for fetching random Wikipedia articles and adding them as todos every hour.

![The Project](../assets/the_project_screenshot.png)
