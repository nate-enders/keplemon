FROM ghcr.io/pyo3/maturin:latest

RUN yum install -y openssl-devel

ARG OS
ARG ARCH
ARG PYTHON_VERSION

ENV PATH="/opt/python/cp${PYTHON_VERSION}-cp${PYTHON_VERSION}/bin:$PATH"

RUN cargo install cargo-make

WORKDIR /io

COPY . .

RUN cargo make build-${OS}-${ARCH}

RUN pip install --upgrade pip setuptools wheel auditwheel pytest

RUN pip install target/wheels/*.whl

ENTRYPOINT []

CMD ["bash", "-l"]