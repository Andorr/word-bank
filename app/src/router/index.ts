import { createRouter, createWebHistory } from '@ionic/vue-router';
import { RouteRecordRaw } from 'vue-router';
import URLS from '@/URLS';

// Views
import Tabs from '../views/Tabs.vue';


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
        path: URLS.tabs.concat(URLS.folders, URLS.foldersUpsert),
        component: () => import('@/views/FolderUpsert/FolderUpsert.vue')
      },
      {
        path: URLS.tabs.concat(URLS.folders, '/:id'),
        component: () => import('@/views/FolderView/FolderView.vue')
      },
      {
        path: URLS.tabs.concat(URLS.landing),
        component: () => import('@/views/Landing/Landing.vue')
      },
      {
        path: 'tab3',
        component: () => import('@/views/Tab3.vue')
      }
    ]
  },
  {
    path: URLS.quiz,
    component: () => import('@/views/QuizLanding/QuizLanding.vue')
  },
  {
    path: URLS.quiz.concat('/:id'),
    component: () => import('@/views/QuizView/QuizView.vue')
  },
]

const router = createRouter({
  history: createWebHistory(process.env.BASE_URL),
  routes
})

export default router
