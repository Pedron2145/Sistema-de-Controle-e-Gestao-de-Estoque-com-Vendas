<script setup>
import { ref, computed, onMounted } from 'vue'
import { RouterView, useRoute, useRouter } from 'vue-router'
import { useAuth } from './composables/useAuth'

const isMenuOpen = ref(true)
const route = useRoute()
const router = useRouter()
const { user, isAuthenticated, hasPermission, checkFirstAccess, logout } = useAuth()

const navItems = computed(() => {
  const items = []

  if (hasPermission('estoque', 'can_view')) {
    items.push({ label: 'Estoque', path: '/estoque', icon: '📦' })
  }

  if (hasPermission('vendas', 'can_view')) {
    items.push({ label: 'Vendas', path: '/vendas', icon: '🛒' })
  }

  if (hasPermission('relatorios', 'can_view')) {
    items.push({ label: 'Relatórios', path: '/relatorios', icon: '📊' })
  }

  if (user.value?.role === 'admin') {
    items.push({ label: 'Usuários', path: '/admin-usuarios', icon: '👥' })
  }

  return items
})

function toggleMenu() {
  isMenuOpen.value = !isMenuOpen.value
}

function goTo(path) {
  router.push(path)
}

async function bootApp() {
  const status = await checkFirstAccess()
  if (!status) {
    router.push('/login')
    return
  }

  if (status.needs_first_admin) {
    router.push('/primeiro-acesso')
    return
  }

  if (!isAuthenticated.value) {
    router.push('/login')
  }
}

onMounted(() => {
  bootApp()
})

async function handleLogout() {
  await logout()
  router.push('/login')
}
</script>

<template>
  <div v-if="route.path === '/login' || route.path === '/primeiro-acesso'" class="standalone-page">
    <RouterView />
  </div>

  <div v-else class="app-shell">
    <aside :class="['sidebar', { 'sidebar-collapsed': !isMenuOpen }]">
      <button class="menu-toggle" @click="toggleMenu" aria-label="Alternar menu lateral">
        <span>{{ isMenuOpen ? '‹' : '›' }}</span>
      </button>

      <div v-if="isMenuOpen" class="nav-panel">
        <div class="brand">
          <div class="brand-mark">ERP</div>
          <div>
            <strong>ERP Basic</strong>
            <small>{{ user?.role === 'admin' ? 'Administrador' : 'Usuário' }}</small>
          </div>
        </div>

        <nav class="nav-list">
          <button
            v-for="item in navItems"
            :key="item.path"
            type="button"
            :class="['nav-item', { active: route.path === item.path }]"
            @click="goTo(item.path)"
          >
            <span class="nav-icon">{{ item.icon }}</span>
            <span>{{ item.label }}</span>
          </button>
        </nav>

        <button class="logout-btn" @click="handleLogout">Sair</button>
      </div>
    </aside>

    <main class="content-area">
      <RouterView />
    </main>
  </div>
</template>

<style>
:global(body) {
  margin: 0;
  font-family: Inter, 'Segoe UI', sans-serif;
  background: linear-gradient(135deg, #f3f6ff 0%, #eef4ff 100%);
  color: #14213d;
}

* {
  box-sizing: border-box;
}

button,
input,
select {
  font: inherit;
}

.app-shell {
  display: flex;
  min-height: 100vh;
  background: linear-gradient(135deg, #f8fbff 0%, #edf4ff 100%);
}

.sidebar {
  position: relative;
  width: 260px;
  background: rgba(255, 255, 255, 0.9);
  border-right: 1px solid #dbe7ff;
  box-shadow: 8px 0 30px rgba(15, 23, 42, 0.04);
  transition: width 0.2s ease;
}

.sidebar-collapsed {
  width: 56px;
}

.menu-toggle {
  position: absolute;
  top: 18px;
  right: -15px;
  width: 30px;
  height: 30px;
  border: none;
  border-radius: 50%;
  background: linear-gradient(135deg, #2563eb 0%, #1d4ed8 100%);
  color: white;
  font-size: 1.2rem;
  cursor: pointer;
  box-shadow: 0 8px 22px rgba(37, 99, 235, 0.35);
  z-index: 2;
}

.nav-panel {
  padding: 24px 16px 20px;
}

.brand {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 10px 8px 18px;
  border-bottom: 1px solid #edf2ff;
  margin-bottom: 18px;
}

.brand-mark {
  width: 42px;
  height: 42px;
  display: grid;
  place-items: center;
  border-radius: 12px;
  background: linear-gradient(135deg, #2563eb 0%, #1d4ed8 100%);
  color: white;
  font-weight: 700;
}

.brand strong,
.brand small {
  display: block;
}

.brand small {
  color: #64748b;
}

.nav-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.nav-item {
  width: 100%;
  display: flex;
  align-items: center;
  gap: 12px;
  border: none;
  border-radius: 12px;
  padding: 12px 14px;
  background: transparent;
  color: #1e293b;
  font-weight: 600;
  text-align: left;
  cursor: pointer;
  transition: all 0.2s ease;
}

.nav-item:hover,
.nav-item.active {
  background: linear-gradient(135deg, #eff6ff 0%, #dbeafe 100%);
  color: #1d4ed8;
}

.nav-icon {
  width: 28px;
  height: 28px;
  display: grid;
  place-items: center;
  border-radius: 10px;
  background: rgba(37, 99, 235, 0.08);
}

.content-area {
  flex: 1;
  padding: 24px;
}

.hero-card,
.panel,
.feedback,
.stat-card,
.table-wrap,
.alert-item {
  border-radius: 20px;
}

@media (max-width: 900px) {
  .app-shell {
    flex-direction: column;
  }

  .sidebar {
    width: 100%;
    border-right: none;
    border-bottom: 1px solid #dbe7ff;
  }

  .sidebar-collapsed {
    width: 100%;
  }

  .menu-toggle {
    right: 18px;
  }

  .content-area {
    padding: 16px;
  }
}

.standalone-page {
  min-height: 100vh;
}

.logout-btn {
  margin-top: 18px;
  width: 100%;
  border: none;
  border-radius: 12px;
  padding: 11px 14px;
  font-weight: 700;
  background: #eff6ff;
  color: #1d4ed8;
  cursor: pointer;
}
</style>
