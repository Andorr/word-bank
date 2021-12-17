import { Word } from '@/lib/models';
import { actions } from './actions';
import { getters } from './getters';
import { mutations } from './mutations';
import { createStore } from 'vuex';


export interface State {
    words: Record<string, Word>;
    wordIdsList: string[];
}

const state: State = {
    words: {},
    wordIdsList: [],
}

export const store = createStore<State>({
    state, 
    actions,
    getters,
    mutations
});