# Inventário

Aplicação de controle de estoque e vendas desenvolvida com Vue 3, Vite e Tauri, com foco em simplicidade, usabilidade e persistência local de dados.

Este projeto foi criado com a finalidade de ser um aprendizado para o uso das IAs em projetos

## ✨ O que a aplicação faz

- Cadastro de produtos com nome, fabricante, marca e quantidade
- Edição e remoção de produtos
- Registro de vendas com baixa automática no estoque
- Validação de regras de negócio, como estoque insuficiente e campos obrigatórios
- Persistência em armazenamento local do navegador
- Exportação e importação de dados em formato JSON para backup e migração
- Interface visual organizada com painel de estatísticas e feedback ao usuário

## 🛠️ Tecnologias usadas

- Vue 3 + Composition API
- Vite
- Tauri para empacotamento como aplicação desktop
- JavaScript moderno
- LocalStorage para persistência

## 🧠 Destaques do projeto

Este projeto reforça conceitos importantes de desenvolvimento frontend e arquitetura de aplicações pequenas:

- Organização de estado com composables
- Separação entre lógica de negócio e componentes de interface
- Regras de validação para evitar inconsistências
- Experiência de uso com feedback visual e fluxo simples
- Base preparada para evoluir para funcionalidades mais robustas, como autenticação, relatórios e integração com banco de dados

## 📁 Estrutura principal

- src/components: componentes visuais da interface
- src/composables/useInventory.js: núcleo da lógica do inventário
- src/App.vue: composição principal da aplicação
- src-tauri: empacotamento e integração com Tauri

## ▶️ Como executar localmente

No diretório do projeto:

```bash
cd inventario
npm install
npm run dev
```

Para gerar a build de produção:

```bash
npm run build
```

## 🚀 Como rodar como desktop com Tauri

```bash
npm run tauri dev
```

## 📌 Objetivo do projeto

Este projeto tem por finalidade o aprendizado sobre o desenvolvimento de soluções com o auxílio de IAs e testar novas tecnologias.

## 🔮 Próximos passos

- Implementar autenticação de usuário
- Adicionar relatórios e gráficos de vendas
- Integrar com banco de dados real
- Criar filtros e busca avançada por produtos
- Melhorar a experiência mobile e desktop

