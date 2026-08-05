<script setup>
import { useInventory } from '../composables/useInventory'
const { products, editProduct, removeProduct } = useInventory()
</script>

<template>
  <div class="panel">
    <div class="panel-header">
      <h2>Produtos cadastrados</h2>
      <span class="badge">{{ products.length }} itens</span>
    </div>

    <div v-if="products.length" class="product-list">
      <article v-for="product in products" :key="product.id" class="product-card">
        <div>
          <h3>{{ product.name }}</h3>
          <p>{{ product.manufacturer }} · {{ product.brand }}</p>
          <p class="stock">Estoque: {{ product.quantity }}</p>
        </div>
        <div class="card-actions">
          <button class="ghost-btn" @click="editProduct(product)">Editar</button>
          <button class="danger-btn" @click="removeProduct(product.id)">Remover</button>
        </div>
      </article>
    </div>
    <p v-else class="empty-state">Nenhum produto cadastrado ainda.</p>
  </div>
</template>

<style scoped></style>
