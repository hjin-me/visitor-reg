FROM rust:latest as builder
ENV CARGO_HOME /build/.cargo
WORKDIR /build
COPY . .
RUN cargo build --target-dir /output --release

# debian release as the same as golang image
# set TimeZone as Asia/Shanghai
# set Local as zh-hans
#FROM harbor.avlyun.org/inf/debian:bullseye
FROM debian:bullseye
RUN sed -i "s#deb.debian.org#ftp.cn.debian.org#g" /etc/apt/sources.list
RUN set -ex; \
	apt-get update; \
	apt-get install -y --no-install-recommends \
	    tzdata \
	    locales \
	    ca-certificates;
RUN locale-gen zh_CN.UTF-8; \
    update-locale zh_CN.UTF-8;
RUN cp /usr/share/zoneinfo/Asia/Shanghai /etc/localtime;
ENV TZ Asia/Shanghai
ENV LANG zh_US.utf8
COPY --from=builder /output/release/visitorreg /usr/local/bin/visitorreg
CMD ["visitorreg"]
