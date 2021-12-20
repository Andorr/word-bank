import { createRouter, createWebHistory } from '@ionic/vue-router';
import { RouteRecordRaw } from 'vue-router';
import URLS from '@/URLS';

// Views
import Tabs from '../views/Tabs.vue'



const routes: Array<RouteRecordRaw> = [
  {
    path: '/',
    redirect: URLS.tabs.concat(URLS.words),
  },
  {
    path: '/tabs',
    component: Tabs,
    children: [
      {
        path: '',
        redirect: URLS.tabs.concat(URLS.words),
      },
      {
        path: URLS.tabs.concat(URLS.words),
        component: () => import('@/views/WordList/WordList.vue'),
      },
      {
        path: URLS.tabs.concat(URLS.words, URLS.wordsUpsert),
        component: () => import('@/views/WordUpsert/WordUpsert.vue')
      },
      {
        path: URLS.tabs.concat(URLS.words, URLS.wordsSearch),
        component: () => import('@/views/WordSearch/WordSearch.vue')
      },
      {
        path: 'tab2',
        component: () => import('@/views/Tab2.vue')
      },
      {
        path: 'tab3',
        component: () => import('@/views/Tab3.vue')
      }
    ]
  }
]

const router = createRouter({
  history: createWebHistory(process.env.BASE_URL),
  routes
})

export default router
