# Instruções de revisão para o Copilot

Ao revisar um pull request deste repositório, o revisor MUST seguir o
[AGENTS.md](../AGENTS.md), principalmente as seções Reviews, Findings e Report.

Cada achado MUST seguir uma destas formas, nesta ordem:

1. Citar a regra do AGENTS.md violada, pelo arquivo e pelo nome da seção, com a
   linha inicial do código.
2. Descrever um defeito de correção, isto é, um resultado errado ou uma
   contradição com a especificação, com o arquivo e a linha inicial.
3. Como último recurso, usar o modelo exato da seção Report do AGENTS.md, com
   arquivo relativo, linha inicial e fonte.

Comentário que não siga nenhuma destas formas MUST NOT ser publicado.

Regras adicionais:

- Aplicar as regras do AGENTS.md literalmente, sem ampliá-las nem substituí-las
  pela própria leitura.
- Não reportar o que uma ferramenta declarada em `docs/TOOLS.md` já verifica por
  conta própria.
- Não reportar conteúdo fora do escopo da mudança.
- A memória do projeto MUST NOT ser usada como critério de revisão.
