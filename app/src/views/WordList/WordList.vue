<template>
  <ion-page>
    <ion-header mode="ios">
      <ion-toolbar>
        <ion-title>Words</ion-title>
      </ion-toolbar>
    </ion-header>
    <ion-content>
      <ion-header collapse="condense">
        <ion-toolbar>
          <ion-grid size="3">
            <ion-row>
              <ion-col>
                <ion-title size="large" class="">Words</ion-title>
              </ion-col>

              <ion-col size="auto" class="ion-align-self-end">
                <div class="flex items-center">
                  <icon-btn @click="goToWordSearch()">
                    <ion-icon class="text-2xl" :icon="icons.search"></ion-icon>
                  </icon-btn>
                  <icon-btn @click="goToFolderUpsert()">
                    <ion-icon
                      class="text-2xl"
                      :icon="icons.addCircle"
                    ></ion-icon>
                  </icon-btn>
                  <icon-btn @click="goToWordUpsert()">
                    <ion-icon class="text-2xl" :icon="icons.add"></ion-icon>
                  </icon-btn>
                </div>
              </ion-col>
            </ion-row>
          </ion-grid>
        </ion-toolbar>
      </ion-header>

      <ion-refresher slot="fixed" @ionRefresh="refresh($event)">
        <ion-refresher-content
          :pulling-icon="icons.chevronDownCircleOutline"
          pulling-text="Pull to refresh"
          refreshing-spinner="dots"
          refreshing-text="Refreshing..."
        >
        </ion-refresher-content>
      </ion-refresher>

      <ion-list v-if="!isLoading">
        <folder-item
          v-for="f in folders"
          :key="f.id"
          :folder="f"
          @click="openFolder(f.id)"
        />
        <word-item
          v-for="word in words"
          :key="word.id"
          :word="word"
          @click="goToWordUpsert(word.id)"
        />
      </ion-list>
      <div v-else class="items-center w-full h-full justify-center flex">
        <ion-spinner class="spinner" name="crescent" color="primary" />
      </div>
    </ion-content>
  </ion-page>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import URLS, { PATHS } from "@/URLS";

// Store
import { ACTIONS } from "@/store/actions";
import { Folder, Word } from "@/lib/models";

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
} from "@ionic/vue";
import IconBtn from "@/components/base/IconBtn.vue";
import WordItem from "@/components/WordItem.vue";
import FolderItem from "@/components/FolderItem.vue";

// Icons
import {
  add,
  addCircle,
  chevronDownCircleOutline,
  search,
} from "ionicons/icons";

// Constants
const ROOT_FOLDER = "61622651-a8d7-43e7-b9fe-b0dfb10fb527";

export default defineComponent({
  name: "WordList",
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
    FolderItem,
  },
  data() {
    return {
      icons: {
        add,
        addCircle,
        chevronDownCircleOutline,
        search,
      },

      isRefreshing: false,
      isLoading: true,

      folderId: ROOT_FOLDER,
    };
  },
  computed: {
    folder(): Folder | null {
      return this.$store.getters.getFolderById(this.folderId);
    },
    folders(): Folder[] {
      const folder = this.folder;
      if (!folder) {
        return [];
      }
      return this.$store.getters
        .getFoldersByParent(folder.id)
        .sort(
          (a: Folder, b: Folder) => -a.createdAt.localeCompare(b.createdAt)
        );
    },
    words(): Word[] {
      const folder = this.folder;
      if (!folder) {
        return [];
      }
      return this.$store.getters.getWordsByIds(folder.words);
    },
  },
  methods: {
    refresh(event: CustomEvent) {
      this.isRefreshing = true;

      this.refreshData().finally(() => {
        (event.target as unknown as { complete: Function }).complete();
        this.isRefreshing = false;
      });
    },
    refreshData() {
      return this.$store.dispatch(ACTIONS.FOLDER_GET, this.folderId);
    },
    openFolder(id: string) {
      const path = URLS.tabs.concat(URLS.folders, "/", id);
      this.$router.push(path);
    },
    goToFolderUpsert(id?: string) {
      this.$router.push(PATHS.folderUpsert(id, this.folderId));
    },
    goToWordUpsert(id?: string) {
      this.$router.push(PATHS.wordUpsert(id, this.folderId));
    },
    goToWordSearch() {
      this.$router.push(URLS.tabs.concat(URLS.words, URLS.wordsSearch));
    },
  },
  mounted() {
    console.log("Mounted! :D");
    this.isLoading = true;
    this.refreshData().finally(() => {
      this.isLoading = false;
    });
  },
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
