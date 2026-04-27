[package]
name = "icebox-llm"
version = "0.0.1"
authors = ["Pedro Henrique Esteves Neto <pedroestevesnt@gmail.com>"]
edition = "2021"
description = "Low-level LLM Sandboxing tool for secure agent execution"

[dependencies]
# 1. Syscalls e Baixo Nível (O coração do Sandbox)
# 'nix' abstrai as chamadas de kernel de forma segura
nix = { version = "0.27", features = ["sched", "user", "net", "mount", "mman", "hostname"] }

# 2. Segurança e Controle de Acesso
# Libseccomp para filtrar quais syscalls o agente de IA pode tentar
libseccomp = "0.3.0"

# 3. Execução Assíncrona (Para os serviços de monitoramento/sentinel)
# Usamos tokio para que o monitoramento não roube performance da IA
tokio = { version = "1.35", features = ["full"] }

# 4. Comunicação e Logs
# 'tracing' é o padrão ouro para logs em sistemas de segurança
tracing = "0.1"
tracing-subscriber = "0.3"
# 'anyhow' para gestão de erros limpa (fundamental para debugging de sistemas)
anyhow = "1.0"
thiserror = "1.0"

# 5. Gestão de Dados e Buffer
# Útil para lidar com as mensagens trocadas entre host e sandbox
bytes = "1.5"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

[dev-dependencies]
# Para os testes que vão tentar 'quebrar' o seu sandbox
proptest = "1.4"

[profile.release]
# Configurações de Sênior: Otimizar para segurança e binário menor
opt-level = 3
lto = true        # Link Time Optimization (binário mais rápido e enxuto)
panic = "abort"   # Em caso de erro crítico, aborte imediatamente (mais seguro para sandboxes)
strip = true      # Remove símbolos de debug do binário final