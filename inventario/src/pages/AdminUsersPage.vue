<script setup>
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { useAuth } from '../composables/useAuth'

const router = useRouter()
const { user, listUsers, createUser, listPermissions, updatePermissions, error, loading, logout } = useAuth()

const users = ref([])
const selectedUserId = ref(null)
const selectedPermissions = ref({})
const form = ref({
  name: '',
  email: '',
  password: '',
  role: 'vendedor',
})

const permissionCatalog = [
  { module: 'vendas', label: 'Vendas' },
  { module: 'estoque', label: 'Estoque' },
  { module: 'relatorios', label: 'Relatórios' },
  { module: 'clientes', label: 'Clientes' },
]

async function loadUsers() {
  users.value = await listUsers()
}

function applyPermissionDefaults(role) {
  const defaults = {
    admin: [
      { module: 'vendas', can_view: true, can_create: true, can_edit: true, can_delete: true },
      { module: 'estoque', can_view: true, can_create: true, can_edit: true, can_delete: true },
      { module: 'relatorios', can_view: true, can_create: true, can_edit: true, can_delete: true },
      { module: 'clientes', can_view: true, can_create: true, can_edit: true, can_delete: true },
    ],
    vendedor: [
      { module: 'vendas', can_view: true, can_create: true, can_edit: false, can_delete: false },
      { module: 'estoque', can_view: true, can_create: false, can_edit: false, can_delete: false },
      { module: 'relatorios', can_view: true, can_create: false, can_edit: false, can_delete: false },
      { module: 'clientes', can_view: true, can_create: true, can_edit: false, can_delete: false },
    ],
  }

  return defaults[role] ?? defaults.vendedor
}

async function submitUser() {
  try {
    const permissions = applyPermissionDefaults(form.value.role)
    await createUser({ ...form.value, permissions })
    form.value = { name: '', email: '', password: '', role: 'vendedor' }
    await loadUsers()
  } catch {
    // erro já tratado
  }
}

async function selectUser(userId) {
  selectedUserId.value = userId
  const permissions = await listPermissions(userId)

  const grouped = {}
  for (const permission of permissions) {
    grouped[permission.module] = { ...permission }
  }

  selectedPermissions.value = grouped
}

async function savePermissions() {
  if (!selectedUserId.value) return

  const permissions = Object.values(selectedPermissions.value)

  await updatePermissions({
    userId: selectedUserId.value,
    permissions,
  })
}

async function leave() {
  await logout()
  router.push('/login')
}

onMounted(async () => {
  if (!user.value) {
    router.push('/login')
    return
  }
  await loadUsers()
})
</script>

<template>
  <div class="page-shell">
    <header class="hero-card">
      <div>
        <p class="eyebrow">Administração</p>
        <h1>Gerenciar usuários</h1>
        <p class="subtitle">Crie usuários e atribua permissões por perfil.</p>
      </div>

      <button class="secondary-btn" @click="leave">Sair</button>
    </header>

    <section class="panel-grid">
      <div class="panel">
        <div class="panel-header">
          <h2>Cadastrar novo usuário</h2>
          <span class="badge">Protegido</span>
        </div>

        <form class="form-stack" @submit.prevent="submitUser">
          <label>
            Nome
            <input v-model="form.name" type="text" required />
          </label>

          <label>
            E-mail
            <input v-model="form.email" type="email" required />
          </label>

          <label>
            Senha
            <input v-model="form.password" type="password" required />
          </label>

          <label>
            Perfil
            <select v-model="form.role">
              <option value="vendedor">Vendedor</option>
              <option value="admin">Administrador</option>
            </select>
          </label>

          <p v-if="error" class="error-text">{{ error }}</p>

          <button type="submit" class="primary-btn" :disabled="loading">
            {{ loading ? 'Salvando...' : 'Criar usuário' }}
          </button>
        </form>
      </div>

      <div class="panel">
        <div class="panel-header">
          <h2>Usuários cadastrados</h2>
          <span class="badge">{{ users.length }}</span>
        </div>

        <div v-if="users.length" class="user-list">
          <button
            v-for="item in users"
            :key="item.id"
            type="button"
            class="user-card"
            :class="{ selected: selectedUserId === item.id }"
            @click="selectUser(item.id)"
          >
            <div>
              <strong>{{ item.name }}</strong>
              <p>{{ item.email }}</p>
            </div>
            <span :class="['role-badge', item.role]">{{ item.role }}</span>
          </button>
        </div>
        <p v-else class="empty-state">Nenhum usuário cadastrado.</p>
      </div>
    </section>

    <section v-if="selectedUserId" class="panel permission-panel">
      <div class="panel-header">
        <h2>Permissões do usuário</h2>
        <button type="button" class="primary-btn" @click="savePermissions">Salvar permissões</button>
      </div>

      <div class="permission-grid">
        <div v-for="module in permissionCatalog" :key="module.module" class="permission-card">
          <h3>{{ module.label }}</h3>

          <label class="check-row">
            <input v-model="selectedPermissions[module.module].can_view" type="checkbox" />
            <span>Visualizar</span>
          </label>

          <label class="check-row">
            <input v-model="selectedPermissions[module.module].can_create" type="checkbox" />
            <span>Criar</span>
          </label>

          <label class="check-row">
            <input v-model="selectedPermissions[module.module].can_edit" type="checkbox" />
            <span>Editar</span>
          </label>

          <label class="check-row">
            <input v-model="selectedPermissions[module.module].can_delete" type="checkbox" />
            <span>Excluir</span>
          </label>
        </div>
      </div>
    </section>
  </div>
</template>

<style scoped>
.page-shell {
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.hero-card {
  background: linear-gradient(135deg, #0f172a 0%, #1d4ed8 100%);
  color: white;
  border-radius: 24px;
  padding: 24px;
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 20px;
}

.eyebrow {
  margin: 0 0 8px;
  font-size: 0.72rem;
  letter-spacing: 0.2em;
  text-transform: uppercase;
  opacity: 0.8;
}

h1 {
  margin: 0;
  font-size: 1.8rem;
}

.subtitle {
  margin: 8px 0 0;
  opacity: 0.92;
}

.secondary-btn,
.primary-btn {
  border: none;
  border-radius: 12px;
  padding: 10px 14px;
  cursor: pointer;
  font-weight: 700;
}

.secondary-btn {
  background: rgba(255, 255, 255, 0.14);
  color: white;
}

.primary-btn {
  background: linear-gradient(135deg, #2563eb 0%, #1d4ed8 100%);
  color: white;
}

.form-stack {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

label {
  display: flex;
  flex-direction: column;
  gap: 6px;
  font-weight: 600;
}

input,
select {
  border: 1px solid #dbe3f0;
  border-radius: 10px;
  background: #f8fafc;
  padding: 10px 12px;
}

.panel-grid {
  display: grid;
  grid-template-columns: 1.2fr 1fr;
  gap: 20px;
}

.panel {
  background: white;
  border-radius: 20px;
  padding: 20px;
  box-shadow: 0 10px 30px rgba(15, 23, 42, 0.08);
}

.panel-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 14px;
}

.panel-header h2 {
  margin: 0;
  font-size: 1.1rem;
}

.badge {
  padding: 6px 10px;
  background: #e0e7ff;
  color: #3730a3;
  border-radius: 999px;
  font-size: 0.8rem;
}

.user-list {
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.user-card {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 16px;
  padding: 12px 14px;
  border: 1px solid #e2e8f0;
  border-radius: 14px;
  background: #f8fafc;
}

.user-card p {
  margin: 4px 0 0;
  color: #475569;
}

.role-badge {
  padding: 6px 10px;
  border-radius: 999px;
  font-size: 0.75rem;
  font-weight: 700;
  text-transform: capitalize;
}

.role-badge.admin {
  background: #dbeafe;
  color: #1d4ed8;
}

.role-badge.vendedor {
  background: #dcfce7;
  color: #166534;
}

.empty-state {
  margin: 0;
  color: #64748b;
}

.error-text {
  margin: 0;
  color: #b91c1c;
}
</style>
