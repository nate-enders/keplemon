FROM ghcr.io/pyo3/maturin:latest

RUN yum install -y openssl-devel

ENV PATH="/opt/python/cp310-cp310/bin:$PATH"

WORKDIR /io

COPY . .

RUN maturin build --release --compatibility manylinux2014 -i python

RUN pip install --upgrade pip setuptools wheel auditwheel pytest

RUN pip install target/wheels/*.whl

ENTRYPOINT []

CMD ["bash", "-l"]