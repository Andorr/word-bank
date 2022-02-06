import { Folder, FolderQueryOptions, FolderResult, PageResult, PaginationOptions, Quiz, QuizOptions, QuizResult, Word, WordQueryOptions } from "@/lib/models";
import WordBank from "@/lib/WordBankClient";
import { ActionTree } from "vuex";
import { State } from '.';
import { LIST_OPTIONS, MUTATIONS } from "./mutations";

export const enum ACTIONS {
    WORD_QUERY = 'WORD_QUERY',
    WORD_INSERT = 'WORD_INSERT',
    WORD_UPDATE = 'WORD_UPDATE',
    WORD_DELETE = 'WORD_DELETE',

    FOLDER_QUERY = 'FOLDER_QUERY',
    FOLDER_GET = 'FOLDER_GET',
    FOLDER_INSERT = 'FOLDER_INSERT',
    FOLDER_UPDATE = 'FOLDER_UPDATE',
    FOLDER_DELETE = 'FOLDER_DELETE',

    QUIZ_START = 'QUIZ_START',
    QUIZ_INSERT_RESULT = 'QUIZ_INSERT_RESULT',

}

export const actions: ActionTree<State, any> = {
    [ACTIONS.WORD_INSERT](store, payload: { word: Word; folderId?: string}): Promise<Word> {
        return WordBank.insertWord(payload.word, payload.folderId).then((w: Word) => {
            store.commit(MUTATIONS.WORD_SET, {
                word: w,
                listOptions: LIST_OPTIONS.FIRST,
            });
            
            // Add word to folder
            if(payload.folderId) {
                const folder: Folder | null = store.getters.getFolderById(payload.folderId);
                if(folder) {
                    folder.words.push(w.id);
                    store.commit(MUTATIONS.FOLDER_SET, folder);
                }
            }

            return w;
        })
    },
    [ACTIONS.WORD_UPDATE](store, payload: { word: Word }): Promise<Word> {
        return WordBank.updateWord(payload.word)
            .then((w: Word) => {
                store.commit(MUTATIONS.WORD_SET, {
                    word: w,
                    listOptions: LIST_OPTIONS.NONE,
                });
                return w;
            })
    },
    [ACTIONS.WORD_QUERY](store, options: ActionQueryOptions = {listOptions: LIST_OPTIONS.LAST}): Promise<PageResult<Word>> {
        return WordBank.queryWords(options.queryOptions, options.pagination).then((pageResult: PageResult<Word>) => {
                store.commit(MUTATIONS.WORDS_SET, {
                    words: pageResult.results,
                    listOptions: options.listOptions
                });
                return pageResult;
            });
    },
    [ACTIONS.WORD_DELETE](store, id: string): Promise<void> {
        return WordBank.deleteWord(id).then(() => {
            store.commit(MUTATIONS.WORD_DELETE, { id: id });

            // Delete word from corresponding folders
            Object.entries(store.state.folders).filter(([, folder]) =>
                folder.words.includes(id)
            ).forEach(([, folder]) => {
                const index = folder.words.indexOf(id);
                folder.words.splice(index, 1);
            })
        })
    },


    [ACTIONS.FOLDER_QUERY](store, options: ActionFolderQueryOptions): Promise<PageResult<Folder>> {
        return WordBank.queryFolders(options.queryOptions, options.pagination)
        .then((pageResult: PageResult<Folder>) => {
            store.commit(MUTATIONS.FOLDERS_SET, pageResult.results);
            return pageResult;
        });
    },
    [ACTIONS.FOLDER_GET](store, id: string): Promise<FolderResult> {
        return WordBank.getFolder(id).then((result) => {
            store.commit(MUTATIONS.WORDS_SET, { 
                words: result.content.words,
                listOptions: LIST_OPTIONS.NONE,
            });
            store.commit(MUTATIONS.FOLDERS_SET, [result.data, ...(result.content.folders || [])])
            return result;
        })
    },
    [ACTIONS.FOLDER_INSERT](store, folder: Folder): Promise<Folder> {
        return WordBank.insertFolder(folder).then((f: Folder) => {
            store.commit(MUTATIONS.FOLDER_SET, f);
            return f;
        });
    },
    [ACTIONS.FOLDER_UPDATE](store, folder: Folder): Promise<Folder> {
        return WordBank.updateFolder(folder).then(() => {
            const f = store.getters.getFolderById(folder.id);
            if(f) {
                f.update(folder);
                store.commit(MUTATIONS.FOLDER_SET, f);
            }

            return f;
        })
    },
    [ACTIONS.FOLDER_DELETE](store, id: string): Promise<void> {
        return WordBank.deleteFolder(id).then(() => {
            store.commit(MUTATIONS.FOLDER_DELETE, { id: id });
        })
    },

    [ACTIONS.QUIZ_START](store, options: QuizOptions): Promise<Quiz> {
        return WordBank.startQuiz(options).then((q: Quiz) => {
            q.options.policy = options.policy;
            store.commit(MUTATIONS.QUIZ_SET, q);
            return q;
        })
    },
    [ACTIONS.QUIZ_INSERT_RESULT](store, result: QuizResult): Promise<QuizResult> {
        return WordBank.insertQuizResult(result).then((qr: QuizResult) => {
            return qr;
        });
    }
}

export type ActionQueryOptions = {
    queryOptions?: WordQueryOptions; 
    pagination?: PaginationOptions; 
    listOptions: LIST_OPTIONS;
}

export type ActionFolderQueryOptions = {
    queryOptions?: FolderQueryOptions;
    pagination?: PaginationOptions;
}