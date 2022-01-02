<template>
    <page :title='(id ? "Save" : "Create") + " Folder"'>
        <template v-slot:header-right>
            <icon-btn v-if='id' slot='end' @click="deleteFolder">
                <ion-icon :icon='icons.trash' />
            </icon-btn>
            <ion-progress-bar v-if='isLoading' type='indeterminate' />
        </template>
        
        <div class="flex flex-col min-h-full">
            <ion-item>
                <ion-label
                    position="stacked"
                >
                    Name
                </ion-label>
                <ion-input placeholder="Name" v-model='name' ></ion-input>
            </ion-item>

            <div class="flex-grow" />
            <div class="mx-4 mb-4">
                <btn class="w-full shadow-lg" :disabled='!isFormValid || isLoading' @click='upsertFolder'>
                    <ion-spinner v-if='isLoading' name="dots" />
                    <span v-else>
                        {{ id ? 'Save' : 'Create' }}
                    </span>
                </btn>
            </div>
        </div>

    </page>
</template>

<script lang="ts">
import { defineComponent } from 'vue'

// Store and types
import { ACTIONS } from '@/store/actions';
import { Folder } from '@/lib/models';

// Components
import {
    IonInput,
    IonIcon,
    IonLabel,
    IonItem,
    IonProgressBar,
    IonSpinner,
    alertController,
} from '@ionic/vue'
import Page from '@/components/layout/Page.vue';
import IconBtn from '@/components/base/IconBtn.vue';
import Btn from '@/components/base/Btn.vue';

// Icons
import { trash } from 'ionicons/icons';

export default defineComponent({
    name: 'FolderUpsert',
    components: {
        Page,
        IonIcon,
        IonProgressBar,
        IonInput,
        IonLabel,
        IonItem,
        IonSpinner,
        IconBtn,
        Btn,
    },
    data() {
        return {
            icons: {
                trash,
            },

            isLoading: false,
            name: '',
            id: null as string | null,
            parent: '' as string,
        }
    },
    computed: {
        isFormValid(): boolean {
            return this.name.length >= 2;
        },
    },
    methods: {
        upsertFolder(): void {
            if(!this.isFormValid) {
                return;
            }
            this.isLoading = true;

            let folder = null;
            if(this.id) {
                folder = Folder.fromObject({
                    id: this.id,
                    name: this.name,
                });
            } else {
                folder = Folder.fromObject({
                    name: this.name,
                    parent: this.parent,
                });
            }

            const actions = (this.id) ? ACTIONS.FOLDER_UPDATE : ACTIONS.FOLDER_INSERT;
            this.$store.dispatch(actions, folder)
                .then(() => {
                    this.$router.back();
                })
                .catch(() => {
                    // TODO: Add error
                    this.isLoading = false;
                });
        },
        async deleteFolder() {
            if(!this.id) {
                return
            }
        
            const alert = await alertController.create({
                header: `Delete folder '${this.name}'?`,
                message: `Are you sure you want to delete the folder '${this.name}'?`,
                buttons: ['Cancel', {
                    text: 'Yes',
                    role: 'yes',
                }]
            });
            await alert.present()

            const {role} = await alert.onDidDismiss();
            if(role === 'yes') {
                this.isLoading = true;
                this.$store.dispatch(ACTIONS.FOLDER_DELETE, this.id)
                    .then(() => {
                        this.$router.back();
                    })
                    .catch(() => {
                        // TODO: Add error
                        this.isLoading = false;
                    });
            }

        },
        mountFolder(id: string) {
            const folder: Folder | null = this.$store.getters.getFolderById(id);
            if(!folder) {
                return
            }
            this.id = folder.id;
            this.name = folder.name;
        }
    },
    mounted() {
        if(this.$route.query.id) {
            this.mountFolder(this.$route.query.id as string)
        }
        this.parent = this.$route.query.parent as string;
    }
})
</script>

<style scoped>

</style>