<template>
    <ion-item 
        :key='word.id'
        class="ion-activatable ripple-parent"
    >
        <div class="w-full my-2">
            <h4 class="mb-0 font-bold">
                {{ word.value }}
            </h4>
            <p class="text-gray-300 mb-0">
                {{ word.translations.map(t => t.value).join(', ') }}
            </p>
            <ion-ripple-effect></ion-ripple-effect>
        </div>
    </ion-item>
</template>

<script lang="ts">
import { defineComponent } from 'vue';

// Logic
import { Word } from '@/lib/models';

// Components
import{
    IonItem,
    IonRippleEffect,
} from '@ionic/vue';

export default defineComponent({
    name: 'WordItem',
    components: {
        IonItem,
        IonRippleEffect,
    },
    props: {
        word: Word,
    }
})
</script>

<style scoped>
.ripple-parent {
    position: relative;
    overflow: hidden;
}
</style>