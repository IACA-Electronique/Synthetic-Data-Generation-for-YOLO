FROM python:3.14-trixie

WORKDIR /app

RUN apt update && apt install -y libgl1 &&\
    curl -LsSf https://hf.co/cli/install.sh | bash &&\
    pip install PyYAML opencv-python

COPY visualize_obb.py .

ENTRYPOINT [ "/usr/bin/bash" ]