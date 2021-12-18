<template>
  <ion-page>
    <ion-header mode="ios">
      <ion-toolbar>
        <ion-title>Words</ion-title>
      </ion-toolbar>
    </ion-header>
    <ion-content :fullscreen="true">
      <ion-header collapse="condense">
        <ion-toolbar>
          <ion-grid size="3">
            <ion-row>
              <ion-col>
                <ion-title size="large" class="">Words</ion-title>
              </ion-col>

            <ion-col size="auto" class="ion-align-self-end">
              <icon-btn @click='goToWordUpsert()'>
                  <ion-icon class="text-2xl" :icon="icons.add"></ion-icon>
              </icon-btn>
            </ion-col>
            </ion-row>
          </ion-grid>
        </ion-toolbar>
      </ion-header>

      <ion-refresher slot="fixed" @ionRefresh='refresh($event)'>
        <ion-refresher-content
          :pulling-icon="icons.chevronDownCircleOutline"
          pulling-text="Pull to refresh"
          refreshing-spinner="dots"
          refreshing-text="Refreshing...">
        </ion-refresher-content>
      </ion-refresher>

      <ion-list v-if='!isLoading'>
        <ion-item 
          v-for='word in words' 
          :key='word.id'
          class="ion-activatable ripple-parent"
          @click="goToWordUpsert(word.id)"
        >
          <div class="w-full my-2">
            <h4 class="mb-0 font-bold">
              {{ word.value }}
            </h4>
            <p class="text-gray-300 mb-0">
              {{ word.translations.map(t => t.value).join(', ') }}
            </p>
            <ion-ripple-effect></ion-ripple-effect>
          </div>
        </ion-item>
      </ion-list>
      <div v-else class="items-center w-full h-full justify-center flex">
        <ion-spinner class="spinner" name='crescent' color='primary' />
      </div>
    
    </ion-content>
  </ion-page>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import URLS from '@/URLS';

import { ACTIONS } from '@/store/actions';

// Components
import { 
  IonPage,
  IonHeader,
  IonToolbar,
  IonRippleEffect,
  IonIcon,
  IonSpinner,
  IonGrid,
  IonRow,
  IonCol,
  IonTitle,
  IonContent,
  IonItem,
  IonList,
  IonRefresher,
  IonRefresherContent, 
} from '@ionic/vue';
import IconBtn from '@/components/base/IconBtn.vue';
import { Word } from '@/lib/models';

// Icons
import { add, chevronDownCircleOutline } from 'ionicons/icons';
import { LIST_OPTIONS } from '@/store/mutations';

export default  defineComponent({
  name: 'Tab1',
  components: { 
    IonHeader,
    IonToolbar,
    IonTitle,
    IonRippleEffect,
    IonIcon,
    IonSpinner,
    IonGrid,
    IonRow,
    IonCol,
    IonContent,
    IonPage,
    IonItem,
    IonList,
    IonRefresher,
    IonRefresherContent,
    IconBtn,
  },
  data() {
    return {
      icons: {
        add,
        chevronDownCircleOutline,
      },

      isRefreshing: false,
      isLoading: true,
    }
  },
  computed: {
    words(): Word[] {
      return this.$store.getters.getWords;
    }
  },
  methods: {
    refresh(event: CustomEvent) {
      this.isRefreshing = true;

      this.refreshData()
        .finally(() => {
          (event.target as unknown as {complete: Function}).complete();
          this.isRefreshing = false;
        })
    },
    refreshData() {
      return this.$store.dispatch(ACTIONS.WORD_LIST, { listOptions: LIST_OPTIONS.OVERWRITE });
    },
    goToWordUpsert(id?: string) {
      let path = URLS.tabs.concat(URLS.words, URLS.wordsUpsert)
      if(id) {
        path = path.concat(`?id=${id}`)
      }
      this.$router.push(path)
    },
  },
  mounted() {
    this.isLoading = true;
    this.refreshData()
      .finally(() => {
        this.isLoading = false;
      });
  }
});
</script>

<style scoped>
.ripple-parent {
    position: relative;
    overflow: hidden;
}

.spinner {
  transform: scale(1.6);
}
</style>