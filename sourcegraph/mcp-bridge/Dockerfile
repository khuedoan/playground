FROM python:3.14-slim

WORKDIR /app

COPY mcp-bridge/server.py /app/server.py
COPY mcp-bridge/sourcegraph_client.py /app/sourcegraph_client.py

ENV PYTHONUNBUFFERED=1
ENV PORT=7081

EXPOSE 7081

CMD ["python", "/app/server.py"]
