import { Folder, Word } from '@/lib/models';
import { actions } from './actions';
import { getters } from './getters';
import { mutations } from './mutations';
import { createStore } from 'vuex';


export interface State {
    words: Record<string, Word>;
    wordIdsList: string[];
    folders: Record<string, Folder>;
}

const state: State = {
    words: {},
    wordIdsList: [],
    folders: {},
}

export const store = createStore<State>({
    state, 
    actions,
    getters,
    mutations
});