import { Store } from 'vuex'
import { State } from './store'

declare module '*.vue' {
  import { defineComponent } from 'vue'
  const component: ReturnType<typeof defineComponent>
  export default component
}

declare module '@vue/runtime-core' {

  interface ComponentCustomProperties {
    $store: Store<State>;
  }

}