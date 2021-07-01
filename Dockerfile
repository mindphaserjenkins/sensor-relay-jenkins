FROM resin/raspberrypi3-alpine-python
COPY target/debug/sensor-relay /sensor-relay
WORKDIR /
CMD ["/sensor-relay"]
