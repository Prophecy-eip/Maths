# Build the docker image
docker build -t prophecy/maths .

docker run --rm -iv${PWD}:${PWD} prophecy/maths sh -s <<EOF
chown -Rv $(id -u):$(id -g) pkg
cp -va pkg ${PWD}
EOF