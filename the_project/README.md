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

3. Create the required resources using kubectl

```bash
docker tag todo-app:latest k3d-local-registry:5000/todo-app:latest
docker tag todo-backend:latest k3d-local-registry:5000/todo-backend:latest
docker push k3d-local-registry:5000/todo-app:latest
docker push k3d-local-registry:5000/todo-backend:latest

kubectl apply -f ../volumes/
kubectl apply -f manifests/
```

4. Access the application

```bash
kubectl port-forward svc/the-project-service 8080:3000
```

Visit `http://localhost:8080`

![The Project](../assets/the_project_screenshot.png)
