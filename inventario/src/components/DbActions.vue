<script setup>
import { useInventory } from '../composables/useInventory'
const { downloadDatabase, triggerImport, fileInput, handleImport } = useInventory()
</script>

<template>
  <div class="db-actions">
    <button class="ghost-btn" type="button" @click="downloadDatabase">Download DB</button>
    <button class="primary-btn" type="button" @click="triggerImport">import DB</button>
    <input ref="fileInput" type="file" accept="application/json" hidden @change="handleImport" />
  </div>
</template>

<style scoped></style>
