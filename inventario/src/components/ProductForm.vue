<script setup>
import { useInventory } from '../composables/useInventory'
import { useAuth } from '../composables/useAuth'
const { productForm, editingProductId, handleProductSubmit, resetProductForm } = useInventory()
const { hasPermission } = useAuth()
const canCreate = hasPermission('estoque', 'can_create')
const canEdit = hasPermission('estoque', 'can_edit')
</script>

<template>
  <div class="panel">
    <div class="panel-header">
      <h2>{{ editingProductId ? 'Editar produto' : 'Adicionar produto' }}</h2>
      <button v-if="editingProductId" class="ghost-btn" @click="resetProductForm">Cancelar</button>
    </div>

    <form class="form-stack" @submit.prevent="handleProductSubmit">
      <label>
        Nome do produto
        <input v-model="productForm.name" placeholder="Ex.: Teclado mecânico" />
      </label>
      <label>
        Fabricante
        <input v-model="productForm.manufacturer" placeholder="Ex.: TechLabs" />
      </label>
      <label>
        Marca
        <input v-model="productForm.brand" placeholder="Ex.: HyperKey" />
      </label>
      <label>
        Quantidade
        <input v-model.number="productForm.quantity" type="number" min="1" />
      </label>
      <button v-if="editingProductId ? canEdit : canCreate" type="submit" class="primary-btn">{{ editingProductId ? 'Salvar alterações' : 'Cadastrar produto' }}</button>
    </form>
  </div>
</template>

<style scoped></style>
