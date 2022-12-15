# Build the docker image
docker build -t prophecy/maths -f Dockerfile_lib .

docker run --rm -iv${PWD}:${PWD} prophecy/maths sh -s <<EOF
chown -Rv $(id -u):$(id -g) target/release/libmaths.so
cp -va target/release/libmaths.so ${PWD}
EOF
