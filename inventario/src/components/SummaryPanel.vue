<script setup>
import { useInventory } from '../composables/useInventory'
const { lowStockProducts, sales } = useInventory()
</script>

<template>
  <div class="panel">
    <div class="panel-header">
      <h2>Resumo</h2>
      <span class="badge danger">{{ lowStockProducts.length }} com estoque baixo</span>
    </div>

    <div v-if="lowStockProducts.length" class="warning-list">
      <p v-for="product in lowStockProducts" :key="product.id">{{ product.name }} · {{ product.quantity }} unidade(s)</p>
    </div>
    <p v-else class="empty-state">Estoque saudável para todos os produtos.</p>

    <div class="sales-list">
      <h3>Últimas vendas</h3>
      <div v-if="sales.length" class="sale-card-list">
        <article v-for="sale in sales.slice(0, 5)" :key="sale.id" class="sale-card">
          <strong>{{ sale.productName }}</strong>
          <p>{{ sale.clientName }} · {{ sale.customerType }}</p>
          <p>{{ sale.quantity }} unidade(s) · {{ sale.date }}</p>
        </article>
      </div>
      <p v-else class="empty-state">Nenhuma venda registrada até o momento.</p>
    </div>
  </div>
</template>

<style scoped></style>
