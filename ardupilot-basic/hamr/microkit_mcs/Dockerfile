FROM jasonbelt/microkit_provers:2026.08.18

ARG VERUS_VERSION=0.2026.08.23.fbbbbcf

USER root
RUN apt-get update -q \
    && apt-get install -y --no-install-recommends ca-certificates curl unzip \
    && apt-get -y clean \
    && rm -rf /var/lib/apt/lists/*

RUN echo "Verus: Install Verus ${VERUS_VERSION}" && \
    curl --fail --location --show-error \
        "https://github.com/verus-lang/verus/releases/download/release/${VERUS_VERSION}/verus-${VERUS_VERSION}-x86-linux.zip" \
        --output /tmp/verus.zip \
    && unzip -q /tmp/verus.zip -d /tmp \
    && rm -rf /home/microkit/provers/verus \
    && mv /tmp/verus-x86-linux /home/microkit/provers/verus \
    && chown -R --reference=/home/microkit /home/microkit/provers/verus \
    && rm -f /tmp/verus.zip

ENV MICROKIT_BOARD=zcu102

USER microkit
WORKDIR /home/microkit
