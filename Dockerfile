FROM balenalib/generic-armv7ahf-alpine
#COPY target/debug/sensor-relay /sensor-relay
COPY sensor-relay /sensor-relay
WORKDIR /
CMD ["/sensor-relay"]
