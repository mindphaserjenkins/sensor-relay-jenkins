FROM arm32v7/alpine
COPY target/debug/sensor-relay /sensor-relay
WORKDIR /
CMD ["/sensor-relay"]
