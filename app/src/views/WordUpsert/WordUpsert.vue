<template>
  <ion-page>
    <ion-header>
      <ion-toolbar>
        <ion-buttons slot="start">
          <ion-back-button></ion-back-button>
        </ion-buttons>
        <ion-title>{{ id ? "Save" : "Create" }} Word</ion-title>
        <icon-btn v-if="id" slot="end" @click="deleteWord">
          <ion-icon :icon="icons.trash" />
        </icon-btn>
      </ion-toolbar>
    </ion-header>
    <ion-content class="relative">
      <div class="flex flex-col min-h-full">
        <ion-item>
          <ion-label position="stacked"> Word </ion-label>
          <ion-input placeholder="Word" v-model="word"></ion-input>
        </ion-item>
        <ion-item>
          <ion-label position="stacked"> Word Type </ion-label>
          <ion-select v-model="wordType" interface="action-sheet">
            <ion-select-option
              v-for="wt in wordTypes"
              :key="wt.value"
              :value="wt.value"
            >
              {{ wt.label }}
            </ion-select-option>
          </ion-select>
        </ion-item>
        <ion-item>
          <ion-label position="stacked"> Translations </ion-label>

          <icon-btn slot="end">
            <ion-icon :icon="icons.add" @click="addTranslation" />
          </icon-btn>
          <div v-for="t in translations" :key="t.id">
            <div class="grid grid-cols-7 px-6">
              <ion-input
                class="col-span-6"
                v-model="t.value"
                placeholder="Word"
              />
              <icon-btn slot="end" @click="deleteTranslation(t.id)">
                <ion-icon :icon="icons.trash" />
              </icon-btn>
            </div>
          </div>
        </ion-item>
        <div class="flex-grow" />
        <div class="mx-4 mb-4">
          <btn
            class="w-full shadow-lg"
            :disabled="!isFormValid || isLoading"
            @click="upsertWord"
          >
            <ion-spinner v-if="isLoading" name="dots" />
            <span v-else>
              {{ id ? "Save" : "Create" }}
            </span>
          </btn>
        </div>
      </div>
    </ion-content>
  </ion-page>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { v4 as uuidv4 } from "uuid";

// Logic
import { Translation, Word } from "@/lib/models";
import { ACTIONS } from "@/store/actions";

// Components
import {
  IonPage,
  IonHeader,
  IonToolbar,
  IonTitle,
  IonContent,
  IonButtons,
  IonBackButton,
  IonItem,
  IonInput,
  IonLabel,
  IonIcon,
  IonSelect,
  IonSelectOption,
  IonSpinner,
  alertController,
} from "@ionic/vue";
import IconBtn from "@/components/base/IconBtn.vue";
import Btn from "@/components/base/Btn.vue";

// Icons
import { trash, add } from "ionicons/icons";

export default defineComponent({
  name: "WordUpsert",
  components: {
    IonPage,
    IonHeader,
    IonToolbar,
    IonTitle,
    IonContent,
    IonButtons,
    IonBackButton,
    IonInput,
    IonItem,
    IonLabel,
    IonIcon,
    IonSelect,
    IonSelectOption,
    IonSpinner,
    IconBtn,
    Btn,
  },
  data() {
    return {
      icons: {
        trash,
        add,
      },

      isLoading: false,
      id: null as string | null,
      word: "",
      translations: [] as Translation[],
      wordType: "NOUN", // TODO: Make this an enum
      wordTypes: [
        { value: "NOUN", label: "Noun" },
        { value: "PRONOUN", label: "Pronoun" },
        { value: "VERB", label: "Verb" },
        { value: "ADJECTIVE", label: "Adjective" },
        { value: "ADVERB", label: "Adverb" },
        { value: "PREPOSITION", label: "Preposition" },
        { value: "CONJUNCTION", label: "Conjunction" },
        { value: "INTERJECTION", label: "Interjection" },
      ],
    };
  },
  computed: {
    isFormValid(): boolean {
      return (
        this.word.length > 0 &&
        this.wordType !== null &&
        this.translations.filter((t) => t.value).length > 0
      );
    },
  },
  methods: {
    addTranslation(): void {
      this.translations.push(new Translation(uuidv4(), ""));
    },
    deleteTranslation(id: string): void {
      const translations = [...this.translations];
      const index = translations.findIndex((t) => t.id === id);
      if (index === -1) {
        return;
      }
      this.translations.splice(index, 1);
    },
    upsertWord(): void {
      if (!this.isFormValid) {
        return;
      }
      this.isLoading = true;

      const word = Word.fromObject({
        id: this.id as string,
        value: this.word,
        class: this.wordType,
        translations: this.translations,
      });

      const actions = this.id ? ACTIONS.WORD_UPDATE : ACTIONS.WORD_INSERT;
      this.$store
        .dispatch(actions, {
          word,
          folderId: this.$route.query.parent as string,
        })
        .then(() => {
          this.$router.back();
        })
        .catch(() => {
          // TODO: Add error
          this.isLoading = false;
        });
    },
    async deleteWord() {
      if (!this.id) {
        return;
      }

      const alert = await alertController.create({
        header: `Delete word '${this.word}'?`,
        message: `Are you sure you want to delete the word '${this.word}'?`,
        buttons: [
          "Cancel",
          {
            text: "Yes",
            role: "yes",
          },
        ],
      });
      await alert.present();

      const { role } = await alert.onDidDismiss();
      if (role === "yes") {
        this.isLoading = true;
        this.$store
          .dispatch(ACTIONS.WORD_DELETE, this.id)
          .then(() => {
            this.$router.back();
          })
          .catch(() => {
            // TODO: Add error
            this.isLoading = false;
          });
      }
    },
    mountWord(id: string) {
      const word: Word | null = this.$store.getters.getWordById(id);
      if (!word) {
        return;
      }
      this.id = word.id;
      this.word = word.value;
      this.translations = word.translations.map(
        (t) => new Translation(t.id, t.value)
      );
      this.wordType = word.classType;
    },
  },
  mounted() {
    if (this.$route.query.id) {
      this.mountWord(this.$route.query.id as string);
    } else {
      this.addTranslation();
    }
  },
});
</script>

<style scoped></style>
