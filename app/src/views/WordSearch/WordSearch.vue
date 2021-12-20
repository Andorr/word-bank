<template>
    <page title='Search'>
        <template v-slot:header>
            <ion-toolbar>
                <ion-searchbar
                    v-model='query'
                >

                </ion-searchbar>
            </ion-toolbar>
            <ion-progress-bar v-if='isLoading' type='indeterminate' />
        </template>
        <div v-if='result === null' class="h-full flex items-center justify-center">
            <div class="max-w-xxs">
                <ion-img :src='searchSvg' />
            </div>
        </div>
        <div v-else-if='words.length === 0' class="h-full flex items-center justify-center">
            <div class="max-w-xxs">
                <ion-img :src='voidSvg' />
                <p class="text-center font-bold mt-2">No results</p>
            </div>
        </div>
        <ion-list v-else>
            <word-item
                v-for='word in words'
                :key='word.id'
                :word='word'
                @click='goToWordUpsert(word.id)'
            />
        </ion-list>
    </page>
</template>

<script lang="ts">
import { defineComponent } from 'vue'

// Store and types
import { ACTIONS } from '@/store/actions';
import { LIST_OPTIONS } from '@/store/mutations';
import { PageResult, Word } from '@/lib/models';

// Components
import {
    IonToolbar,
    IonSearchbar,
    IonList,
    IonProgressBar,
    IonImg,
} from '@ionic/vue'
import Page from '@/components/layout/Page.vue';
import WordItem from '@/components/WordItem.vue';
import URLS from '@/URLS';

// Assets
import SearchSvg from '@/assets/img/search.svg';
import VoidSvg from '@/assets/img/void.svg';

export default defineComponent({
    name: 'WordSearch',
    components: {
        Page,
        IonToolbar,
        IonSearchbar,
        IonList,
        WordItem,
        IonProgressBar,
        IonImg,
    },
    data() {
        return {
            searchSvg: SearchSvg, 
            voidSvg: VoidSvg,

            query: '',
            result: null as PageResult | null,
            isLoading: false,
        }
    },
    computed: {
        words(): Word[] {
            return this.result ? this.result.results : [];
        }
    },
    methods: {
        search() {
            this.isLoading = true;
            this.$store.dispatch(ACTIONS.WORD_QUERY, {
                queryOptions: {
                    query: this.query,
                },
                listOptions: LIST_OPTIONS.NONE,
            })
            .then((result: PageResult) => {
                this.result = result;
            })
            .finally(() => {
                this.isLoading = false;
            })
        },
        goToWordUpsert(id: string) {
            this.$router.push(
                URLS.tabs.concat(URLS.words, URLS.wordsUpsert, `?id=${id}`)
            )
        },
    },
    watch: {
        query() {
            if(this.query) {
                this.search();
            } else {
                this.result = null;
            }
        }
    }
})
</script>

<style scoped>

</style>