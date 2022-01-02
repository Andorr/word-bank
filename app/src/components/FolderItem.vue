<template>
    <ion-item 
        :button='true'
    >
        <ion-thumbnail class="flex items-center justify-center">
            <ion-icon :icon='icons.folder' />
        </ion-thumbnail>
        <div class="w-full my-2">
            <h4 class="mb-0 font-bold">
                {{ folder.name }}
            </h4>
            <p class="text-gray-300 mb-0">
                ({{ folder.words.length }})
            </p>
            <ion-ripple-effect></ion-ripple-effect>
        </div>
    </ion-item>
</template>

<script lang="ts">
import { defineComponent } from 'vue';

// Logic
import { Folder } from '@/lib/models';

// Components
import{
    IonItem,
    IonThumbnail,
    IonIcon,
    IonRippleEffect,
} from '@ionic/vue';

// Icons
import { folder } from 'ionicons/icons';

export default defineComponent({
    name: 'FolderItem',
    components: {
        IonItem,
        IonThumbnail,
        IonIcon,
        IonRippleEffect,
    },
    props: {
        folder: Folder,
    },
    data() {
        return {
            icons: { folder }
        }
    }
})
</script>

<style scoped>
.ripple-parent {
    position: relative;
    overflow: hidden;
}
</style>