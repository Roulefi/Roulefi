import Vue from 'vue'
import Router from 'vue-router'
import index from '@/components/index'
import stake from '@/components/stake'
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

    {
      path: '/stake',
      name: 'stake',
      component: stake,
    },
  ]
});

export default router;