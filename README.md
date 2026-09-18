# Inventário

Aplicação de controle de estoque e vendas desenvolvida com Vue 3, Vite e Tauri, com foco em simplicidade, usabilidade e persistência centralizada em MySQL.


## ✨ O que a aplicação faz

- Cadastro de produtos com nome, fabricante, marca e quantidade
- Edição e remoção de produtos
- Registro de vendas com vários produtos, remoção de itens antes da confirmação e baixa automática no estoque
- Validação de regras de negócio, como estoque insuficiente e campos obrigatórios
- Persistência centralizada em MySQL executado com Docker
- Sessões, usuários e permissões armazenados no MySQL
- Interface visual organizada com painel de estatísticas e feedback ao usuário

## 🛠️ Tecnologias usadas

- Vue 3 + Composition API
- Vite
- Tauri para empacotamento como aplicação desktop
- JavaScript moderno
- MySQL 8.4 via Docker Compose
- SQLx no backend Rust para acesso ao banco
- Argon2 para hash de senhas

## 🧠 Destaques do projeto

Este projeto reforça conceitos importantes de desenvolvimento frontend e arquitetura de aplicações pequenas:

- Organização de estado com composables
- Separação entre lógica de negócio e componentes de interface
- Regras de validação para evitar inconsistências
- Experiência de uso com feedback visual e fluxo simples
- Backend Tauri como única camada de acesso ao banco, sem conexão direta do Vue com o MySQL

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

### Migração de vendas existentes

As vendas agora usam `sales` como cabeçalho e `sale_items` para os produtos. Para um banco já criado, execute `db/migrate_sales_items.sql` uma vez com o MySQL em execução. Bancos novos recebem o modelo automaticamente pelo Docker Compose.

## 🚀 Como rodar como desktop com Tauri

```bash
npm run tauri dev
```

## 📌 Objetivo do projeto

Desenvolvido apenas por uma pessoa com o auxílio da IA gratuita Github Copilot, visando melhorar as própias capacitações para aplicar e entender regras ne negócios, se baseando em empresas que buscam uma aplicação leve e simples para lidar com seu estoque, com foco central na arquitetura de "Endereçamento de Estoque" e vendas sem um PDV.

## 🔮 Próximos passos

- Implementar autenticação de usuário
- Adicionar relatórios e gráficos de vendas
- Criar filtros e busca avançada por produtos
- Melhorar a experiência mobile e desktop

