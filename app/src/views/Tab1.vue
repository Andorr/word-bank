<template>
  <ion-page>
    <ion-header>
      <ion-toolbar>
        <ion-title>Tab 1</ion-title>
      </ion-toolbar>
    </ion-header>
    <ion-content :fullscreen="true">
      <ion-header collapse="condense">
        <ion-toolbar>
          <ion-title size="large">Tab 1</ion-title>
        </ion-toolbar>
      </ion-header>    
      <ExploreContainer name="Tab 1 page" />
    </ion-content>
  </ion-page>
</template>

<script lang="ts">
import { defineComponent } from 'vue';

// Components
import { IonPage, IonHeader, IonToolbar, IonTitle, IonContent} from '@ionic/vue';
import ExploreContainer from '@/components/ExploreContainer.vue';
import WordBankClient from '@/lib/WordBankClient';
import { PageResult, Word } from '@/lib/models';

export default  defineComponent({
  name: 'Tab1',
  components: { ExploreContainer, IonHeader, IonToolbar, IonTitle, IonContent, IonPage },
  data() {
    return {
      words: [] as Word[],
    }
  },
  methods: {
    refreshData() {
      const client = new WordBankClient('8Zy1M6KsAVEc8UuRu7yJ1JzXOZoD7ORT');
      client.listWords()
        .then((words: PageResult) => {
          this.words = words.results;
        })
    }
  },
  mounted() {
    this.refreshData();
  }
});
</script>