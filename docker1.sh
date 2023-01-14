trap "echo; exit" INT
trap "echo; exit" HUP

export PROJECT_NAME=${PROJECT_NAME}

printf "\n*** Building Docker container. Please wait... \n***"
DOCKER_BUILDKIT=0 docker compose -f docker-compose1.yml build --build-arg \
    PROJECT_NAME=$PROJECT_NAME
DOCKER_BUILDKIT=0 docker compose -f docker-compose1.yml up -d
if [ $? -ne 0 ]; then
    kill "$PPID"; exit 1;
fi

CONTAINER_ID=$(docker ps -n=1 -q)
printf "\n*** Finished building. Access Docker container ID $CONTAINER_ID\n"
docker exec -it $CONTAINER_ID /bin/sh
