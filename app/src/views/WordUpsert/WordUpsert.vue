<template>
    <ion-page>
        <ion-header>
            <ion-toolbar>
                <ion-buttons slot="start">
                    <ion-back-button></ion-back-button>
                </ion-buttons>
                <ion-title>Insert Word</ion-title>
            </ion-toolbar>
        </ion-header>
        <ion-content class="relative">
            <div class="mb-24">
                <ion-item>
                    <ion-label
                        position="stacked"
                    >
                    Word
                    </ion-label>
                    <ion-input placeholder="Word" v-model='word' ></ion-input>
                </ion-item>
                <ion-item>
                    <ion-label
                        position="stacked"
                    >
                    Word Type
                    </ion-label>
                    <ion-select
                        :value="wordType"
                        interface="action-sheet"
                    >
                        <ion-select-option
                            v-for='wt in wordTypes' 
                            :key='wt.value'
                            :value='wt.value'
                        >
                        {{ wt.label }}
                        </ion-select-option>
                    </ion-select>
                </ion-item>
                <ion-item>
                    <ion-label
                        position="stacked"
                    >
                    Translations
                    </ion-label>

                    <icon-btn slot='end'>
                        <ion-icon :icon='icons.add' @click='addTranslation' />
                    </icon-btn>
                    <template v-for='t in translations' :key='t.id' >
                        <div class="grid grid-cols-7 px-6">
                            <ion-input class="col-span-6" v-model='t.value' placeholder="Word" />
                            <icon-btn slot='end' @click='deleteTranslation(t.id)'>
                                <ion-icon :icon='icons.trash' />
                            </icon-btn>
                        </div>
                    </template>
                </ion-item>

            </div>
            <div class="fixed bottom-8 w-full px-2 z-20">
                <btn class="w-full shadow-lg" :disabled='!isFormValid' @click='insertWord'>
                    <ion-spinner v-if='isLoading' name="dots" />
                    <span v-else>Create</span>
                </btn>
            </div>
        </ion-content>
    </ion-page>
</template>

<script lang="ts">
import { defineComponent } from 'vue'
import { v4 as uuidv4 } from 'uuid';

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
} from '@ionic/vue';
import IconBtn from '@/components/base/IconBtn.vue';
import Btn from '@/components/base/Btn.vue';

// Icons
import { trash, add } from 'ionicons/icons';
import WordBank from '@/lib/WordBankClient';
import { Translation, Word } from '@/lib/models';

export default defineComponent({
    name: 'WordUpsert',
    components: { 
        IonPage, 
        IonHeader, 
        IonToolbar, 
        IonTitle, 
        IonContent,
        // IonButton,
        IonButtons,
        IonBackButton,
        IonInput,
        IonItem,
        IonLabel,
        IonIcon,
        IonSelect,
        IonSelectOption,
        IonSpinner,
        // IonFooter,
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
            word: '',
            translations: [] as Translation[],
            wordType: 'NOUN',
            wordTypes: [
                {value: 'NOUN', label: 'Noun'},
                {value: 'PRONOUN', label: 'Pronoun'},
                {value: 'VERB', label: 'Verb'},
                {value: 'ADJECTIVE', label: 'Adjective'},
                {value: 'ADVERB', label: 'Adverb'},
                {value: 'PREPOSITION', label: 'Preposition'},
                {value: 'CONJUNCTION', label: 'Conjunction'},
                {value: 'INTERJECTION', label: 'Interjection'},
            ]
        }
    },
    computed: {
        isFormValid(): boolean {
            return this.word.length > 0 && this.wordType !== null && this.translations.filter(t => t.value).length > 0;
        },
    },
    methods: {
        addTranslation(): void {
            this.translations.push(new Translation(uuidv4(), ''));
        },
        deleteTranslation(id: string): void {
            const translations = [...this.translations];
            const index = translations.findIndex(t => t.id === id);
            if(index === -1) {
                return;
            }
            this.translations.splice(index, 1);
        },
        insertWord(): void {
            if(!this.isFormValid) {
                return;
            }
            this.isLoading = true;

            const word = Word.fromObject({
                value: this.word,
                kind: this.wordType,
                translations: this.translations.filter(t => t.value).map(t => t.value),
            });

            WordBank.insertWord(word)
            .then(() => {
                this.$router.back();
            })
            .catch(() => {
                this.isLoading = false;
            });
        }
    },
    mounted() {
        this.addTranslation();
    }
})
</script>

<style scoped>

</style>