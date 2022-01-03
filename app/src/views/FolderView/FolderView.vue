<template>
  <ion-page>
    <ion-header mode="ios">
      <ion-toolbar>
        <ion-buttons slot="start">
          <ion-back-button></ion-back-button>
        </ion-buttons>
        <div class="flex items-center" slot="end">
          <icon-btn @click="editCurrentFolder()">
            <ion-icon class="text-2xl" :icon="icons.createOutline"></ion-icon>
          </icon-btn>
          <icon-btn @click="goToFolderUpsert()">
            <ion-icon class="text-2xl" :icon="icons.addCircle"></ion-icon>
          </icon-btn>
          <icon-btn @click="goToWordUpsert()">
            <ion-icon class="text-2xl" :icon="icons.add"></ion-icon>
          </icon-btn>
        </div>
      </ion-toolbar>
    </ion-header>
    <ion-content :fullscreen="true">
      <ion-header collapse="condense">
        <ion-toolbar>
          <ion-title>
            <ion-icon class="text-xs" :icon="icons.folder" />
            <span class="ml-1">{{ name }}</span>
          </ion-title>
        </ion-toolbar>
        <ion-progress-bar v-if="isLoading" type="indeterminate" />
      </ion-header>
      <template v-if="!isLoading">
        <ion-list v-if="folders.length > 0 || words.length > 0">
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
        <div v-else class="h-full -m-12 flex items-center justify-center">
          <div class="max-w-xxs">
            <ion-img :src="icons.void" />
            <p class="text-center font-bold mt-2">
              Empty folder! <br />Let's add some words!
            </p>
          </div>
        </div>
      </template>
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
  IonTitle,
  IonContent,
  IonList,
  IonProgressBar,
  IonButtons,
  IonBackButton,
  IonImg,
} from "@ionic/vue";
import IconBtn from "@/components/base/IconBtn.vue";
import WordItem from "@/components/WordItem.vue";
import FolderItem from "@/components/FolderItem.vue";

// Icons
import { add, addCircle, createOutline, folder } from "ionicons/icons";
import VoidSvg from "@/assets/img/void.svg";

export default defineComponent({
  name: "FolderView",
  components: {
    IonHeader,
    IonToolbar,
    IonTitle,
    IonIcon,
    IonSpinner,
    IonContent,
    IonPage,
    IonList,
    IonProgressBar,
    IonButtons,
    IonBackButton,
    IonImg,
    IconBtn,
    WordItem,
    FolderItem,
  },
  data() {
    return {
      icons: {
        add,
        addCircle,
        createOutline,
        folder,
        void: VoidSvg,
      },

      isLoading: true,
      folderId: "",

      hasFetchedFolder: {} as Record<string, boolean>,
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
    name(): string {
      return this.folder ? this.folder.name : "Unknown";
    },
  },
  methods: {
    refreshData() {
      return this.$store.dispatch(ACTIONS.FOLDER_GET, this.folderId);
    },
    openFolder(id: string) {
      this.$router.push(URLS.tabs.concat(URLS.folders, "/", id));
    },
    editCurrentFolder() {
      const parentId = this.folder ? this.folder.parent : undefined;
      this.$router.push(PATHS.folderUpsert(this.folderId, parentId));
    },
    goToFolderUpsert(id?: string) {
      this.$router.push(PATHS.folderUpsert(id, this.folderId));
    },
    goToWordUpsert(id?: string) {
      this.$router.push(PATHS.wordUpsert(id, this.folderId));
    },
    reload(): Promise<void> {
      this.folderId = this.$route.params.id as string;

      if (this.hasFetchedFolder[this.folderId]) {
        return Promise.resolve();
      }

      this.isLoading = true;
      return this.refreshData()
        .then(() => {
          this.hasFetchedFolder[this.folderId] = true;
        })
        .finally(() => {
          this.isLoading = false;
        });
    },
    on404GoHome() {
      if (!this.$store.getters.getFolderById(this.folderId)) {
        this.$router.replace(URLS.tabs.concat(URLS.words));
      }
    },
  },
  mounted() {
    this.reload().finally(this.on404GoHome);
  },
  watch: {
    "$route.params.id": function (id) {
      if (id) {
        this.reload().finally(this.on404GoHome);
      }
    },
  },
});
</script>

<style scoped></style>
