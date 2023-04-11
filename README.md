make sure to run docker in elevated role with
```
newgrp docker
```

Docker then can be run with
```
docker run --init -p 5005:5005 nas-ws
```

## Incremental update
after making changes to the code, we need to rebuild docker with
```
docker build -t nas-ws .
```
