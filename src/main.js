import Vue from 'vue'
import App from './App.vue'
import router from './router'
import 'vue2-toast/lib/toast.css';
import Toast from 'vue2-toast';
Vue.use(Toast, {
  defaultType: 'center',
  duration: 3000,
  wordWrap: true,
  width: '350px',
  height: '200px'
});
Vue.config.productionTip = false

new Vue({
  render: h => h(App),
  router,
}).$mount('#app')
