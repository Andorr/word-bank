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
              <div class="flex items-center">
                <icon-btn @click='goToWordSearch()'>
                    <ion-icon class="text-2xl" :icon="icons.search"></ion-icon>
                </icon-btn>
                <icon-btn @click='goToWordUpsert()'>
                    <ion-icon class="text-2xl" :icon="icons.add"></ion-icon>
                </icon-btn>
              </div>
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
        <word-item 
          v-for='word in words' 
          :key='word.id'
          :word='word'
          @click="goToWordUpsert(word.id)"
        />
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

// Store
import { ACTIONS } from '@/store/actions';
import { Word } from '@/lib/models';

// Components
import { 
  IonPage,
  IonHeader,
  IonToolbar,
  IonIcon,
  IonSpinner,
  IonGrid,
  IonRow,
  IonCol,
  IonTitle,
  IonContent,
  IonList,
  IonRefresher,
  IonRefresherContent, 
} from '@ionic/vue';
import IconBtn from '@/components/base/IconBtn.vue';
import WordItem from '@/components/WordItem.vue';

// Icons
import { add, chevronDownCircleOutline, search } from 'ionicons/icons';
import { LIST_OPTIONS } from '@/store/mutations';

export default  defineComponent({
  name: 'WordList',
  components: { 
    IonHeader,
    IonToolbar,
    IonTitle,
    IonIcon,
    IonSpinner,
    IonGrid,
    IonRow,
    IonCol,
    IonContent,
    IonPage,
    IonList,
    IonRefresher,
    IonRefresherContent,
    IconBtn,
    WordItem,
  },
  data() {
    return {
      icons: {
        add,
        chevronDownCircleOutline,
        search,
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
      return this.$store.dispatch(ACTIONS.WORD_QUERY, { listOptions: LIST_OPTIONS.OVERWRITE });
    },
    goToWordUpsert(id?: string) {
      let path = URLS.tabs.concat(URLS.words, URLS.wordsUpsert)
      if(id) {
        path = path.concat(`?id=${id}`)
      }
      this.$router.push(path)
    },
    goToWordSearch() {
      this.$router.push(URLS.tabs.concat(URLS.words, URLS.wordsSearch))
    }
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