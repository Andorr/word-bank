import { PageResult, Word } from "@/lib/models";
import WordBank from "@/lib/WordBankClient";
import { ActionTree } from "vuex";
import { State } from '.';
import { LIST_OPTIONS, MUTATIONS } from "./mutations";

export const enum ACTIONS {
    WORD_LIST = 'WORD_LIST',
    WORD_INSERT = 'WORD_INSERT',
    WORD_UPDATE = 'WORD_UPDATE',
    WORD_DELETE = 'WORD_DELETE',
}

export const actions: ActionTree<State, any> = {
    [ACTIONS.WORD_INSERT](store, word: Word): Promise<Word> {
        return WordBank.insertWord(word).then((w: Word) => {
            store.commit(MUTATIONS.WORD_SET, {
                word: w,
                listOptions: LIST_OPTIONS.FIRST,
            });
            return w;
        });
    },
    [ACTIONS.WORD_UPDATE](store, word: Word): Promise<Word> {
        return WordBank.updateWord(word)
            .then((w: Word) => {
                store.commit(MUTATIONS.WORD_SET, {
                    word: w,
                    listOptions: LIST_OPTIONS.NONE,
                });
                return w;
            })
    },
    [ACTIONS.WORD_LIST](store, options: {listOptions: LIST_OPTIONS} = {listOptions: LIST_OPTIONS.LAST}): Promise<PageResult> {
        return WordBank.listWords().then((pageResult: PageResult) => {
                store.commit(MUTATIONS.WORDS_SET, {
                    words: pageResult.results,
                    listOptions: options.listOptions
                });
                return pageResult;
            });
    },
    [ACTIONS.WORD_DELETE](store, id: string): Promise<void> {
        return WordBank.deleteWord(id).then(() => {
            store.commit(MUTATIONS.WORDS_DELETE, { id: id });
        })
        .catch(console.error)
    }
}

