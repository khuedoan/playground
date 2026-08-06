FROM node:24.7.0-bookworm-slim

ARG CLOUDFLARE_OS_REV=aedcda8b3066ff666f57ae28ecef7341d6c2dee7
ARG CLOUDFLARE_OS_SHA256=d0dff6a820f6e19c42eba02d937c6330de3a2c6a0de9bcb3a294262376360c93

RUN apt-get update \
    && apt-get install --yes --no-install-recommends ca-certificates curl git patch procps \
    && rm -rf /var/lib/apt/lists/* \
    && npm install --global pnpm@11.17.0

WORKDIR /opt/cloudflare-os
RUN curl --fail --location --retry 3 \
      "https://github.com/cloudflare/cloudflare-os/archive/${CLOUDFLARE_OS_REV}.tar.gz" \
      --output /tmp/cloudflare-os.tar.gz \
    && echo "${CLOUDFLARE_OS_SHA256}  /tmp/cloudflare-os.tar.gz" | sha256sum --check --strict \
    && tar --extract --gzip --file /tmp/cloudflare-os.tar.gz --strip-components=1 \
    && rm /tmp/cloudflare-os.tar.gz

COPY bind-all-interfaces.patch /tmp/bind-all-interfaces.patch
RUN patch --strip=1 < /tmp/bind-all-interfaces.patch \
    && pnpm install --frozen-lockfile \
    && rm /tmp/bind-all-interfaces.patch

EXPOSE 8787
CMD ["pnpm", "run-local"]
