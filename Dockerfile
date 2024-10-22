FROM ubuntu:latest
LABEL authors="nat"

ENTRYPOINT ["top", "-b"]