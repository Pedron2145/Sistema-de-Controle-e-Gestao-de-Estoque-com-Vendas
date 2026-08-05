<script setup>
import { useInventory } from '../composables/useInventory'
const { totalProducts, totalStock, sales } = useInventory()
</script>

<template>
  <header class="hero-card">
    <div>
      <p class="eyebrow">ERP básico</p>
      <h1>Controle de estoque e vendas</h1>
      <p class="subtitle">
        Cadastre produtos, ajuste informações e registre vendas com baixa automática no estoque.
      </p>
    </div>

    <div class="stats-grid">
      <div class="stat-card">
        <strong>{{ totalProducts }}</strong>
        <span>Produtos</span>
      </div>
      <div class="stat-card">
        <strong>{{ totalStock }}</strong>
        <span>Unidades em estoque</span>
      </div>
      <div class="stat-card">
        <strong>{{ sales.length }}</strong>
        <span>Vendas registradas</span>
      </div>
    </div>
  </header>
</template>

<style scoped></style>
