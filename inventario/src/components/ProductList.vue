<script setup>
import { useInventory } from '../composables/useInventory'
import { useAuth } from '../composables/useAuth'
const { products, editProduct, removeProduct } = useInventory()
const { hasPermission } = useAuth()
const canEdit = hasPermission('estoque', 'can_edit')
const canDelete = hasPermission('estoque', 'can_delete')
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
          <h3>{{ product.name }} <span class="product-code">{{ product.code }}</span></h3>
          <p>{{ product.supplier }} · {{ product.manufacturer }} · {{ product.brand }}</p>
          <p>Posição: {{ String(product.street).padStart(2, '0') }}-{{ String(product.position).padStart(2, '0') }}-{{ String(product.level).padStart(2, '0') }}-{{ String(product.apartment).padStart(2, '0') }}</p>
          <p class="stock">Estoque: {{ product.quantity }}</p>
        </div>
        <div class="card-actions">
          <span :class="['status-badge', product.status]">{{ product.status }}</span>
          <button v-if="canEdit" class="ghost-btn" @click="editProduct(product)">Editar</button>
          <button v-if="canDelete" class="danger-btn" @click="removeProduct(product.id)">Remover</button>
        </div>
      </article>
    </div>
    <p v-else class="empty-state">Nenhum produto cadastrado ainda.</p>
  </div>
</template>

<style scoped></style>
