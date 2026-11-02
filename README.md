# Simple Fog Control para Horizon (teste, ainda nao compilado)

Estrutura:
- mixins/simple-fog-control/  -> o mod (horizon-mixin.toml + fog.rs)
- .github/workflows/check.yml -> compila o Horizon modificado no GitHub Actions

Uso: crie um repositorio no GitHub com estes arquivos, aba Actions,
"Check mod against Horizon", Run workflow. Se falhar, baixe o artefato
"check-log" (ou copie o log do passo que falhou) e envie ao Claude.

Config do mod no jogo: config/simple_fog_control.txt
    start=0.5
    end=0.9
