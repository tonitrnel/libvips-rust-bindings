# (c) Copyright 2019-2024 OLX
FROM rust:1.80-alpine3.20

ENV VIPS_VERSION=8.15.3

RUN apk add --quiet --update --no-cache --repository=https://dl-cdn.alpinelinux.org/alpine/v3.20/main \
  wget\
  bash\
  build-base \
  pkgconfig \
  clang \
  clang-libclang \
  meson \
  ninja \
  cgif-dev \
  expat-dev \
  fftw-dev \
  giflib-dev \
  glib-dev \
  gobject-introspection-dev \
  lcms2-dev \
  libexif-dev \
  libgsf-dev \
  libheif-dev \
  libimagequant-dev \
  libjpeg-turbo-dev \
  libpng-dev \
  librsvg-dev \
  libwebp-dev \
  openssl-dev \
  orc-dev \
  pango-dev \
  tiff-dev

COPY compile.sh /
COPY introspect.c /
COPY vips.h /

RUN chmod +x /compile.sh

RUN wget -q https://github.com/libvips/libvips/releases/download/v${VIPS_VERSION}/vips-${VIPS_VERSION}.tar.xz

RUN mkdir vips && tar xJf vips-${VIPS_VERSION}.tar.xz -C vips --strip-components 1 \
  && cd /vips \
  && meson setup build \
  && cd /vips/build \
  && meson compile >/dev/null \
  && meson install > /dev/null \
  && ldconfig /etc/ld.so.conf.d \
  && rm -rf vips \
  && rm -f vips-${VIPS_VERSION}.tar.xz \
  && rustup component add rustfmt
