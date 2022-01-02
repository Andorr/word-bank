import { Folder, Word } from '@/lib/models';
import { GetterTree } from 'vuex'
import { State } from '.'

export const getters: GetterTree<State, any> = {
    getWords(state): Word[] {
        return state.wordIdsList.map(id => state.words[id]).filter(w => w);
    },
    getWordById: (state) => (id: string): Word | null => {
        return state.words[id] || null
    },
    getWordsByIds: (state) => (ids: string[]): Word[] => {
        return Object.keys(state.words).filter(id => ids.includes(id)).map(id => state.words[id]);
    },
    getFolderById: (state) => (id: string): Folder | null => {
        return state.folders[id] || null
    },
    getFoldersByIds: (state) => (ids: string[]): Folder[] => {
        return Object.keys(state.folders).filter(id => ids.includes(id)).map(id => state.folders[id]);
    },
    getFoldersByParent: (state) => (parentId: string): Folder[] => {
        return Object.values(state.folders).filter((folder) => folder.parent === parentId)
            .map((folder) => folder);
    }
}