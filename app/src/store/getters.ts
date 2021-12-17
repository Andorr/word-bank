import { Word } from '@/lib/models';
import { GetterTree } from 'vuex'
import { State } from '.'

export const getters: GetterTree<State, any> = {
    getWords(state): Word[] {
        return state.wordIdsList.map(id => state.words[id]).filter(w => w);
    },
    getWordById: (state) => (id: string): Word | null => {
        return state.words[id] || null
    }
}