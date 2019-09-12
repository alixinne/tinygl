import Vue from 'vue'
import App from './App.vue'
import Buefy from 'buefy'
import VueStringFilter from 'vue-string-filter'
import store from './store'

Vue.config.productionTip = false

Vue.use(Buefy)
Vue.use(VueStringFilter)

new Vue({
  store,
  render: h => h(App)
}).$mount('#app')
