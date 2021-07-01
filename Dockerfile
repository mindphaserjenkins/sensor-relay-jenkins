FROM balenalib/generic-armv7ahf-alpine
RUN apt install libc6-armhf-cross
#COPY target/debug/sensor-relay /sensor-relay
COPY sensor-relay /sensor-relay
WORKDIR /
CMD ["/sensor-relay"]
