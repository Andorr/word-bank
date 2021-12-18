import { Word } from '@/lib/models';
import { MutationTree } from 'vuex';
import { State } from '.';

export const enum MUTATIONS {
    WORD_SET = 'WORD_SET',
    WORDS_SET = 'WORDS_SET',
    WORDS_DELETE = 'WORDS_DELETE',
}

export const enum LIST_OPTIONS {
    NONE = 'NONE',
    FIRST = 'FIRST',
    LAST = 'LAST',
    OVERWRITE = 'OVERWRITE',
}

export const mutations: MutationTree<State> = {
    [MUTATIONS.WORD_SET](state: State, payload: {word: Word; listOptions: LIST_OPTIONS}) {
        console.log(payload)
        state.words[payload.word.id] = payload.word;
        if(payload.listOptions == LIST_OPTIONS.FIRST) {
            state.wordIdsList.unshift(payload.word.id)
        } else if(payload.listOptions == LIST_OPTIONS.LAST) {
            state.wordIdsList.push(payload.word.id)
        }
    },
    [MUTATIONS.WORDS_SET](state: State, payload: {words: Word[]; listOptions: LIST_OPTIONS}) {
        payload.words.forEach(w => {
            state.words[w.id] = w;
        })
        if(payload.listOptions == LIST_OPTIONS.OVERWRITE) {
            state.wordIdsList = payload.words.map(w => w.id)
        } else if(payload.listOptions == LIST_OPTIONS.LAST) {
            state.wordIdsList.push(...payload.words.map(w => w.id))
        }
    },
    [MUTATIONS.WORDS_DELETE](state: State, payload: { id: string } = { id: '' }) {
        delete state.words[payload.id]
        const index = state.wordIdsList.findIndex(id => id === payload.id)
        if(index !== -1) {
            state.wordIdsList.splice(index, 1)
        }
    }
}