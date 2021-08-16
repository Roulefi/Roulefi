import Vue from 'vue'
import Router from 'vue-router'
import index from '@/components/index'
Vue.use(Router)

const router = new Router({
  mode: 'history',
  routes: [
    //管理员
    {
      path: '/',
      name: 'index',
      component: index,
    },
  ]
});

export default router;