import { createRouter, createWebHashHistory } from 'vue-router'
import routes from './routes.js'

const createHistory = createWebHashHistory

const router = createRouter({
  scrollBehavior: () => ({ left: 0, top: 0 }),
  routes,
  history: createHistory()
})

export default router
