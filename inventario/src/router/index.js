import { createRouter, createWebHistory } from 'vue-router'
import InventoryPage from '../pages/InventoryPage.vue'
import SalesPage from '../pages/SalesPage.vue'
import ReportsPage from '../pages/ReportsPage.vue'
import LoginPage from '../pages/LoginPage.vue'
import FirstAccessPage from '../pages/FirstAccessPage.vue'
import AdminUsersPage from '../pages/AdminUsersPage.vue'
import DemandsPage from '../pages/DemandsPage.vue'
import { useAuth } from '../composables/useAuth'

function hasPermission(moduleName, action = 'can_view') {
  const { user, permissions } = useAuth()
  if (!user.value) return false
  if (user.value.role === 'admin') return true
  if (moduleName === 'relatorios' && user.value.role !== 'pce') return false
  if (moduleName === 'demandas' && user.value.role !== 'estoquista') return false

  const modulePermission = permissions.value.find((item) => item.module === moduleName)
  return Boolean(modulePermission?.[action])
}

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      redirect: '/login',
    },
    {
      path: '/login',
      name: 'login',
      component: LoginPage,
    },
    {
      path: '/primeiro-acesso',
      name: 'primeiro-acesso',
      component: FirstAccessPage,
    },
    {
      path: '/estoque',
      name: 'estoque',
      component: InventoryPage,
      meta: { requiresAuth: true, module: 'estoque', action: 'can_view' },
    },
    {
      path: '/vendas',
      name: 'vendas',
      component: SalesPage,
      meta: { requiresAuth: true, module: 'vendas', action: 'can_view' },
    },
    {
      path: '/demandas',
      name: 'demandas',
      component: DemandsPage,
      meta: { requiresAuth: true, module: 'demandas', action: 'can_view' },
    },
    {
      path: '/relatorios',
      name: 'relatorios',
      component: ReportsPage,
      meta: { requiresAuth: true, module: 'relatorios', action: 'can_view' },
    },
    {
      path: '/admin-usuarios',
      name: 'admin-usuarios',
      component: AdminUsersPage,
      meta: { requiresAuth: true, requiresAdmin: true },
    },
  ],
})

router.beforeEach((to, from, next) => {
  const { session, user } = useAuth()

  if (to.meta.requiresAuth && !session.value?.token) {
    return next('/login')
  }

  if (to.meta.requiresAdmin && user.value?.role !== 'admin') {
    return next('/estoque')
  }

  if (to.meta.module && !hasPermission(to.meta.module, to.meta.action || 'can_view')) {
    return next('/estoque')
  }

  next()
})

export default router
