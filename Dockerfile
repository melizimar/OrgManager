# Etapa 1: Construção
FROM rust:1.83 AS builder

# Definir o diretório de trabalho
WORKDIR /usr/src/app

# Copiar arquivos de configuração do Cargo
COPY Cargo.toml ./ 
COPY Cargo.lock ./ 

# Copiar somente os arquivos necessários para a compilação
COPY src/ ./src

# Baixar dependências
RUN cargo fetch

# Compilar o aplicativo no modo release
RUN cargo build --release

# Etapa 2: Produção
FROM debian:bookworm

# Instalar dependências básicas
RUN apt-get update && apt-get install -y \
    ca-certificates && \
    rm -rf /var/lib/apt/lists/* \
    libpq-dev \
    && apt-get clean

# Copiar o binário compilado da etapa anterior
COPY --from=builder /usr/src/app/target/release/org-manager /usr/local/bin/org-manager

# Definir o comando de execução
CMD ["org-manager"]