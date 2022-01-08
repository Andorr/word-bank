<template>
  <page title="Quiz">
    <ion-list class="flex flex-col h-full">
      <ion-list-header class="mb-2">Quiz Options:</ion-list-header>
      <ion-item>
        <ion-label>Quiz Type</ion-label>
        <ion-select v-model="quizType" interface="action-sheet">
          <ion-select-option
            v-for="qt in quizTypes"
            :key="qt.value"
            :value="qt.value"
          >
            {{ qt.label }}
          </ion-select-option>
        </ion-select>
      </ion-item>

      <ion-item>
        <ion-label>Word Option</ion-label>
        <ion-select v-model="quizWordOption" interface="action-sheet">
          <ion-select-option
            v-for="qopt in quizWordOptions"
            :key="qopt.value"
            :value="qopt.value"
          >
            {{ qopt.label }}
          </ion-select-option>
        </ion-select>
      </ion-item>
      <ion-item v-if="isQuizWordOptionRandom" class="ml-4">
        <ion-label position="stacked"
          >Number of words: {{ wordCount }}</ion-label
        >
        <ion-range
          min="5"
          max="50"
          step="5"
          snaps="true"
          pin
          v-model="wordCount"
          color="primary"
        ></ion-range>
      </ion-item>
      <div v-else class="ml-4">
        <ion-searchbar v-model="folderQuery" debounce="500" />
        <ion-progress-bar v-if="isLoadingFolders" type="indeterminate" />
        <ion-list>
          <ion-item
            v-for="f in folders"
            :key="f.id"
            @click="toggleFolder(f.id)"
          >
            <ion-label> {{ f.name }} ({{ f.words.length }}) </ion-label>
            <ion-icon v-if="f.id === folderId" slot="end" :icon="icons.check" />
          </ion-item>
        </ion-list>
      </div>
      <div class="flex-grow" />
      <div class="mx-4 mb-4">
        <btn
          class="w-full shadow-lg"
          :disabled="!isFormValid || isLoading"
          @click="quizStart"
        >
          <ion-spinner v-if="isLoading" name="dots" />
          <span v-else>Start</span>
        </btn>
      </div>
    </ion-list>
  </page>
</template>

<script lang="ts">
import { defineComponent } from "vue";

// Store
import { Folder, PageResult, Quiz, QuizOptions, QuizType } from "@/lib/models";
import { ActionFolderQueryOptions, ACTIONS } from "@/store/actions";

// Components
import {
  IonSelect,
  IonSelectOption,
  IonLabel,
  IonItem,
  IonList,
  IonListHeader,
  IonRange,
  IonSpinner,
  IonSearchbar,
  IonProgressBar,
  IonIcon,
} from "@ionic/vue";
import Page from "@/components/layout/Page.vue";
import Btn from "@/components/base/Btn.vue";

// Types
import { QuizWordOption } from "./models";

// Icons
import { checkmarkCircle } from "ionicons/icons";

export default defineComponent({
  name: "QuizLanding",
  components: {
    Page,
    IonSelect,
    IonSelectOption,
    IonLabel,
    IonItem,
    IonList,
    IonListHeader,
    IonRange,
    IonSpinner,
    IonSearchbar,
    IonProgressBar,
    IonIcon,
    Btn,
  },
  computed: {
    isQuizWordOptionRandom(): boolean {
      return this.quizWordOption === QuizWordOption.Random;
    },
    isFormValid(): boolean {
      return this.isQuizWordOptionRandom || this.folderId !== null;
    },
  },
  data() {
    return {
      icons: {
        check: checkmarkCircle,
      },

      isLoading: false,
      isLoadingFolders: false,

      quizTypes: [
        { value: QuizType.Normal, label: "Normal" },
        { value: QuizType.Endless, label: "Endless" },
      ],
      quizType: QuizType.Normal,
      quizWordOptions: [
        { value: QuizWordOption.Folder, label: "Folder" },
        { value: QuizWordOption.Random, label: "Random" },
      ],
      quizWordOption: QuizWordOption.Folder,
      wordCount: 20,

      folderQuery: "" as string,
      folders: [] as Folder[],
      folderId: null as string | null, // Selected folder Id
    };
  },
  methods: {
    buildOptions(): QuizOptions {
      return {
        kind: this.quizType,
        words: {
          folderId: this.isQuizWordOptionRandom
            ? undefined
            : (this.folderId as string),
          count: this.isQuizWordOptionRandom ? this.wordCount : undefined,
        },
      };
    },
    quizStart() {
      this.isLoading = true;
      this.$store
        .dispatch(ACTIONS.QUIZ_START, this.buildOptions())
        .then((q: Quiz) => {
          console.log(q);
        })
        .finally(() => (this.isLoading = false));
    },
    queryFolders() {
      this.isLoadingFolders = true;
      this.$store
        .dispatch(ACTIONS.FOLDER_QUERY, {
          queryOptions: { query: this.folderQuery },
          pagination: { limit: 10 },
        } as ActionFolderQueryOptions)
        .then((result: PageResult<Folder>) => {
          this.folders = result.results.filter((f) => f.words.length > 0);
          this.folderId = null;
        })
        .finally(() => (this.isLoadingFolders = false));
    },
    toggleFolder(id: string) {
      this.folderId = this.folderId === id ? null : id;
    },
  },
  mounted() {
    this.queryFolders();
  },
  watch: {
    quizWordOption(option: QuizWordOption) {
      if (option === QuizWordOption.Folder) {
        this.queryFolders();
      }
    },
    folderQuery() {
      if (this.folderQuery) {
        this.queryFolders();
      }
    },
  },
});
</script>

<style scoped></style>
